use crate::data::hierarchy::AppData;
use crate::data::AppEvent;
use crate::mqtt::{init_connect, public, subscribe};
use log::{debug, error};
use rumqttc::v5::AsyncClient;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
pub async fn deal_event(
    event_sink: druid::ExtEventSink,
    rx: Receiver<AppEvent>,
    tx: Sender<AppEvent>,
) {
    let mut mqtt_clients: HashMap<usize, AsyncClient> = HashMap::new();
    loop {
        let event = match rx.recv() {
            Ok(event) => event,
            Err(_) => {
                error!("RecvError");
                break;
            }
        };
        debug!("{:?}", event);
        match event {
            AppEvent::Connect(broker) => match init_connect(broker.clone(), tx.clone()).await {
                Ok(client) => {
                    let id = broker.id;
                    mqtt_clients.insert(id, client.clone());
                    event_sink.add_idle_callback(move |data: &mut AppData| {
                        data.init_connection(id);
                    });
                }
                Err(e) => {
                    error!("{:?}", e);
                }
            },
            AppEvent::Subscribe(input, index) => {
                match subscribe(index, input.clone().into(), &mqtt_clients).await {
                    Ok(id) => {
                        event_sink.add_idle_callback(move |data: &mut AppData| {
                            data.subscribe(index, input, id);
                        });
                    }
                    Err(e) => {
                        error!("{:?}", e);
                    }
                }
            }
            AppEvent::Public(input, index) => {
                match public(index, input.clone().into(), &mqtt_clients).await {
                    Ok(id) => {
                        event_sink.add_idle_callback(move |data: &mut AppData| {
                            data.public(index, input, id);
                        });
                    }
                    Err(e) => {
                        error!("{:?}", e);
                    }
                }
            }
            AppEvent::ReceivePublic(index, msg) => {
                event_sink.add_idle_callback(move |data: &mut AppData| {
                    data.receive_msg(index, msg);
                });
            }
            _ => {}
        }
    }
}

// fn update_app_data_connection(data: &mut AppData, broker: Arc<Broker>) {}
