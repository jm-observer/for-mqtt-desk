use crate::data::common::{Broker, Id, PayloadTy, QoS};
use crate::data::common::{
    Msg, PublicMsg, PublicStatus, SubscribeHis, SubscribeMsg, SubscribeStatus, SubscribeTopic,
};
use crate::data::{AString, AppEvent, EventUnSubscribe};
use crate::mqtt::data::MqttPublicInput;
use crate::util::consts::QosToString;
use crate::util::db::ArcDb;
use crate::util::hint::*;
use crate::util::now_time;
use anyhow::Result;
use anyhow::{anyhow, bail};
use bytes::Bytes;
use crossbeam_channel::Sender;
use custom_utils::tx;
use druid::im::Vector;
use druid::{Data, Lens};
use for_mqtt_client::protocol::packet::SubscribeReasonCode;
use for_mqtt_client::SubscribeAck;
use log::{debug, error, warn};
use std::sync::Arc;
//
// #[derive(Debug, Clone, Lens, Data)]
// pub struct AppData {
//     pub brokers: Vector<Broker>,
//     pub broker_tabs: Vector<usize>,
//     pub tab_statuses: HashMap<usize, TabStatus>,
//     pub subscribe_hises: HashMap<usize, Vector<SubscribeHis>>,
//     pub subscribe_topics: HashMap<usize, Vector<SubscribeTopic>>,
//     pub msgs: HashMap<usize, Vector<Msg>>,
//     pub subscribe_input: HashMap<usize, SubscribeInput>,
//     pub public_input: HashMap<usize, PublicInput>,
//     pub unsubscribe_ing: HashMap<usize, Vector<UnsubcribeTracing>>,
//     #[data(ignore)]
//     #[lens(ignore)]
//     pub db: ArcDb,
//     pub hint: AString,
//     /// #[data(ignore)] 不能加这个，不然就无法改变self_signed_file。为什么？不知道！简单的案例无法复现出来。
//     #[lens(ignore)]
//     pub self_signed_file: Option<usize>,
//     pub display_history: bool,
//     pub display_broker_info: bool,
// }

#[derive(Debug, Clone, Lens, Data)]
pub struct AppData {
    pub brokers: Vector<Broker>,
    pub broker_tabs: Vector<usize>,
    #[data(ignore)]
    #[lens(ignore)]
    pub db: ArcDb,
    pub hint: AString,
    /// #[data(ignore)] 不能加这个，不然就无法改变self_signed_file。为什么？不知道！简单的案例无法复现出来。
    #[lens(ignore)]
    pub self_signed_file: Option<usize>,
    pub display_history: bool,
    pub display_broker_info: bool,
    #[data(ignore)]
    #[lens(ignore)]
    pub tx: Sender<AppEvent>,
}

impl AppData {
    pub(crate) fn client_disconnect(&mut self, id: usize) -> Result<()> {
        let broker = self.find_mut_broker_by_id(id)?;
        broker.disconnect(false);
        Ok(())
    }
    fn send_event(&self, event: AppEvent) {
        if let Err(e) = self.tx.send(event) {
            error!("fail to send event: {:?}", e.0)
        }
    }
    pub fn set_self_signed_file(&mut self, index: usize) {
        self.self_signed_file = Some(index);
        self.self_signed_file.replace(index);
    }
    pub fn get_self_signed_file(&self) -> Option<usize> {
        self.self_signed_file.clone()
    }
    pub fn touch_add_broker(&mut self) {
        self.unselect_broker();
        if let Some(broker) = self.brokers.iter_mut().find(|x| x.stored == false) {
            debug!("had a new broker");
            broker.selected = true;
            self.display_broker_info = true;
        } else {
            debug!("new_broker");
            let mut broker = self.db.new_broker();
            broker.selected = true;
            self.brokers.push_front(broker);
            self.display_broker_info = true;
        }
    }
    /// 取消所有选中
    pub fn unselect_broker(&mut self) {
        self.brokers.iter_mut().for_each(|x| x.selected = false);
    }
    fn init_broker_tab(&mut self, id: usize) -> bool {
        let mut is_exist = false;
        if self.broker_tabs.iter().find(|x| **x == id).is_none() {
            self.broker_tabs.push_front(id);
        } else {
            is_exist = true;
        }
        is_exist
    }
    pub fn get_selected_subscribe_his(&self) -> Result<SubscribeHis> {
        let broker = self.get_selected_broker()?;
        if let Some(his) = broker.subscribe_hises.iter().find(|x| x.selected) {
            return Ok(his.clone());
        }
        bail!("could not find  subscribe his selected");
    }
    pub fn get_selected_broker_id(&self) -> Option<usize> {
        self.brokers
            .iter()
            .find(|x| x.selected)
            .map(|x| x.id.clone())
    }
    pub fn get_selected_broker_index(&self) -> Option<usize> {
        self.brokers
            .iter()
            .enumerate()
            .find(|x| x.1.selected)
            .map(|x| x.0)
    }
    pub fn get_selected_broker(&self) -> Result<&Broker> {
        self.brokers
            .iter()
            .find(|x| x.selected)
            .ok_or(anyhow!("could not find broker selected"))
    }

