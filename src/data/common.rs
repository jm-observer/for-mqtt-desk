mod impls;

use crate::data::AString;
use crate::util::db::ArcDb;
use druid::{Data, Lens};
use serde::{Deserialize, Serialize};

#[derive(Data, Clone, Debug)]
pub struct SubscribeTopic {
    pub pkid: u16,
    #[data(ignore)]
    pub topic: AString,
    #[data(ignore)]
    pub qos: QoS,
    #[data(eq)]
    pub status: SubscribeStatus,
}
#[derive(Debug, Clone, Eq, PartialEq, Lens, Deserialize, Serialize)]
pub struct SubscribeHis {
    pub(crate) topic: AString,
    pub(crate) qos: QoS,
}

#[derive(Debug, Data, Clone, Eq, PartialEq)]
pub enum Msg {
    Public(PublicMsg),
    Subscribe(SubscribeMsg),
}

#[derive(Debug, Data, Clone, Eq, PartialEq, Lens)]
pub struct PublicMsg {
    pub pkid: u16,
    pub topic: AString,
    pub msg: AString,
    pub qos: QoS,
    pub status: PublicStatus,
}
#[derive(Debug, Data, Clone, Eq, PartialEq)]
pub enum PublicStatus {
    Ing,
    Success,
}

#[derive(Debug, Data, Clone, Eq, PartialEq, Lens, Default)]
pub struct PublicInput {
    pub topic: AString,
    pub msg: AString,
    pub qos: AString,
    pub retain: bool,
}

#[derive(Data, Clone, Debug, Eq, PartialEq)]
pub struct SubscribeMsg {
    pub pkid: u16,
    pub topic: AString,
    pub msg: AString,
    pub qos: QoS,
}

// #[derive(Data, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
// pub enum Qos {
//     Qos0,
//     Qos1,
//     Qos2,
// }
#[derive(Data, Debug, Clone, Eq, PartialEq, Lens, Default)]
pub struct SubscribeInput {
    pub(crate) topic: AString,
    pub(crate) qos: AString,
}
#[derive(Data, Debug, Clone, Eq, PartialEq)]
pub enum SubscribeStatus {
    Ing,
    Success,
    Fail,
}

#[derive(Debug, Clone, Data)]
pub struct TabStatus {
    pub(crate) id: usize,
    pub(crate) try_connect: bool,
    pub(crate) connected: bool,
    #[data(ignore)]
    pub db: ArcDb,
}

#[derive(Data, Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum TabKind {
    Connection,
    Broker,
}
#[derive(Debug, Data, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub enum QoS {
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactlyOnce = 2,
}
