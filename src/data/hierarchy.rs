use crate::data::common::{Broker, Id, PayloadTy, QoS, SignedTy};
use crate::data::common::{
    Msg, PublicInput, PublicMsg, PublicStatus, SubscribeHis, SubscribeInput, SubscribeMsg,
    SubscribeStatus, SubscribeTopic, TabStatus,
};
use crate::data::{AString, AppEvent, EventUnSubscribe};
use crate::util::consts::QosToString;
use crate::util::db::ArcDb;
use crate::util::hint::*;
use crate::util::{general_id, now_time};
use anyhow::Result;
use anyhow::{anyhow, bail};
use bytes::Bytes;
use crossbeam_channel::Sender;
use custom_utils::{tx, tx_async};
use druid::im::Vector;
use druid::{im::HashMap, Data, Lens};
use for_mqtt_client::protocol::packet::{PubAck, SubscribeReasonCode};
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
        let index = match self.get_selected_broker_index() {
            None => 0,
            Some(index) => index,
        };
        self.brokers.get_mut(0).unwrap()
    }
    pub fn get_selected_broker_or_zero(&self) -> &Broker {
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
    pub fn save_broker(&mut self, id: usize) -> Result<()> {
        if let Some(broker) = self.brokers.iter_mut().find(|x| (*x).id == id) {
            broker.stored = true;
            self.db.save_broker(broker.clone_to_db())?;
        }
        Ok(())
    }
    pub fn reconnect(&mut self, id: usize) -> Result<()> {
        self.disconnect(id)?;
        self.init_connection(id)?;
        // if let Some(broker) = self.brokers.iter().find(|x| (*x).id == id) {
        //     tx!(self.db.tx, AppEvent::Connect(broker.clone()))
        // } else {
        //     error!("not find the broker");
        // }
        Ok(())
    }
    pub fn init_connection(&mut self, id: usize) -> Result<()> {
        let broker = self.find_mut_broker_by_id(id)?;
        if broker.client_id.as_str().is_empty() {
            broker.client_id = general_id().into();
        }

        if broker.addr.is_empty() {
            bail!("addr not be empty");
        } else if broker.port.is_none() {
            bail!("port not be empty");
        } else if broker.use_credentials {
            if broker.user_name.is_empty() {
                bail!("user name not be empty");
            } else if broker.password.is_empty() {
                bail!("password not be empty");
            }
        } else if broker.tls && broker.signed_ty == SignedTy::SelfSigned {
            if broker.self_signed_ca.is_empty() {
                bail!("self signed ca not be empty");
            }
        }
        broker.tab_status.try_connect = true;
        broker.stored = true;
        let broker_db = broker.clone_to_db();
        let broker = broker.clone();
        self.init_broker_tab(id);
        self.db.save_broker(broker_db)?;
        self.display_broker_info = false;
        self.send_event(AppEvent::ToConnect(broker));
        Ok(())
    }
    pub fn update_to_connected(&mut self, id: usize) -> Result<()> {
        let status = &mut self.find_mut_broker_by_id(id)?.tab_status;
        status.try_connect = false;
        status.connected = true;
        Ok(())
    }
    pub fn disconnect(&mut self, id: usize) -> Result<()> {
        let broker = self.find_mut_broker_by_id(id)?;
        broker.tab_status.try_connect = false;
        broker.tab_status.connected = false;
        broker.subscribe_topics.clear();
        broker.msgs.clear();
        broker.unsubscribe_ing.clear();
        self.send_event(AppEvent::ToDisconnect(id));
        Ok(())
    }
    pub fn close_connection(&mut self, id: usize) -> Result<()> {
        let status = &mut self.find_mut_broker_by_id(id)?.tab_status;
        status.try_connect = false;
        status.connected = false;
        Ok(())
    }
    pub fn unsubscribe(
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
                return Ok(self.db.tx.send(AppEvent::ScrollSubscribeWin)?);
            } else {
                bail!("can't find broker's subscribe");
            }
        } else {
            bail!("can't find broker's unsubscribe_tracing");
        }
    }
    pub fn to_unscribe(&mut self, broker_id: usize, trace_id: u32) -> Result<()> {
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
            tx!(self.db.tx, AppEvent::UnSubscribeIng(event));
            return Ok(());
        }
        warn!("can't find the subscribe to unsubscibe");
        Ok(())
    }

    fn subscribe(&mut self, id: usize, sub: SubscribeTopic) -> Result<()> {
        let broker = self.find_mut_broker_by_id(id)?;
        if broker
            .subscribe_topics
            .iter()
            .find(|x| x.is_equal(&sub))
            .is_none()
        {
            broker.subscribe_topics.push_back(sub.into());
        } else if let Some((index, _)) = broker
            .subscribe_topics
            .iter()
            .enumerate()
            .find(|(index, x)| x.topic == sub.topic)
        {
            broker.subscribe_topics.remove(index);
            broker.subscribe_topics.push_back(sub.into());
        }
        Ok(())
    }
    pub fn subscribe_by_his(
        &mut self,
        id: usize,
        input: SubscribeHis,
        trace_id: u32,
    ) -> Result<()> {
        self.subscribe(id, SubscribeTopic::from_his(input, trace_id))?;
        Ok(self.db.tx.send(AppEvent::ScrollSubscribeWin)?)
    }

    pub fn subscribe_by_input(
        &mut self,
        id: usize,
        input: SubscribeInput,
        trace_id: u32,
    ) -> Result<()> {
        self.subscribe(id, SubscribeTopic::from(input.clone(), trace_id))?;
        let broker = self.find_mut_broker_by_id(id)?;
        let his: SubscribeHis = input.into();
        if broker.subscribe_hises.iter().find(|x| *x == &his).is_none() {
            broker.subscribe_hises.push_back(his.into());
        }
        Ok(self.db.tx.send(AppEvent::ScrollSubscribeWin)?)
    }
    pub fn remove_subscribe_his(&mut self) -> Result<()> {
        let Some(id) = self.get_selected_broker_id() else {
            bail!(DELETE_SUBSCRIBE_NO_SELECTED);
        };
        let broker = self.find_mut_broker_by_id(id)?;
        if let Some(index) = broker
            .subscribe_hises
            .iter()
            .enumerate()
            .find(|(_index, his)| his.selected)
            .map(|(index, _his)| index)
        {
            broker.subscribe_hises.remove(index);
            return Ok(());
        }
        bail!(DELETE_SUBSCRIBE_NO_SELECTED);
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
                        subscribe_topic.qos = QoS::AtMostOnce.qos_to_string();
                        subscribe_topic.status = SubscribeStatus::SubscribeSuccess;
                    }
                    SubscribeReasonCode::QoS1 => {
                        subscribe_topic.qos = QoS::AtLeastOnce.qos_to_string();
                        subscribe_topic.status = SubscribeStatus::SubscribeSuccess;
                    }
                    SubscribeReasonCode::QoS2 => {
                        subscribe_topic.qos = QoS::ExactlyOnce.qos_to_string();
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
    pub fn publish(&mut self, id: usize, input: PublicMsg, trace_id: u32) -> Result<()> {
        debug!("publish: tarce_id {}", trace_id);
        let broker = self.find_mut_broker_by_id(id)?;
        broker.msgs.push_back(input.into());
        if broker.msgs.len() > 50 {
            broker.msgs.pop_front();
        }
        Ok(self.db.tx.send(AppEvent::ScrollMsgWin)?)
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
        if let Ok(broker) = self.get_selected_broker() {
            self.init_broker_tab(broker.id);
        } else {
            // todo
            warn!("edit_broker: not selected broker");
        }
    }
    pub fn connect_broker_selected(&mut self) -> Result<()> {
        let broker = self.get_selected_broker()?;
        self.init_connection(broker.id)?;
        Ok(())
    }
    pub fn db_click_broker(&mut self, id: usize) -> Result<()> {
        // 若已存在，则跳转至该tag；重连。否则，新增tag，连接
        if self.broker_tabs.iter().find(|x| **x == id).is_some() {
            self.disconnect(id)?;
        }
        self.init_connection(id)?;
        // if self.init_broker_tab(id) {
        //     self.db.tx.send(AppEvent::ReConnect(id))?;
        // } else {
        //     let broker = self.find_broker_by_id(id)?;
        //     self.db.tx.send(AppEvent::ToDisconnect(id))?;
        // }
        Ok(())
    }
    fn select_broker(&mut self, id: usize) {
        for broker in self.brokers.iter_mut() {
            if broker.id == id {
                broker.selected = true;
            } else {
                broker.selected = false;
            }
        }
        self.display_broker_info = true;
    }
    pub fn touch_delete_broker_selected(&mut self) -> Result<()> {
        let broker = self.brokers.remove(
            self.get_selected_broker_index()
                .ok_or(anyhow!("could not find broker selected"))?,
        );
        if let Some((index, _)) = self
            .broker_tabs
            .iter()
            .enumerate()
            .find(|x| *(*x).1 == broker.id)
        {
            self.broker_tabs.remove(index);
            self.disconnect(broker.id)?;
        }
        self.db.delete_broker(broker.id)?;
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
            }
            if self.db.tx.send(AppEvent::Disconnect(id)).is_err() {
                error!("fail to send event");
            }
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
        let payload_ty =
            if let Some(subscribe) = broker.subscribe_topics.iter().find(|x| x.topic == topic) {
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
        Ok(self.db.tx.send(AppEvent::ScrollMsgWin)?)
    }
    pub fn clear_msg(&mut self, id: usize) -> Result<()> {
        let broker = self.find_mut_broker_by_id(id)?.msgs.clear();
        Ok(self.db.tx.send(AppEvent::ScrollMsgWin)?)
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
