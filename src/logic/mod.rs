use crate::data::hierarchy::AppData;
use crate::data::{AppEvent, EventUnSubscribe};
use crate::mqtt::{init_connect, mqtt_public, mqtt_subscribe, to_unsubscribe};
// use crate::ui::tabs::init_brokers_tabs;
use crate::data::common::{Broker, Id, PublicInput, SubscribeHis, SubscribeInput, SubscribeMsg};
use crate::ui::tabs::{ID_ONE, INCREMENT};
use anyhow::Result;
use custom_utils::rx;
use log::{debug, error};
use rumqttc::v5::{
    mqttbytes::{PubAck, SubAck},
    AsyncClient,
};
use std::collections::HashMap;
use std::hash::Hash;
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
            AppEvent::AddBroker => add_broker(&event_sink),
            AppEvent::EditBroker => edit_broker(&event_sink),
            AppEvent::ConnectBroker => connect_broker(&event_sink),
            AppEvent::SaveBroker(index) => save_broker(&event_sink, index),
            AppEvent::RemoveSubscribeHis => remove_subscribe_his(&event_sink),
            AppEvent::ToUnSubscribe { broker_id, pk_id } => {
                to_un_subscribe(&event_sink, broker_id, pk_id)
            }
            AppEvent::UnSubscribeIng(event) => {
                un_subscribe_ing(&event_sink, event, &mqtt_clients).await
            }
            AppEvent::UnSubAck(broke_id, unsubscribe_pk_id) => {
                un_sub_ack(&event_sink, broke_id, unsubscribe_pk_id)
            }
            AppEvent::Connect(broker) => {
                connect(&event_sink, &mut mqtt_clients, tx.clone(), broker).await
            }
            AppEvent::Subscribe(input, index) => {
                subscribe(&event_sink, &mqtt_clients, index, input).await
            }
            AppEvent::SubscribeFromHis(his) => {
                subscribe_from_his(&event_sink, &mqtt_clients, his).await
            }
            AppEvent::Public(input, index) => {
                publish(&event_sink, &mqtt_clients, index, input).await
            }
            AppEvent::ReceivePublic(index, msg) => receive_public(&event_sink, index, msg),
            AppEvent::PubAck(id, ack) => pub_ack(&event_sink, id, ack),
            AppEvent::SubAck(id, ack) => sub_ack(&event_sink, id, ack),
            AppEvent::SelectTabs(id) => select_tabs(&event_sink, id),
            AppEvent::ClickBroker(id) => click_broker(&event_sink, tx.clone(), &mut clicks, id),
            AppEvent::DbClickCheck(id) => db_click_check(&mut clicks, id),
            AppEvent::ClickSubscribeHis(his) => {
                click_subscribe_his(&event_sink, tx.clone(), &mqtt_clients, &mut click_his, his)
                    .await
            }
            AppEvent::DbClickCheckSubscribeHis(his) => {
                db_click_check_subscribe_his(&mut click_his, his).await
            }
            AppEvent::ReConnect(id) => re_connect(&event_sink, &mut mqtt_clients, id).await,
            AppEvent::Disconnect(id) => disconnect(&event_sink, &mut mqtt_clients, id).await,
            AppEvent::CloseBrokerTab(id) => close_broker_tab(&event_sink, id),
            AppEvent::CloseConnectionTab(id) => {
                close_connection_tab(&event_sink, &mut mqtt_clients, id).await
            }
            AppEvent::DeleteBroker => delete_broker(&event_sink),
            AppEvent::ConnectAckSuccess(id) => connect_ack_success(&event_sink, id), // _ => {}
            AppEvent::ConnectAckFail(_id, _msg) => todo!(),
        }
    }
}

fn add_broker(event_sink: &druid::ExtEventSink) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        data.add_broker();
    });
}
fn edit_broker(event_sink: &druid::ExtEventSink) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        data.edit_broker();
    });
}

fn connect_broker(event_sink: &druid::ExtEventSink) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        data.connect_broker();
    });
}

fn save_broker(event_sink: &druid::ExtEventSink, index: usize) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.save_broker(index) {
            error!("{:?}", e);
        }
    });
}

fn remove_subscribe_his(event_sink: &druid::ExtEventSink) {
    event_sink.add_idle_callback(move |data: &mut AppData| data.remove_subscribe_his());
}

fn to_un_subscribe(event_sink: &druid::ExtEventSink, broker_id: usize, pk_id: u16) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.to_unscribe(broker_id, pk_id) {
            error!("{:?}", e);
        }
    });
}

async fn un_subscribe_ing(
    event_sink: &druid::ExtEventSink,
    event: EventUnSubscribe,
    mqtt_clients: &HashMap<usize, AsyncClient>,
) {
    let EventUnSubscribe {
        broke_id,
        subscribe_pk_id,
        topic,
    } = event;
    match to_unsubscribe(broke_id, topic, &mqtt_clients).await {
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
    }
}

