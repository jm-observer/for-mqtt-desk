pub mod data;

use crate::data::common::Broker;
use crate::data::common::SubscribeMsg;
use crate::data::AppEvent;
use crate::mqtt::data::{MqttPublicInput, MqttSubscribeInput};
use anyhow::{bail, Result};
use druid::piet::TextStorage;
use log::{debug, error};
use rumqttc::v5::mqttbytes::v5::Packet;
use rumqttc::v5::{
    mqttbytes::{ConnectReturnCode, Publish},
    AsyncClient, Event, MqttOptions,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::time::Duration;

pub async fn init_connect(broker: Broker, tx: Sender<AppEvent>) -> Result<AsyncClient> {
    let mut mqttoptions =
        MqttOptions::new(broker.client_id.as_str(), broker.addr.as_str(), broker.port);
    if broker.use_credentials {
        mqttoptions.set_credentials(&*broker.user_name, &*broker.password);
    }
    let some = serde_json::from_str(broker.params.as_str())?;
    update_option(&mut mqttoptions, some);

    debug!("{:?}", mqttoptions);
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    let _client_tmp = client.clone();
    let id = broker.id;
    debug!("start");
    tokio::spawn(async move {
        debug!("start");
        while let Ok(event) = eventloop.poll().await {
            let event = match event {
                Event::Incoming(event) => event,
                _ => continue,
            };
            let tx = tx.clone();
            debug!("{:?}", event);
            match *event {
                Packet::ConnAck(ack) => {
                    deal_conn_ack(ack.code, tx, id);
                }
                Packet::PubAck(ack, _) => {
                    if let Err(_) = tx.send(AppEvent::PubAck(id, ack)) {
                        error!("fail to send event!");
                    };
                }
                Packet::SubAck(ack, _) => {
                    if let Err(_) = tx.send(AppEvent::SubAck(id, ack)) {
                        error!("fail to send event!");
                    };
                }
                Packet::UnsubAck(ack) => {
                    if let Err(_) = tx.send(AppEvent::UnSubAck(id, ack.pkid)) {
                        error!("fail to send event!");
                    };
                }
                Packet::Publish(msg, _) => {
                    let Publish {
                        dup: _,
                        qos,
                        retain: _,
                        topic,
                        pkid,
                        payload,
                    } = msg;
                    if let Err(_) = tx.send(AppEvent::ReceivePublic(
                        id,
                        SubscribeMsg {
                            pkid,
                            topic: String::from_utf8_lossy(topic.as_ref()).to_string().into(),
                            msg: String::from_utf8_lossy(payload.as_ref()).to_string().into(),
                            qos: qos.into(),
                        },
                    )) {
                        error!("fail to send event!");
                    };
                }
                _ => {}
            }
        }
        debug!("end");
    });
    Ok(client)
}

fn deal_conn_ack(ack_code: ConnectReturnCode, tx: Sender<AppEvent>, id: usize) {
    match ack_code {
        ConnectReturnCode::Success => {
            debug!("connect success!");
            if let Err(_) = tx.send(AppEvent::ConnectAckSuccess(id)) {
                error!("fail to send event!");
            }
        }
        error => {
            if let Err(_) = tx.send(AppEvent::ConnectAckFail(id, format!("{:?}", error).into())) {
                error!("fail to send event!");
            }
        }
    }
}

pub async fn mqtt_subscribe(
    index: usize,
    input: MqttSubscribeInput,
    clients: &HashMap<usize, AsyncClient>,
) -> Result<u16> {
    let Some(client) = clients.get(&index) else {
        bail!("can't get mqtt client: {}", index);
    };
    Ok(client.subscribe_and_tracing(input.topic, input.qos).await?)
}

pub async fn to_unsubscribe(
    index: usize,
    topic: String,
    clients: &HashMap<usize, AsyncClient>,
) -> Result<u16> {
    let Some(client) = clients.get(&index) else {
        bail!("can't get mqtt client: {}", index);
    };
    Ok(client.unsubscribe_and_tracing(topic).await?)
}

pub async fn mqtt_public(
    index: usize,
    input: MqttPublicInput,
    clients: &HashMap<usize, AsyncClient>,
) -> Result<u16> {
    let Some(client) = clients.get(&index) else {
        bail!("can't get mqtt client: {}", index);
    };
    Ok(client
        .publish_and_tracing(input.topic, input.qos, input.retain, input.msg)
        .await?)
}

fn update_option(option: &mut MqttOptions, some: SomeMqttOption) {
    let SomeMqttOption {
        keep_alive,
        clean_session,
        max_incoming_packet_size,
        max_outgoing_packet_size,
        inflight,
        conn_timeout,
    } = some;
    option
        .set_keep_alive(Duration::from_secs(keep_alive))
        .set_clean_session(clean_session)
        .set_max_packet_size(max_incoming_packet_size, max_outgoing_packet_size)
        .set_inflight(inflight)
        .set_connection_timeout(conn_timeout);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SomeMqttOption {
    // seconds
    keep_alive: u64,
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