    pub fn get_selected_mut_broker(&mut self) -> Result<&mut Broker> {
        self.brokers
            .iter_mut()
            .find(|x| x.selected)
            .ok_or(anyhow!("could not find broker selected"))
    }
    pub fn get_mut_selected_broker_or_zero(&mut self) -> &mut Broker {
        let _index = match self.get_selected_broker_index() {
            None => 0,
            Some(index) => index,
        };
        self.brokers.get_mut(_index).unwrap()
    }
    pub fn get_selected_broker_or_zero(&self) -> &Broker {
        // for i in self.brokers.iter() {
        //     debug!("{} {}", i.id, i.selected);
        // }
        if let Some(broker) = self.brokers.iter().find(|x| x.selected) {
            broker
        } else {
            self.brokers.get(0).unwrap()
        }
    }

    pub fn find_broker_by_id(&self, id: usize) -> Result<&Broker> {
        self.brokers
            .iter()
            .find(|x| (*x).id == id)
            .ok_or(anyhow!("could not find broker:{}", id))
    }
    pub fn find_mut_broker_by_id(&mut self, id: usize) -> Result<&mut Broker> {
        self.brokers
            .iter_mut()
            .find(|x| (*x).id == id)
            .ok_or(anyhow!("could not find broker:{}", id))
    }
    pub fn find_broker_by_index(&self, id: usize) -> Result<&Broker> {
        self.brokers
            .get(id)
            .ok_or(anyhow!("could not find broker:{}", id))
    }
    pub fn find_mut_broker_by_index(&mut self, id: usize) -> Result<&mut Broker> {
        self.brokers
            .get_mut(id)
            .ok_or(anyhow!("could not find broker:{}", id))
    }
    pub fn touch_save_broker(&mut self) -> Result<()> {
        let broker = self.get_selected_mut_broker()?;
        broker.stored = true;
        let broker = broker.clone_to_db();
        self.db.save_broker(broker)?;
        Ok(())
    }
    pub fn touch_reconnect(&mut self) -> Result<()> {
        let broker = self.get_selected_mut_broker()?;
        broker.disconnect(false);
        broker.init_connection()?;
        let broker = broker.clone();
        self.disconnect(broker.id)?;
        self.init_connection_by_broker(broker)?;
        Ok(())
    }
    pub fn init_connection_for_selected(&mut self) -> Result<()> {
        let broker = self.get_selected_mut_broker()?;
        broker.init_connection()?;
        let broker = broker.clone();
        self.init_connection_by_broker(broker)?;
        self.display_broker_info = false;
        Ok(())
    }

    fn init_connection_by_broker(&mut self, broker: Broker) -> Result<()> {
        let broker_db = broker.clone_to_db();
        let broker = broker.clone();
        self.init_broker_tab(broker.id);
        self.db.save_broker(broker_db)?;
        self.display_broker_info = false;
        self.send_event(AppEvent::ToConnect(broker));
        Ok(())
    }

    pub fn update_to_connected(&mut self, id: usize, _retain: bool) -> Result<()> {
        let broker = self.find_mut_broker_by_id(id)?;
        let status = &mut broker.tab_status;
        status.try_connect = false;
        status.connected = true;
        if !_retain {
            broker.subscribe_topics.clear();
        }
        Ok(())
    }
    pub(crate) fn touch_disconnect(&mut self) -> Result<()> {
        let broker = self.get_selected_mut_broker()?;
        broker.disconnect(false);
        let id = broker.id;
        self.disconnect(id)
    }
    fn disconnect(&self, id: usize) -> Result<()> {
        self.send_event(AppEvent::ToDisconnect(id));
        Ok(())
    }
    pub fn close_connection(&mut self, id: usize) -> Result<()> {
        let status = &mut self.find_mut_broker_by_id(id)?.tab_status;
        status.try_connect = false;
        status.connected = false;
        Ok(())
    }
    pub fn to_unsubscribe(
        &mut self,
        broker_id: usize,
        subscribe_pkid: u32,
        unsubscribe_pkid: u32,
    ) -> Result<()> {
        let _broker = self.find_mut_broker_by_id(broker_id)?;
        _broker.unsubscribe_ing.push_back(UnsubcribeTracing {
            subscribe_pk_id: subscribe_pkid,
            unsubscribe_pk_id: unsubscribe_pkid,
        });
        Ok(())
    }

