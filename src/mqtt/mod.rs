pub mod data;

use crate::data::common::{Broker, Protocol};
use crate::data::common::{SignedTy, SubscribeMsg};
use crate::data::{AString, AppEvent};
use crate::mqtt::data::{MqttPublicInput, MqttSubscribeInput};
use crate::util::consts::QosToString;
use anyhow::{bail, Result};
use crossbeam_channel::Sender;
use druid::piet::TextStorage;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use for_mqtt_client::protocol::packet::Publish;
use for_mqtt_client::protocol::MqttOptions;
use for_mqtt_client::tls::TlsConfig;
use for_mqtt_client::MqttEvent;
pub use for_mqtt_client::{
    protocol::packet::{PubAck, SubAck},
    Client, QoS, QoSWithPacketId,
};

pub async fn init_connect(broker: Broker, tx: Sender<AppEvent>) -> Result<Client> {
    let mut mqttoptions =
        MqttOptions::new(broker.client_id.clone(), broker.addr.as_str(), broker.port);
    if broker.use_credentials {
        mqttoptions.set_credentials(broker.user_name.clone(), broker.password.clone());
    }
    let some = serde_json::from_str(broker.params.as_str())?;
    let mqttoptions = update_tls_option(update_option(mqttoptions, some), broker.clone());

    debug!("{:?}", mqttoptions);
    let client = match broker.protocol {
        Protocol::V4 => mqttoptions.connect_to_v4().await,
        Protocol::V5 => mqttoptions.connect_to_v5().await,
    };
    let mut eventloop = client.init_receiver();
    let id = broker.id;
    debug!("start");
    tokio::spawn(async move {
        debug!("start");
        while let Ok(event) = eventloop.recv().await {
            let tx = tx.clone();
            debug!("{:?}", event);
            match event {
                MqttEvent::ConnectSuccess(_) => {
                    deal_conn_success(tx, id);
                }
                MqttEvent::ConnectFail(err) => {
                    deal_conn_fail(format!("{:?}", err), tx, id);
                }
                MqttEvent::PublishSuccess(packet_id) => {
                    if let Err(_) = tx.send(AppEvent::PubAck(id, packet_id)) {
                        error!("fail to send event!");
                    };
                }
                MqttEvent::SubscribeAck(packet) => {
                    if let Err(_) = tx.send(AppEvent::SubAck(id, packet)) {
                        error!("fail to send event!");
                    };
                }
                MqttEvent::UnsubscribeAck(packet) => {
                    if let Err(_) = tx.send(AppEvent::UnSubAck(id, packet)) {
                        error!("fail to send event!");
                    };
                }
                MqttEvent::Publish(msg) => {
                    let Publish {
                        dup: _,
                        qos,
                        retain: _,
                        topic,
                        payload,
                        ..
                    } = msg;
                    if let Err(_) = tx.send(AppEvent::ReceivePublic(id, topic, payload, qos.into()))
                    {
                        error!("fail to send event!");
                    };
                }
                MqttEvent::PublishFail(reason) => {
                    error!("{}", reason);
                }
                MqttEvent::SubscribeFail(reason) => {
                    error!("{}", reason);
                }
                MqttEvent::UnsubscribeFail(reason) => {
                    error!("{}", reason);
                }
                MqttEvent::ConnectedErr(reason) => {
                    error!("{}", reason);
                }
                MqttEvent::Disconnected => {
                    info!("Disconnected");
                }
            }
        }
        debug!("end");
    });
    Ok(client)
}

fn deal_conn_success(tx: Sender<AppEvent>, id: usize) {
    debug!("connect success!");
    if let Err(_) = tx.send(AppEvent::ConnectAckSuccess(id)) {
        error!("fail to send event!");
    }
}
fn deal_conn_fail(err: String, tx: Sender<AppEvent>, id: usize) {
    if let Err(_) = tx.send(AppEvent::ConnectAckFail(id, err.into())) {
        error!("fail to send event!");
    }
}

pub async fn mqtt_subscribe(
    index: usize,
    input: MqttSubscribeInput,
    clients: &HashMap<usize, Client>,
) -> Result<u32> {
    let Some(client) = clients.get(&index) else {
        bail!("can't get mqtt client: {}", index);
    };
    Ok(client.to_subscribe(input.topic, input.qos.into()).await?)
}

pub async fn to_unsubscribe(
    index: usize,
    topic: String,
    clients: &HashMap<usize, Client>,
) -> Result<u32> {
    let Some(client) = clients.get(&index) else {
        bail!("can't get mqtt client: {}", index);
    };
    Ok(client.unsubscribe(topic).await?)
}

pub async fn mqtt_public(
    index: usize,
    input: MqttPublicInput,
    clients: &HashMap<usize, Client>,
) -> Result<u32> {
    let Some(client) = clients.get(&index) else {
        bail!("can't get mqtt client: {}", index);
    };
    Ok(client
        .publish(input.topic, input.qos.into(), input.msg, input.retain)
        .await?)
}

fn update_option(mut option: MqttOptions, some: SomeMqttOption) -> MqttOptions {
    let SomeMqttOption {
        keep_alive,
        clean_session,
        max_incoming_packet_size,
        max_outgoing_packet_size,
        inflight,
        conn_timeout,
    } = some;
    // .set_inflight(inflight)
    // .set_connection_timeout(conn_timeout)
    option
        .set_clean_session(clean_session)
        .set_max_packet_size(max_incoming_packet_size, max_outgoing_packet_size);
    option.set_keep_alive(keep_alive)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SomeMqttOption {
    // seconds
    keep_alive: u16,
    clean_session: bool,
    max_incoming_packet_size: usize,
    max_outgoing_packet_size: usize,
    inflight: u16,
    // seconds
    conn_timeout: u64,
}

impl Default for SomeMqttOption {
    fn default() -> Self {
        Self {
            keep_alive: 60,
            clean_session: true,
            max_incoming_packet_size: 10 * 1024,
            max_outgoing_packet_size: 10 * 1024,
            inflight: 100,
            conn_timeout: 5,
        }
    }
}

fn update_tls_option(option: MqttOptions, value: Broker) -> MqttOptions {
    if value.tls {
        let tls_config = match value.signed_ty {
            SignedTy::Ca => TlsConfig::default(),
            SignedTy::SelfSigned => {
                TlsConfig::default().set_server_ca_pem_file(value.self_signed_ca.as_str().into())
            }
        };
        option.set_tls(tls_config)
    } else {
        option
    }
}

#[cfg(test)]
mod test {
    use crate::mqtt::SomeMqttOption;

    #[test]
    fn test_option() {
        let option = SomeMqttOption::default();
        println!("{}", serde_json::to_string(&option).unwrap());

        let option_str = r#"{
	"keep_alive": 60,
	"clean_session": true,
	"max_incoming_packet_size": 10240,
	"max_outgoing_packet_size": 10240,
	"inflight": 100,
	"conn_timeout": 5
}
        "#;
        let option: SomeMqttOption = serde_json::from_str(option_str).unwrap();
        println!("{:?}", option);
    }
}
