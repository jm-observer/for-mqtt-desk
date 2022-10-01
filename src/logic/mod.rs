use crate::data::hierarchy::AppData;
use crate::data::AppEvent;
use crate::mqtt::{init_connect, public, subscribe};
// use crate::ui::tabs::init_brokers_tabs;
use log::{debug, error};
use rumqttc::v5::AsyncClient;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
pub async fn deal_event(
    event_sink: druid::ExtEventSink,
    rx: Receiver<AppEvent>,
    tx: Sender<AppEvent>,
) {
    let mut mqtt_clients: HashMap<usize, AsyncClient> = HashMap::new();
    let mut clicks: HashMap<usize, usize> = HashMap::new();
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
            AppEvent::AddBroker => {
                event_sink.add_idle_callback(move |data: &mut AppData| {
                    data.add_broker();
                });
            }
            AppEvent::Connect(broker) => match init_connect(broker.clone(), tx.clone()).await {
                Ok(client) => {
                    let id = broker.id;
                    mqtt_clients.insert(id, client.clone());
                    event_sink.add_idle_callback(move |data: &mut AppData| {
                        if let Err(e) = data.init_connection(id) {
                            error!("{:?}", e);
                        }
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
                            if let Err(e) = data.subscribe(index, input, id) {
                                error!("{:?}", e);
                            }
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
            AppEvent::PubAck(id, ack) => {
                event_sink.add_idle_callback(move |data: &mut AppData| {
                    data.puback(id, ack);
                });
            }
            AppEvent::SubAck(id, ack) => {
                event_sink.add_idle_callback(move |data: &mut AppData| {
                    data.suback(id, ack);
                });
            }
            AppEvent::ClickBroker(id) => {
                if let Some(_previous) = clicks.remove(&id) {
                    event_sink.add_idle_callback(move |data: &mut AppData| {
                        data.db_click_broker(id);
                    });
                } else {
                    clicks.insert(id, id);
                    let async_tx = tx.clone();
                    event_sink.add_idle_callback(move |data: &mut AppData| {
                        data.click_broker(id);
                    });
                    tokio::spawn(async move {
                        tokio::time::sleep(Duration::from_millis(280)).await;
                        if let Err(e) = async_tx.send(AppEvent::DbClickCheck(id)) {
                            error!("{:?}", e);
                        }
                    });
                }
            }
            AppEvent::DbClickCheck(id) => {
                if let Some(_previous) = clicks.remove(&id) {
                    // event_sink.add_idle_callback(move |data: &mut AppData| {
                    //     data.click_broker(id);
                    // });
                }
            }
            AppEvent::Disconnect(id) => {
                if let Some(client) = mqtt_clients.remove(&id) {
                    if let Err(e) = client.disconnect().await {
                        error!("{:?}", e);
                    }
                }
            }
            AppEvent::CloseBrokerTab(id) => {
                event_sink.add_idle_callback(move |data: &mut AppData| {
                    if let Err(e) = data.close_tab(id) {
                        error!("{:?}", e);
                    }
                    // let root = init_brokers_tabs();
                    // println!("{:?}", root.debug_state(&data));
                });
            }
            AppEvent::CloseConnectionTab(id) => {
                if let Some(client) = mqtt_clients.get(&id) {
                    if let Err(e) = client.disconnect().await {
                        error!("{:?}", e);
                    }
                } else {
                    error!("can't find client");
                }
                event_sink.add_idle_callback(move |data: &mut AppData| {
                    data.close_connection(id);
                });
            }
            AppEvent::DeleteBroker => {
                event_sink.add_idle_callback(move |data: &mut AppData| {
                    if let Err(e) = data.delete_broker() {
                        error!("{:?}", e);
                    }
                });
            }
            AppEvent::ConnectAckSuccess(id) => {
                event_sink.add_idle_callback(move |data: &mut AppData| {
                    if let Err(e) = data.connected(id) {
                        error!("{:?}", e);
                    }
                });
            }
            _ => {}
        }
    }
}

// fn update_app_data_connection(data: &mut AppData, broker: Arc<Broker>) {}
