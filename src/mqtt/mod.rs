use crate::data::db::Broker;
use crate::data::AppEvent;
use anyhow::Result;
use druid::piet::TextStorage;
use rumqttc::v5::{AsyncClient, MqttOptions};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::time::Duration;
use tokio::task;

pub async fn init(broker: Arc<Broker>, tx: Sender<AppEvent>) -> Result<AsyncClient> {
    let mut mqttoptions = MqttOptions::new(
        broker.client_id.as_str(),
        broker.addr.as_str(),
        broker.port.parse()?,
    );
    mqttoptions.set_keep_alive(Duration::from_secs(20));

    let (client, mut notifier) = AsyncClient::connect(mqttoptions, 10).await;
    let _client_tmp = client.clone();
    task::spawn(async move {
        for event in notifier.iter() {
            println!("{:?}", event);
        }
    });
    Ok(client)
}
