use crate::data::db::Broker;
use crate::data::hierarchy::AppData;
use crate::data::AppEvent;
use crate::mqtt::init_connect;
use log::{debug, error};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
pub async fn deal_event(
    event_sink: druid::ExtEventSink,
    rx: Receiver<AppEvent>,
    tx: Sender<AppEvent>,
) {
    loop {
        let event = match rx.recv() {
            Ok(event) => event,
            Err(_) => {
                error!("RecvError");
                break
            }
        };
        debug!("{:?}", event);
        match event {
            AppEvent::Connect(broker) => {
                let tmp_broker = broker.clone();
                let tmp_tx = tx.clone();
                    match init_connect(tmp_broker, tmp_tx).await {
                    Ok(client) => {
                        let id = broker.id;
                        event_sink.add_idle_callback(move |data: &mut AppData| {
                            data.init_connection(id);
                            data.mqtt_clients.insert(id, client);
                        });
                    }
                    Err(e) => {
                        error!("{:?}", e);
                    }
                }
            },
            _ => {}
        }
    }
}

fn update_app_data_connection(data: &mut AppData, broker: Arc<Broker>) {}
