use crate::data::hierarchy::AppData;
use crate::data::{AppEvent, EventUnSubscribe};
use crate::mqtt::{init_connect, public, subscribe, to_unsubscribe};
// use crate::ui::tabs::init_brokers_tabs;
use crate::data::common::SubscribeHis;
use crate::ui::tabs::{ID_ONE, INCREMENT};
use anyhow::Result;
use custom_utils::rx;
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
) -> Result<()> {
    let mut mqtt_clients: HashMap<usize, AsyncClient> = HashMap::new();
    let mut clicks: HashMap<usize, usize> = HashMap::new();
    let mut click_his: Option<SubscribeHis> = None;
    loop {
        // let event = ;
        // debug!("{:?}", event);
        match rx!(rx) {
            AppEvent::AddBroker => {
                event_sink.add_idle_callback(move |data: &mut AppData| {
                    data.add_broker();
                });
            }
            AppEvent::EditBroker => {
                event_sink.add_idle_callback(move |data: &mut AppData| {
                    data.edit_broker();
                });
            }
            AppEvent::ConnectBroker => {
                event_sink.add_idle_callback(move |data: &mut AppData| {
                    data.connect_broker();
                });
            }
            AppEvent::SaveBroker(index) => {
                event_sink.add_idle_callback(move |data: &mut AppData| {
                    if let Err(e) = data.save_broker(index) {
                        error!("{:?}", e);
                    }
                });
            }
            AppEvent::RemoveSubscribeHis { broker_id, his_id } => {
                event_sink.add_idle_callback(move |data: &mut AppData| {
                    data.remove_subscribe_his(broker_id, his_id);
                });
            }
            AppEvent::ToUnSubscribe { broker_id, pk_id } => {
                event_sink.add_idle_callback(move |data: &mut AppData| {
                    if let Err(e) = data.to_unscribe(broker_id, pk_id) {
                        error!("{:?}", e);
                    }
                });
            }
            AppEvent::UnSubscribeIng(EventUnSubscribe {
                broke_id,
                subscribe_pk_id,
                topic,
            }) => match to_unsubscribe(broke_id, topic, &mqtt_clients).await {
                Ok(pk_id) => {
                    event_sink.add_idle_callback(move |data: &mut AppData| {
                        if let Err(e) = data.unscribeing(broke_id, subscribe_pk_id, pk_id) {
                            error!("{:?}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("{:?}", e);
                }
            },
            AppEvent::UnSubAck(broke_id, unsubscribe_pk_id) => {
                event_sink.add_idle_callback(move |data: &mut AppData| {
                    if let Err(e) = data.unsubscribe_ack(broke_id, unsubscribe_pk_id) {
                        error!("{:?}", e);
                    }
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
                            if let Err(e) = data.subscribe_by_input(index, input, id) {
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
                debug!("{:?}", input);
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
            AppEvent::SelectTabs(id) => {
                if let Err(e) = event_sink.submit_command(INCREMENT, id, ID_ONE) {
                    error!("{:?}", e);
                }
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
                        if let Err(e) = data.click_broker(id) {
                            error!("{:?}", e);
                        }
                    });
                    tokio::spawn(async move {
                        tokio::time::sleep(Duration::from_millis(280)).await;
                        if let Err(e) = async_tx.send(AppEvent::DbClickCheck(id)) {
                            error!("{:?}", e);
                        }
                    });
                }
            }
            AppEvent::DbClickCheck(id) => if let Some(_previous) = clicks.remove(&id) {},
            AppEvent::ClickSubscribeHis(his) => {
                let index = his.broker_id;
                if let Some(_previous) = click_his.take() {
                    if _previous == his {
                        // double
                        if let Some(client) = mqtt_clients.get(&index) {
                            let Ok(pkid) = client.subscribe_and_tracing(his.topic.as_str(), his.qos.into()).await else {
                                error!("!!!!!!");
                                continue;
                            };
                            event_sink.add_idle_callback(move |data: &mut AppData| {
                                if let Err(e) = data.subscribe(index, _previous, pkid) {
                                    error!("{:?}", e);
                                }
                            });
                        }
                        continue;
                    }
                }
                click_his = Some(his.clone());
                let async_tx = tx.clone();
                tokio::spawn(async move {
                    tokio::time::sleep(Duration::from_millis(280)).await;
                    if let Err(e) = async_tx.send(AppEvent::DbClickCheckSubscribeHis(his)) {
                        error!("{:?}", e);
                    }
                });
            }
            AppEvent::DbClickCheckSubscribeHis(his) => {
                if click_his.as_ref().map_or(false, |x| *x == his) {
                    click_his.take();
                }
            }
            AppEvent::ReConnect(id) => {
                if let Some(client) = mqtt_clients.remove(&id) {
                    if let Err(e) = client.disconnect().await {
                        error!("{:?}", e);
                    }
                }
                event_sink.add_idle_callback(move |data: &mut AppData| {
                    if let Err(e) = data.reconnect(id) {
                        error!("{:?}", e);
                    }
                });
            }
            AppEvent::Disconnect(id) => {
                if let Some(client) = mqtt_clients.remove(&id) {
                    if let Err(e) = client.disconnect().await {
                        error!("{:?}", e);
                    }
                }
                event_sink.add_idle_callback(move |data: &mut AppData| {
                    if let Err(e) = data.disconnect(id) {
                        error!("{:?}", e);
                    }
                });
            }
            AppEvent::CloseBrokerTab(id) => {
                event_sink.add_idle_callback(move |data: &mut AppData| {
                    if let Err(e) = data.close_tab(id) {
                        error!("{:?}", e);
                    }
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
