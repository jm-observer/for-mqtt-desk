use crate::data::common::{Broker, Id};
use crate::data::common::{
    Msg, PublicInput, PublicMsg, PublicStatus, SubscribeHis, SubscribeInput, SubscribeMsg,
    SubscribeStatus, SubscribeTopic, TabStatus,
};
use crate::data::{AString, AppEvent, EventUnSubscribe};
use crate::util::consts::QosToString;
use crate::util::db::ArcDb;
use crate::util::hint::*;
use anyhow::bail;
use anyhow::Result;
use custom_utils::{tx, tx_async};
use druid::im::Vector;
use druid::{im::HashMap, Data, Lens};
use for_mqtt_client::v3_1_1::{PubAck, SubAck, SubscribeReasonCode};
use for_mqtt_client::{SubscribeAck, SubscribeFilterAck};
use log::{debug, error, warn};

#[derive(Debug, Clone, Lens, Data)]
pub struct AppData {
    pub brokers: Vector<Broker>,
    pub broker_tabs: Vector<usize>,
    pub tab_statuses: HashMap<usize, TabStatus>,
    pub subscribe_hises: HashMap<usize, Vector<SubscribeHis>>,
    pub subscribe_topics: HashMap<usize, Vector<SubscribeTopic>>,
    pub msgs: HashMap<usize, Vector<Msg>>,
    pub subscribe_input: HashMap<usize, SubscribeInput>,
    pub public_input: HashMap<usize, PublicInput>,
    pub unsubscribe_ing: HashMap<usize, Vector<UnsubcribeTracing>>,
    #[data(ignore)]
    #[lens(ignore)]
    pub db: ArcDb,
    pub hint: AString,
}

impl AppData {
    pub fn add_broker(&mut self) {
        let broker = self.db.new_broker();
        self.init_broker_tab(broker.id);
        self.brokers.push_back(broker);
    }
    fn init_broker_tab(&mut self, id: usize) {
        if self.broker_tabs.iter().find(|x| **x == id).is_none() {
            self.broker_tabs.push_front(id);
        }
        if self.tab_statuses.get(&id).is_none() {
            self.tab_statuses.insert(
                id,
                TabStatus {
                    id: id,
                    try_connect: false,
                    connected: false,
                },
            );
        }
    }
    pub fn get_selected_subscribe_his(&self) -> Option<SubscribeHis> {
        if let Some(id) = self.get_selected_broker_id() {
            if let Some(hises) = self.subscribe_hises.get(&id) {
                if let Some(his) = hises.iter().find(|x| x.selected) {
                    return Some(his.clone());
                }
            }
        }
        warn!("could not find  subscribe his selected");
        None
    }
    pub fn get_selected_broker_id(&self) -> Option<usize> {
        self.brokers
            .iter()
            .find(|x| x.selected)
            .map(|x| x.id.clone())
    }
    pub fn get_selected_broker(&self) -> Option<&Broker> {
        self.brokers.iter().find(|x| x.selected)
    }
    pub fn find_broker(&self, id: usize) -> Option<&Broker> {
        self.brokers.iter().find(|x| (*x).id == id)
    }
    pub fn save_broker(&mut self, id: usize) -> Result<()> {
        if let Some(broker) = self.brokers.iter_mut().find(|x| (*x).id == id) {
            broker.stored = true;
            self.db.save_broker(id, broker)?;
            if !self.subscribe_hises.contains_key(&id) {
                self.subscribe_hises.insert(id, Vector::new());
            }
        }
        Ok(())
    }
    pub fn reconnect(&mut self, id: usize) -> Result<()> {
        self.disconnect(id)?;
        if let Some(broker) = self.brokers.iter().find(|x| (*x).id == id) {
            tx!(self.db.tx, AppEvent::Connect(broker.clone()))
        } else {
            error!("not find the broker");
        }
        Ok(())
    }
    pub fn init_connection(&mut self, id: usize) -> Result<()> {
        if let Some(status) = self.tab_statuses.get_mut(&id) {
            status.try_connect = true;
        }
        if let Some(broker) = self.brokers.iter_mut().find(|x| (*x).id == id) {
            broker.stored = true;
            self.db.save_broker(id, broker)?;
        }
        if self.subscribe_hises.get_mut(&id).is_none() {
            self.subscribe_hises.insert(id, Vector::new());
        }
        self.subscribe_topics.insert(id, Vector::new());
        self.msgs.insert(id, Vector::new());
        self.subscribe_input.insert(id, SubscribeInput::init(id));
        self.public_input.insert(id, PublicInput::default().into());
        Ok(())
    }
    pub fn connected(&mut self, id: usize) -> Result<()> {
        if let Some(status) = self.tab_statuses.get_mut(&id) {
            status.try_connect = false;
            status.connected = true;
        }
        Ok(())
    }
    pub fn disconnect(&mut self, id: usize) -> Result<()> {
        if let Some(status) = self.tab_statuses.get_mut(&id) {
            status.try_connect = false;
            status.connected = false;
        } else {
            debug!("not find the connection")
        }
        Ok(())
    }
    pub fn close_connection(&mut self, id: usize) {
        if let Some(status) = self.tab_statuses.get_mut(&id) {
            status.try_connect = false;
            status.connected = false;
        } else {
            error!("can't find the connection");
        }
    }
    pub fn unscribeing(
        &mut self,
        broker_id: usize,
        subscribe_pkid: u32,
        unsubscribe_pkid: u32,
    ) -> Result<()> {
        if let Some(_broker) = self.find_broker(broker_id) {
            if let Some(list) = self.unsubscribe_ing.get_mut(&broker_id) {
                list.push_back(UnsubcribeTracing {
                    subscribe_pk_id: subscribe_pkid,
                    unsubscribe_pk_id: unsubscribe_pkid,
                })
            } else {
                let mut list = Vector::new();
                list.push_back(UnsubcribeTracing {
                    subscribe_pk_id: subscribe_pkid,
                    unsubscribe_pk_id: unsubscribe_pkid,
                });
                self.unsubscribe_ing.insert(broker_id, list);
            }
        } else {
            bail!("can't find broker");
        }
        Ok(())
    }

