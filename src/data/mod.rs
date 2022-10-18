pub mod common;
pub mod db;
pub mod hierarchy;
pub mod lens;

use crate::data::common::{Id, PublicInput, SubscribeHis, SubscribeInput, SubscribeMsg};
use common::Broker;
use rumqttc::v5::mqttbytes::{PubAck, SubAck};
use std::sync::Arc;

pub type AString = Arc<String>;

#[derive(Debug)]
pub enum AppEvent {
    RemoveSubscribeHis { broker_id: usize, his_id: Id },
    AddBroker,
    EditBroker,
    ConnectBroker,
    SaveBroker(usize),
    ReConnect(usize),
    // select brokers tab
    SelectTabs(usize),
    Connect(Broker),
    Subscribe(SubscribeInput, usize),
    ToUnSubscribe { broker_id: usize, pk_id: u16 },
    UnSubscribeIng(EventUnSubscribe),
    ConnectAckSuccess(usize),
    ConnectAckFail(usize, Arc<String>),
    Public(PublicInput, usize),
    ReceivePublic(usize, SubscribeMsg),
    PubAck(usize, PubAck),
    SubAck(usize, SubAck),
    UnSubAck(usize, u16),
    ClickBroker(usize),
    DbClickCheck(usize),
    // ClickSubscribeHis(usize, SubscribeHis),
    ClickSubscribeHis(SubscribeHis),
    DbClickCheckSubscribeHis(SubscribeHis),
    CloseBrokerTab(usize),
    CloseConnectionTab(usize),
    DeleteBroker,
    // e.g: delete broker; close tab; click button "disconnect"
    Disconnect(usize),
}
#[derive(Debug, Clone)]
pub struct EventUnSubscribe {
    pub broke_id: usize,
    pub subscribe_pk_id: u16,
    pub topic: String,
}
