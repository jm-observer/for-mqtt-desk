pub mod common;
pub mod db;
pub mod hierarchy;
pub mod lens;

use crate::data::common::{PublicInput, SubscribeInput, SubscribeMsg};
use common::Broker;
use rumqttc::v5::mqttbytes::{PubAck, SubAck};
use std::sync::Arc;

pub type AString = Arc<String>;

#[derive(Debug)]
pub enum AppEvent {
    AddBroker,
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
    CloseBrokerTab(usize),
    CloseConnectionTab(usize),
    DeleteBroker,
    Disconnect(usize),
}
