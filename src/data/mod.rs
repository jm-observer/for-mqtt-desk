pub mod common;
pub mod db;
pub mod hierarchy;
pub mod lens;

use crate::data::common::{PublicInput, SubscribeHis, SubscribeInput, SubscribeMsg};
use common::Broker;
use rumqttc::v5::mqttbytes::{PubAck, SubAck};
use std::sync::Arc;

pub type AString = Arc<String>;

#[derive(Debug)]
pub enum AppEvent {
    AddBroker,
    SaveBroker(usize),
    ReConnect(usize),
    // select brokers tab
    SelectTabs(usize),
    Connect(Broker),
    Subscribe(SubscribeInput, usize),
    ConnectAckSuccess(usize),
    ConnectAckFail(usize, Arc<String>),
    Public(PublicInput, usize),
    ReceivePublic(usize, SubscribeMsg),
    PubAck(usize, PubAck),
    SubAck(usize, SubAck),
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