    pub fn unsubscribe_ack(&mut self, broker_id: usize, unsubscribe_trace_id: u32) -> Result<()> {
        let _broker = self.find_mut_broker_by_id(broker_id)?;
        if let Some(index) = _broker
            .unsubscribe_ing
            .iter()
            .enumerate()
            .find(|(_index, x)| x.unsubscribe_pk_id == unsubscribe_trace_id)
            .map(|(index, _x)| index)
        {
            let tracing = _broker.unsubscribe_ing.remove(index);
            if let Some(index) = _broker
                .subscribe_topics
                .iter_mut()
                .enumerate()
                .find(|(_index, his)| (*his).trace_id == tracing.subscribe_pk_id)
                .map(|(index, _x)| index)
            {
                _broker.subscribe_topics.remove(index);
                return Ok(self.db.tx.send(AppEvent::UpdateScrollSubscribeWin)?);
            } else {
                bail!("can't find broker's subscribe");
            }
        } else {
            bail!("can't find broker's unsubscribe_tracing");
        }
    }
    pub fn touch_unsubscribe(&mut self, broker_id: usize, trace_id: u32) -> Result<()> {
        let _broker = self.find_mut_broker_by_id(broker_id)?;
        if let Some(index) = _broker
            .subscribe_topics
            .iter_mut()
            .find(|his| (*his).trace_id == trace_id)
        {
            index.status = SubscribeStatus::UnSubscribeIng;
            let event = EventUnSubscribe {
                broke_id: broker_id,
                subscribe_pk_id: index.trace_id,
                topic: index.topic.as_ref().clone(),
            };
            self.send_event(AppEvent::ToUnsubscribeIng(event));
            return Ok(());
        }
        warn!("can't find the subscribe to unsubscibe");
        Ok(())
    }

    fn subscribe(&mut self, sub: SubscribeTopic) -> Result<()> {
        let id = sub.broker_id;
        let broker = self.find_mut_broker_by_id(id)?;
        if broker
            .subscribe_topics
            .iter()
            .find(|x| x.is_equal(&sub))
            .is_none()
        {
            broker.subscribe_topics.push_back(sub.clone().into());
        } else if let Some((index, _)) = broker
            .subscribe_topics
            .iter()
            .enumerate()
            .find(|(_index, x)| x.topic == sub.topic)
        {
            broker.subscribe_topics.remove(index);
            broker.subscribe_topics.push_back(sub.clone().into());
        }

        let his: SubscribeHis = sub.clone().into();
        if broker.subscribe_hises.iter().find(|x| *x == &his).is_none() {
            broker.subscribe_hises.push_back(his.into());
            let broker = broker.clone_to_db();
            self.db.save_broker(broker)?;
        }
        self.db.tx.send(AppEvent::ToSubscribe(sub))?;
        self.db.tx.send(AppEvent::UpdateScrollSubscribeWin)?;

        Ok(())
    }
    pub fn touch_subscribe_from_his(&mut self, input: SubscribeHis) -> Result<()> {
        debug!("{:?}", input);
        self.subscribe(SubscribeTopic::from_his(input, Id::to_id()))?;
        Ok(())
    }

    pub fn touch_subscribe_by_input(&mut self, id: usize) -> Result<()> {
        let input = self.find_broker_by_id(id)?.subscribe_input.clone();
        self.subscribe(SubscribeTopic::from(input, Id::to_id()))?;
        Ok(())
    }

    // pub fn subscribe_by_input(
    //     &mut self,
    //     id: usize,
    //     input: SubscribeInput,
    //     trace_id: u32,
    // ) -> Result<()> {
    //     self.subscribe(id, SubscribeTopic::from(input.clone(), trace_id))?;
    //     let broker = self.find_mut_broker_by_id(id)?;
    //     let his: SubscribeHis = input.into();
    //     if broker.subscribe_hises.iter().find(|x| *x == &his).is_none() {
    //         broker.subscribe_hises.push_back(his.into());
    //     }
    //     Ok(self.db.tx.send(AppEvent::ScrollSubscribeWin)?)
    // }

