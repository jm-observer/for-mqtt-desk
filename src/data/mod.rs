pub mod click_ty;
pub mod common;
pub mod db;
pub mod hierarchy;
pub mod lens;

use crate::data::click_ty::ClickTy;
use crate::data::common::{Id, PublicInput, QoS, SubscribeHis, SubscribeInput, SubscribeMsg};
use bytes::Bytes;
use common::Broker;
use for_mqtt_client::protocol::packet::{PubAck, SubAck};
use for_mqtt_client::{SubscribeAck, UnsubscribeAck};
use std::sync::Arc;

pub type AString = Arc<String>;

#[derive(Debug)]
pub enum AppEvent {
    RemoveSubscribeHis,
    AddBroker,
    EditBroker,
    ConnectBroker,
    SaveBroker(usize),
    ReConnect(usize),
    // select brokers tab
    SelectTabs(usize),
    Connect(Broker),
    Subscribe(SubscribeInput, usize),
    SubscribeFromHis(SubscribeHis),
    ToUnSubscribe {
        broker_id: usize,
        trace_id: u32,
    },
    UnSubscribeIng(EventUnSubscribe),
    ConnectAckSuccess(usize),
    ConnectAckFail(usize, Arc<String>),
    Public(PublicInput, usize),
    ReceivePublic(usize, Arc<String>, Arc<Bytes>, QoS),
    PubAck(usize, u32),
    SubAck(usize, SubscribeAck),
    UnSubAck(usize, UnsubscribeAck),
    Click(ClickTy),
    ClickLifeDead(ClickTy),
    CloseBrokerTab(usize),
    CloseConnectionTab(usize),
    DeleteBroker,
    // e.g: delete broker; close tab; click button "disconnect"
    Disconnect(usize),
    UpdateStatusBar(String),
    //
    ClearMsg(usize),
    /// 滚动消息窗口
    ScrollMsgWin,
    /// 滚动订阅窗口
    ScrollSubscribeWin,
}
#[derive(Debug, Clone)]
pub struct EventUnSubscribe {
    pub broke_id: usize,
    pub subscribe_pk_id: u32,
    pub topic: String,
}