fn un_sub_ack(event_sink: &druid::ExtEventSink, broke_id: usize, unsubscribe_pk_id: u16) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.unsubscribe_ack(broke_id, unsubscribe_pk_id) {
            error!("{:?}", e);
        }
    });
}

async fn connect(
    event_sink: &druid::ExtEventSink,
    mqtt_clients: &mut HashMap<usize, AsyncClient>,
    tx: Sender<AppEvent>,
    broker: Broker,
) {
    match init_connect(broker.clone(), tx.clone()).await {
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
    }
}

async fn subscribe(
    event_sink: &druid::ExtEventSink,
    mqtt_clients: &HashMap<usize, AsyncClient>,
    index: usize,
    input: SubscribeInput,
) {
    match mqtt_subscribe(index, input.clone().into(), &mqtt_clients).await {
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

async fn subscribe_from_his(
    event_sink: &druid::ExtEventSink,
    mqtt_clients: &HashMap<usize, AsyncClient>,
    input: SubscribeHis,
) {
    match mqtt_subscribe(input.broker_id, input.clone().into(), &mqtt_clients).await {
        Ok(id) => {
            event_sink.add_idle_callback(move |data: &mut AppData| {
                if let Err(e) = data.subscribe(input.broker_id, input, id) {
                    error!("{:?}", e);
                }
            });
        }
        Err(e) => {
            error!("{:?}", e);
        }
    }
}

async fn publish(
    event_sink: &druid::ExtEventSink,
    mqtt_clients: &HashMap<usize, AsyncClient>,
    index: usize,
    input: PublicInput,
) {
    debug!("{:?}", input);
    match mqtt_public(index, input.clone().into(), &mqtt_clients).await {
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

fn receive_public(event_sink: &druid::ExtEventSink, index: usize, msg: SubscribeMsg) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        data.receive_msg(index, msg);
    });
}

fn pub_ack(event_sink: &druid::ExtEventSink, id: usize, ack: PubAck) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        data.puback(id, ack);
    });
}

fn sub_ack(event_sink: &druid::ExtEventSink, id: usize, ack: SubAck) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        data.suback(id, ack);
    });
}
fn select_tabs(event_sink: &druid::ExtEventSink, id: usize) {
    if let Err(e) = event_sink.submit_command(INCREMENT, id, ID_ONE) {
        error!("{:?}", e);
    }
}
fn click_broker(
    event_sink: &druid::ExtEventSink,
    tx: Sender<AppEvent>,
    clicks: &mut HashMap<usize, usize>,
    id: usize,
) {
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

fn db_click_check(clicks: &mut HashMap<usize, usize>, id: usize) {
    if let Some(_previous) = clicks.remove(&id) {}
}

async fn click_subscribe_his(
    event_sink: &druid::ExtEventSink,
    tx: Sender<AppEvent>,
    mqtt_clients: &HashMap<usize, AsyncClient>,
    click_his: &mut Option<SubscribeHis>,
    his: SubscribeHis,
) {
    let index = his.broker_id;
    if let Some(_previous) = click_his.take() {
        if _previous == his {
            // double
            if let Some(client) = mqtt_clients.get(&index) {
                let Ok(pkid) = client.subscribe_and_tracing(his.topic.as_str(), his.qos.into()).await else {
                    error!("!!!!!!");
                    return;
                };
                event_sink.add_idle_callback(move |data: &mut AppData| {
                    if let Err(e) = data.subscribe(index, _previous, pkid) {
                        error!("{:?}", e);
                    }
                });
            }
            return;
        }
    }
    *click_his = Some(his.clone());
    let data_his = his.clone();
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.click_subscribe_his(data_his) {
            error!("{:?}", e);
        }
    });
    let async_tx = tx.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(280)).await;
        if let Err(e) = async_tx.send(AppEvent::DbClickCheckSubscribeHis(his)) {
            error!("{:?}", e);
        }
    });
}

async fn db_click_check_subscribe_his(click_his: &mut Option<SubscribeHis>, his: SubscribeHis) {
    if click_his.as_ref().map_or(false, |x| *x == his) {
        click_his.take();
    }
}

async fn re_connect(
    event_sink: &druid::ExtEventSink,
    mqtt_clients: &mut HashMap<usize, AsyncClient>,
    id: usize,
) {
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

async fn disconnect(
    event_sink: &druid::ExtEventSink,
    mqtt_clients: &mut HashMap<usize, AsyncClient>,
    id: usize,
) {
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

fn close_broker_tab(event_sink: &druid::ExtEventSink, id: usize) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.close_tab(id) {
            error!("{:?}", e);
        }
    });
}
async fn close_connection_tab(
    event_sink: &druid::ExtEventSink,
    mqtt_clients: &mut HashMap<usize, AsyncClient>,
    id: usize,
) {
    if let Some(client) = mqtt_clients.remove(&id) {
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

fn delete_broker(event_sink: &druid::ExtEventSink) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.delete_broker() {
            error!("{:?}", e);
        }
    });
}
fn connect_ack_success(event_sink: &druid::ExtEventSink, id: usize) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.connected(id) {
            error!("{:?}", e);
        }
    });
}
