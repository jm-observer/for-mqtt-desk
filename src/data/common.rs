use crate::data::AString;
use druid::{Data, Lens};
use serde::{Deserialize, Serialize};

#[derive(Data, Clone, Debug, Eq, PartialEq)]
pub struct SubscribeTopic {
    pub topic: AString,
    pub qos: Qos,
    pub status: SubscribeStatus,
}
#[derive(Data, Debug, Clone, Eq, PartialEq, Lens, Deserialize, Serialize)]
pub struct SubscribeHis {
    pub(crate) topic: AString,
    pub(crate) qos: Qos,
}

#[derive(Debug, Data, Clone, Eq, PartialEq)]
pub enum Msg {
    Public(PublicMsg),
    Subscribe(SubscribeMsg),
}

#[derive(Debug, Data, Clone, Eq, PartialEq, Lens)]
pub struct PublicMsg {
    pub topic: AString,
    pub msg: AString,
    pub qos: Qos,
    pub status: PublicStatus,
}
#[derive(Debug, Data, Clone, Eq, PartialEq)]
pub enum PublicStatus {
    Ing,
    Success,
}

#[derive(Debug, Data, Clone, Eq, PartialEq, Lens)]
pub struct PublicMsgInput {
    pub topic: AString,
    pub msg: AString,
    pub qos: AString,
}

#[derive(Data, Clone, Debug, Eq, PartialEq)]
pub struct SubscribeMsg {
    pub topic: AString,
    pub msg: AString,
    pub qos: Qos,
}

#[derive(Data, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Qos {
    Qos0,
    Qos1,
    Qos2,
}
#[derive(Data, Debug, Clone, Eq, PartialEq, Lens)]
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
