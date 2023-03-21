pub mod click_ty;
pub mod common;
pub mod db;
pub mod hierarchy;
pub mod lens;

use crate::data::click_ty::ClickTy;
use crate::data::common::{QoS, SubscribeHis, SubscribeTopic};
use bytes::Bytes;
use common::Broker;

use crate::mqtt::data::MqttPublicInput;
use for_mqtt_client::{SubscribeAck, UnsubscribeAck};
use std::sync::Arc;

pub type AString = Arc<String>;

#[derive(Debug)]
pub enum AppEvent {
    /// broker列表的新增图标。新增broker
    TouchAddBroker,
    /// broker列表的编辑图标。编辑选择的broker
    TouchEditBrokerSelected,
    /// broker列表的连接图标。连接选择的broker
    TouchConnectBrokerSelected,
    /// broker列表的删除图标。删除选择的broker
    TouchDeleteBrokerSelected,
    SaveBroker(usize),
    ReConnect(usize),
    /// broker信息界面中连接按钮。
    ConnectByButton(usize),
    /// 调用第三方库连接broker
    ToConnect(Broker),
    /// 调用第三方库断开连接
    ToDisconnect(usize),
    // e.g: delete broker; close tab; click button "disconnect"
    Disconnect(usize),
    // select brokers tab
    SelectTabs(usize),
    RemoveSubscribeHis,
    /// 根据输入进行订阅
    TouchSubscribeByInput(usize),
    TouchSubscribeFromHis(SubscribeHis),
    /// 通知client进行订阅
    ToSubscribe(SubscribeTopic),
    ToUnSubscribe {
        broker_id: usize,
        trace_id: u32,
    },
    ToPublish(MqttPublicInput),
    UnSubscribeIng(EventUnSubscribe),
    ConnectAckSuccess(usize),
    ConnectAckFail(usize, Arc<String>),
    TouchPublic(usize),
    ReceivePublic(usize, Arc<String>, Arc<Bytes>, QoS),
    PubAck(usize, u32),
    SubAck(usize, SubscribeAck),
    UnSubAck(usize, UnsubscribeAck),
    Click(ClickTy),
    ClickLifeDead(ClickTy),
    CloseBrokerTab(usize),
    CloseConnectionTab(usize),

    UpdateStatusBar(String),
    /// 清空消息
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