    pub fn touch_remove_subscribe_his(&mut self, id: usize) -> Result<()> {
        let broker = self.find_mut_broker_by_id(id)?;
        if let Some(index) = broker
            .subscribe_hises
            .iter()
            .enumerate()
            .find(|(_index, his)| his.selected)
            .map(|(index, _his)| index)
        {
            broker.subscribe_hises.remove(index);
            let broker = broker.clone_to_db();
            self.db.save_broker(broker)?;
            return Ok(());
        }
        warn!("{}", DELETE_SUBSCRIBE_NO_SELECTED);
        Ok(())
    }
    pub fn touch_click_tab(&mut self, broker_id: usize) -> Result<()> {
        self.select_broker(broker_id);
        Ok(())
    }

    pub fn sub_ack(&mut self, id: usize, input: SubscribeAck) -> Result<()> {
        let broker = self.find_mut_broker_by_id(id)?;
        let SubscribeAck { id, mut acks } = input;
        if let Some(ack) = acks.pop() {
            if let Some(subscribe_topic) = broker
                .subscribe_topics
                .iter_mut()
                .find(|x| x.trace_id == id)
            {
                match ack {
                    SubscribeReasonCode::QoS0 => {
                        subscribe_topic.qos = QoS::AtMostOnce.clone();
                        subscribe_topic.status = SubscribeStatus::SubscribeSuccess;
                    }
                    SubscribeReasonCode::QoS1 => {
                        subscribe_topic.qos = QoS::AtLeastOnce.clone();
                        subscribe_topic.status = SubscribeStatus::SubscribeSuccess;
                    }
                    SubscribeReasonCode::QoS2 => {
                        subscribe_topic.qos = QoS::ExactlyOnce.clone();
                        subscribe_topic.status = SubscribeStatus::SubscribeSuccess;
                    }
                    _reasone => {
                        subscribe_topic.status = SubscribeStatus::SubscribeFail;
                    }
                }
            } else {
                warn!("could not find subscribe");
            }
        }
        Ok(())
        // } else {
        //     warn!("could not find subscribe");
        // }
    }
    pub fn publish(&mut self, id: usize) -> Result<()> {
        let broker = self.find_mut_broker_by_id(id)?;
        let (payload, payload_str) = broker
            .public_input
            .payload_ty
            .to_bytes(&broker.public_input.msg)?;
        let trace_id = Id::to_id();
        let msg = PublicMsg {
            trace_id,
            topic: broker.public_input.topic.clone(),
            msg: Arc::new(payload_str),
            qos: broker.public_input.qos.qos_to_string(),
            status: PublicStatus::Ing,
            payload_ty: broker.public_input.payload_ty.to_arc_string(),
            time: Arc::new(now_time()),
        };
        debug!("publish: tarce_id {}", trace_id);

        broker.msgs.push_back(msg.into());
        if broker.msgs.len() > 50 {
            broker.msgs.pop_front();
        }

        let publish = MqttPublicInput {
            broker_id: broker.id,
            trace_id,
            topic: broker.public_input.topic.clone(),
            msg: payload,
            qos: broker.public_input.qos.clone(),
            retain: broker.public_input.retain,
        };

        self.send_event(AppEvent::ToPublish(publish));
        self.send_event(AppEvent::UpdateScrollMsgWin);
        Ok(())
    }
    pub fn click_broker(&mut self, id: usize) -> Result<()> {
        self.select_broker_and_display(id);
        for (index, tab) in self.broker_tabs.iter().enumerate() {
            if *tab == id {
                tx!(self.db.tx, AppEvent::UpdateToSelectTabs(index));
            }
        }
        Ok(())
    }
    pub fn edit_broker(&mut self) {
        if let Ok(broker) = self.get_selected_broker() {
            self.init_broker_tab(broker.id);
        } else {
            warn!("edit_broker: not selected broker");
        }
    }
    pub fn touch_connect_broker_selected(&mut self) -> Result<()> {
        self.init_connection_for_selected()?;
        Ok(())
    }
    pub fn db_click_broker(&mut self, id: usize) -> Result<()> {
        // 若已存在，则跳转至该tag；重连。否则，新增tag，连接
        let broker = self.find_mut_broker_by_id(id)?;
        broker.disconnect(false);
        broker.init_connection()?;
        let broker = broker.clone();
        self.disconnect(broker.id)?;
        self.init_connection_by_broker(broker)?;
        // if self.init_broker_tab(id) {
        //     self.db.tx.send(AppEvent::ReConnect(id))?;
        // } else {
        //     let broker = self.find_broker_by_id(id)?;
        //     self.db.tx.send(AppEvent::ToDisconnect(id))?;
        // }
        Ok(())
    }
    fn select_broker_and_display(&mut self, id: usize) {
        for broker in self.brokers.iter_mut() {
            if broker.id == id {
                broker.selected = true;
            } else {
                broker.selected = false;
            }
        }
        self.display_broker_info = true;
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
    pub fn touch_delete_broker_selected(&mut self) -> Result<()> {
        // let broker = self.get_selected_mut_broker()?;
        let mut broker = self.brokers.remove(
            self.get_selected_broker_index()
                .ok_or(anyhow!("could not find broker selected"))?,
        );
        broker.disconnect(true);
        let id = broker.id;
        self.close_broker_tab(id)?;
        self.disconnect(id)?;

        self.db.delete_broker(id)?;
        if self.brokers.len() == 0 {
            self.touch_add_broker();
        }
        Ok(())
    }

    pub fn init_broker(&mut self) {
        if self.brokers.len() == 0 {
            self.touch_add_broker();
            self.display_broker_info = true;
            self.display_history = false;
        } else {
            self.display_broker_info = false;
            self.display_history = true;
        }
    }

    pub fn click_subscribe_his(&mut self, his: SubscribeHis) -> Result<()> {
        let Some(id) = self.get_selected_broker_id() else {
            warn!("could not get selected broker");
            return Ok(())
        };
        let broker = self.find_mut_broker_by_id(id)?;
        broker.subscribe_hises.iter_mut().for_each(|x| {
            if x == &his {
                x.selected = true;
            } else {
                x.selected = false;
            }
        });
        Ok(())
    }

    pub fn touch_close_broker_tab(&mut self, id: usize) -> Result<()> {
        self.close_broker_tab(id)?;
        self.find_mut_broker_by_id(id)?.disconnect(true);
        self.disconnect(id)?;
        Ok(())
    }

    fn close_broker_tab(&mut self, id: usize) -> Result<()> {
        if let Some((index, _)) = self.broker_tabs.iter().enumerate().find(|x| *(*x).1 == id) {
            debug!("close_tab：{} {}", index, self.broker_tabs.len());
            self.broker_tabs.remove(index);
        }
        Ok(())
    }

    pub fn pub_ack(&mut self, id: usize, trace_id: u32) -> Result<()> {
        debug!("pub_ack: tarce_id {}", trace_id);
        let broker = self.find_mut_broker_by_id(id)?;
        let mut is_ack = false;
        for msg in broker.msgs.iter_mut() {
            if let Msg::Public(msg) = msg {
                if msg.trace_id == trace_id {
                    is_ack = true;
                    msg.status = PublicStatus::Success;
                }
            }
        }
        if !is_ack {
            bail!("pub_ack could not find pub({})", trace_id);
        }
        Ok(())
    }
    pub fn receive_msg(
        &mut self,
        id: usize,
        topic: Arc<String>,
        payload: Arc<Bytes>,
        qos: QoS,
    ) -> Result<()> {
        let broker = self.find_mut_broker_by_id(id)?;
        let payload_ty = if let Some(subscribe) = broker
            .subscribe_topics
            .iter()
            .find(|x| x.match_topic(topic.as_str()))
        {
            subscribe.payload_ty.clone()
        } else {
            warn!("could not find this publish's subscribe record");
            PayloadTy::default()
        };
        let payload = payload_ty.format(payload);
        let msg = SubscribeMsg {
            topic,
            msg: Arc::new(payload),
            qos: qos.qos_to_string(),
            payload_ty: payload_ty.to_arc_string(),
            time: Arc::new(now_time()),
        };
        broker.msgs.push_back(msg.into());
        if broker.msgs.len() > 50 {
            broker.msgs.pop_front();
        }
        Ok(self.db.tx.send(AppEvent::UpdateScrollMsgWin)?)
    }
    pub fn clear_msg(&mut self, id: usize) -> Result<()> {
        let _broker = self.find_mut_broker_by_id(id)?.msgs.clear();
        Ok(self.db.tx.send(AppEvent::UpdateScrollMsgWin)?)
    }

    // pub fn msgs_ref_mut(&mut self, id: usize) -> &mut Vector<Msg> {
    //     if !self.msgs.contains_key(&id) {
    //         self.msgs.insert(id, Vector::new());
    //     }
    //     let Some(msgs) = self.msgs.get_mut(&id) else {
    //             unreachable!()
    //     };
    //     msgs
    // }
    // pub fn msgs_ref(&self, id: usize) -> &Vector<Msg> {
    //     if let Some(msgs) = self.msgs.get(&id) {
    //         msgs
    //     } else {
    //         unreachable!()
    //     }
    // }
}
#[derive(Debug, Clone, Data)]
pub struct UnsubcribeTracing {
    pub subscribe_pk_id: u32,
    pub unsubscribe_pk_id: u32,
}
