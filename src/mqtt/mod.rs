pub mod data;

use crate::data::common::Broker;
use crate::data::common::SubscribeMsg;
use crate::data::AppEvent;
use crate::mqtt::data::{MqttPublicInput, MqttSubscribeInput};
use anyhow::{bail, Result};
use crossbeam_channel::Sender;
use druid::piet::TextStorage;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use for_mqtt_client::v3_1_1::{MqttOptions, Publish};
use for_mqtt_client::MqttEvent;
pub use for_mqtt_client::{
    v3_1_1::{PubAck, SubAck},
    Client, QoS, QoSWithPacketId,
};

pub async fn init_connect(broker: Broker, tx: Sender<AppEvent>) -> Result<Client> {
    let mut mqttoptions =
        MqttOptions::new(broker.client_id.clone(), broker.addr.as_str(), broker.port);
    if broker.use_credentials {
        mqttoptions.set_credentials(broker.user_name.clone(), broker.password.clone());
    }
    let some = serde_json::from_str(broker.params.as_str())?;
    update_option(&mut mqttoptions, some);

    debug!("{:?}", mqttoptions);
    let (client, mut eventloop) = mqttoptions.run().await;
    let id = broker.id;
    debug!("start");
    tokio::spawn(async move {
        debug!("start");
        while let Ok(event) = eventloop.recv().await {
            let tx = tx.clone();
            debug!("{:?}", event);
            match event {
                MqttEvent::ConnectSuccess => {
                    deal_conn_success(tx, id);
                }
                MqttEvent::ConnectFail(err) => {
                    deal_conn_fail(err, tx, id);
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
                    } = msg;
                    if let Err(_) = tx.send(AppEvent::ReceivePublic(
                        id,
                        SubscribeMsg {
                            topic: topic.clone(),
                            msg: String::from_utf8_lossy(payload.as_bytes())
                                .to_string()
                                .into(),
                            qos: qos.into(),
                        },
                    )) {
                        error!("fail to send event!");
                    };
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
    Ok(client.subscribe(input.topic, input.qos.into()).await.id)
}

pub async fn to_unsubscribe(
    index: usize,
    topic: String,
    clients: &HashMap<usize, Client>,
) -> Result<u32> {
    let Some(client) = clients.get(&index) else {
        bail!("can't get mqtt client: {}", index);
    };
    Ok(client.unsubscribe(topic).await.id)
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
        .await?
        .id())
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
        .set_keep_alive(keep_alive)
        .set_clean_session(clean_session)
        .set_max_packet_size(max_incoming_packet_size, max_outgoing_packet_size)
        .set_inflight(inflight)
        .set_connection_timeout(conn_timeout);
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
