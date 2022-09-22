mod impls;

use crate::data::AString;
use crate::util::db::ArcDb;
use druid::{Data, Lens};
use serde::{Deserialize, Serialize};

#[derive(Data, Clone, Debug, Lens)]
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

impl Msg {
    pub fn is_public(&self) -> bool {
        if let Msg::Public(_) = self {
            return true;
        }
        false
    }
    pub fn is_sucess(&self) -> bool {
        if let Msg::Public(msg) = self {
            if msg.status == PublicStatus::Success {
                true
            } else {
                false
            }
        } else {
            true
        }
    }
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
#[repr(u8)]
pub enum QoS {
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactlyOnce = 2,
}