    pub fn unsubscribe_ack(&mut self, broker_id: usize, unsubscribe_trace_id: u32) -> Result<()> {
        if let Some(_broker) = self.find_broker(broker_id) {
            if let Some(list) = self.unsubscribe_ing.get_mut(&broker_id) {
                if let Some(index) = list
                    .iter()
                    .enumerate()
                    .find(|(_index, x)| x.unsubscribe_pk_id == unsubscribe_trace_id)
                    .map(|(index, _x)| index)
                {
                    let tracing = list.remove(index);
                    if let Some(list) = self.subscribe_topics.get_mut(&broker_id) {
                        if let Some(index) = list
                            .iter_mut()
                            .enumerate()
                            .find(|(_index, his)| (*his).trace_id == tracing.subscribe_pk_id)
                            .map(|(index, _x)| index)
                        {
                            list.remove(index);
                            return Ok(());
                        } else {
                            bail!("can't find broker's subscribe");
                        }
                    }
                } else {
                    bail!("can't find broker's unsubscribe_tracing");
                }
            } else {
                bail!("can't find broker's unsubscribe_ing");
            }
        } else {
            bail!("can't find broker");
        }
        Ok(())
    }
    pub fn to_unscribe(&mut self, broker_id: usize, trace_id: u32) -> Result<()> {
        if let Some(_broker) = self.find_broker(broker_id) {
            if let Some(list) = self.subscribe_topics.get_mut(&broker_id) {
                if let Some(index) = list.iter_mut().find(|his| (*his).trace_id == trace_id) {
                    index.status = SubscribeStatus::UnSubscribeIng;
                    let event = EventUnSubscribe {
                        broke_id: broker_id,
                        subscribe_pk_id: index.trace_id,
                        topic: index.topic.as_ref().clone(),
                    };
                    tx!(self.db.tx, AppEvent::UnSubscribeIng(event));
                    return Ok(());
                }
            }
        }
        warn!("can't find the subscribe to unsubscibe");
        Ok(())
    }
    pub fn subscribe(&mut self, id: usize, input: SubscribeHis, trace_id: u32) -> Result<()> {
        if let Some(subscribe_topics) = self.subscribe_topics.get_mut(&id) {
            let sub = SubscribeTopic::from_his(input, trace_id);
            subscribe_topics.push_back(sub.into());
        }
        Ok(())
    }
    pub fn remove_subscribe_his(&mut self) -> Result<()> {
        let Some(id) = self.get_selected_broker_id() else {
            bail!(DELETE_SUBSCRIBE_NO_SELECTED);
        };
        if let Some(hises) = self.subscribe_hises.get_mut(&id) {
            if let Some(index) = hises
                .iter()
                .enumerate()
                .find(|(_index, his)| his.selected)
                .map(|(index, _his)| index)
            {
                hises.remove(index);
                self.db.update_subscribe_his(id, hises)?;
                return Ok(());
            }
        }
        bail!(DELETE_SUBSCRIBE_NO_SELECTED);
    }
    pub fn subscribe_by_input(
        &mut self,
        id: usize,
        input: SubscribeInput,
        packet_id: u32,
    ) -> Result<()> {
        if let Some(subscribe_topics) = self.subscribe_topics.get_mut(&id) {
            let sub = SubscribeTopic::from(input.clone(), packet_id);
            subscribe_topics.push_back(sub.into());
        }
        if let Some(subscribe_hises) = self.subscribe_hises.get_mut(&id) {
            let his: SubscribeHis = input.into();
            debug!("{:?}", subscribe_hises);
            debug!("{:?}", his);
            if subscribe_hises.iter().find(|x| *x == &his).is_none() {
                subscribe_hises.push_back(his.into());
                self.db.update_subscribe_his(id, &subscribe_hises)?;
            }
        }
        Ok(())
    }
    pub fn suback(&mut self, id: usize, mut input: SubscribeAck) {
        if let Some(subscribe_topics) = self.subscribe_topics.get_mut(&id) {
            let (id, ack) = (input.id, input.filter_ack.remove(0));
            if let Some(subscribe_topic) = subscribe_topics.iter_mut().find(|x| x.trace_id == id) {
                match ack.ack {
                    SubscribeReasonCode::Success(qos) => {
                        subscribe_topic.qos = qos.qos_to_string();
                        subscribe_topic.status = SubscribeStatus::SubscribeSuccess;
                    }
                    SubscribeReasonCode::Failure => {
                        subscribe_topic.status = SubscribeStatus::SubscribeFail;
                    }
                }
            } else {
                warn!("todo");
            }
        } else {
            warn!("todo");
        }
    }
    pub fn publish(&mut self, id: usize, input: PublicInput, trace_id: u32) {
        debug!("publish: tarce_id {}", trace_id);
        if let Some(msgs) = self.msgs.get_mut(&id) {
            let sub: Msg = PublicMsg::from(input.clone(), trace_id).into();
            msgs.push_back(sub.into());
        }
    }
    pub fn click_broker(&mut self, id: usize) -> Result<()> {
        self.select_broker(id);
        for (index, tab) in self.broker_tabs.iter().enumerate() {
            if *tab == id {
                tx!(self.db.tx, AppEvent::SelectTabs(index));
            }
        }
        Ok(())
    }
    pub fn edit_broker(&mut self) {
        if let Some(broker) = self.get_selected_broker() {
            self.init_broker_tab(broker.id);
        } else {
            // todo
            warn!("edit_broker: not selected broker");
        }
    }
    pub fn connect_broker(&mut self) {
        if let Some(broker) = self.get_selected_broker() {
            if let Err(e) = self.db.tx.send(AppEvent::Connect(broker.clone())) {
                error!("{:?}", e);
            }
            self.init_broker_tab(broker.id);
        } else {
            // todo
            warn!("connect_broker: not selected broker");
        }
    }
    pub fn db_click_broker(&mut self, id: usize) {
        self.init_broker_tab(id);
        if let Some(broker) = self.find_broker(id) {
            if let Err(e) = self.db.tx.send(AppEvent::Connect(broker.clone())) {
                error!("{:?}", e);
            }
        } else {
            error!("can't find broker");
        }
    }
    fn select_broker(&mut self, id: usize) {
        for broker in self.brokers.iter_mut() {
            if broker.id == id {
                broker.selected = true;
            } else {
                broker.selected = false;
            }
        }
    }
    pub fn delete_broker(&mut self) -> Result<()> {
        let mut selected_index = None;
        for (index, broker) in self.brokers.iter().enumerate() {
            if broker.selected {
                selected_index = Some(index);
                break;
            }
        }
        if let Some(index) = selected_index {
            let broker = self.brokers.remove(index);
            if let Some((index, _)) = self
                .broker_tabs
                .iter()
                .enumerate()
                .find(|x| *(*x).1 == broker.id)
            {
                self.broker_tabs.remove(index);
                debug!("close_tab：{} {}", index, self.broker_tabs.len());
            }
            self.tab_statuses.remove(&broker.id);
            self.db.delete_broker(broker.id)?;
            self.db.tx.send(AppEvent::Disconnect(index))?;
            // self.db.tx.send(AppEvent::CloseBrokerTab(index))?;
        } else {
            bail!("not selected broker to delete");
        }
        Ok(())
    }

    pub fn click_subscribe_his(&mut self, his: SubscribeHis) -> Result<()> {
        let Some(id) = self.get_selected_broker_id() else {
            warn!("could not get selected broker");
            return Ok(())
        };
        if let Some(hises) = self.subscribe_hises.get_mut(&id) {
            hises.iter_mut().for_each(|x| {
                if x == &his {
                    x.selected = true;
                } else {
                    x.selected = false;
                }
            });
        } else {
            warn!("could not get subscribe hises of broker selected");
        }
        Ok(())
    }

    pub fn close_tab(&mut self, id: usize) -> Result<()> {
        if let Some((index, _)) = self.broker_tabs.iter().enumerate().find(|x| *(*x).1 == id) {
            debug!("close_tab：{} {}", index, self.broker_tabs.len());
            self.broker_tabs.remove(index);
            // 删除未保存的broker todo 会导致tab的label panic
            if let Some((index, _broker)) = self.brokers.iter().enumerate().find(|x| {
                let broker = (*x).1;
                broker.id == id && broker.stored == false
            }) {
                self.brokers.remove(index);
                self.tab_statuses.remove(&id);
            }
            if self.db.tx.send(AppEvent::Disconnect(id)).is_err() {
                error!("fail to send event");
            }
        }
        Ok(())
    }
    pub fn pub_ack(&mut self, id: usize, trace_id: u32) {
        debug!("pub_ack: tarce_id {}", trace_id);
        if let Some(msgs) = self.msgs.get_mut(&id) {
            for msg in msgs.iter_mut() {
                if let Msg::Public(msg) = msg {
                    if msg.trace_id == trace_id {
                        msg.status = PublicStatus::Success;
                    }
                }
            }
        }
    }
    pub fn receive_msg(&mut self, id: usize, input: SubscribeMsg) {
        if let Some(msgs) = self.msgs.get_mut(&id) {
            let sub: Msg = input.into();
            msgs.push_back(sub.into());
        }
    }
    pub fn clear_msg(&mut self, id: usize) -> Result<()> {
        if let Some(msgs) = self.msgs.get_mut(&id) {
            msgs.clear();
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Data)]
pub struct UnsubcribeTracing {
    pub subscribe_pk_id: u32,
    pub unsubscribe_pk_id: u32,
}
