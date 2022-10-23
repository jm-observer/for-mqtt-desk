mod impls;

use crate::data::db::BrokerDB;
use crate::data::{AString, AppEvent};
use druid::{Data, Lens};
use log::debug;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::mpsc::Sender;

static U32: AtomicU32 = AtomicU32::new(0);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Data)]
pub struct Id(u32);

impl Default for Id {
    fn default() -> Self {
        Self(U32.fetch_add(1, Ordering::Release))
    }
}

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
#[derive(Debug, Clone, Eq, Lens, Deserialize, Serialize, Data)]
pub struct SubscribeHis {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) broker_id: usize,
    #[serde(skip)]
    pub(crate) selected: bool,
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
    pub qos: QoS,
    pub retain: bool,
}

#[derive(Data, Clone, Debug, Eq, PartialEq)]
pub struct SubscribeMsg {
    pub pkid: u16,
    pub topic: AString,
    pub msg: AString,
    pub qos: QoS,
}

#[derive(Data, Debug, Clone, Eq, PartialEq, Lens)]
pub struct SubscribeInput {
    pub broker_id: usize,
    pub(crate) topic: AString,
    pub(crate) qos: QoS,
}
#[derive(Data, Debug, Clone, Eq, PartialEq)]
pub enum SubscribeStatus {
    SubscribeIng,
    SubscribeSuccess,
    SubscribeFail,
    UnSubscribeIng,
}

#[derive(Debug, Clone, Data)]
pub struct TabStatus {
    pub(crate) id: usize,
    pub(crate) try_connect: bool,
    pub(crate) connected: bool,
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
impl Default for QoS {
    fn default() -> Self {
        QoS::AtMostOnce
    }
}
impl ToString for QoS {
    fn to_string(&self) -> String {
        match self {
            QoS::AtMostOnce => "0".to_string(),
            QoS::AtLeastOnce => "1".to_string(),
            QoS::ExactlyOnce => "2".to_string(),
        }
    }
}

#[derive(Debug, Clone, Data, Lens)]
pub struct Broker {
    pub id: usize,
    pub client_id: AString,
    pub name: AString,
    pub addr: AString,
    pub port: u16,
    pub params: AString,
    pub use_credentials: bool,
    pub user_name: AString,
    pub password: AString,
    #[data(ignore)]
    #[lens(ignore)]
    pub stored: bool,
    #[data(ignore)]
    #[lens(ignore)]
    pub tx: Sender<AppEvent>,
    #[lens(ignore)]
    pub selected: bool,
}

impl Broker {
    pub fn clone_to_db(&self) -> BrokerDB {
        BrokerDB {
            id: self.id,
            client_id: self.client_id.clone(),
            name: self.name.clone(),
            addr: self.addr.clone(),
            port: self.port.clone(),
            params: self.params.clone(),
            use_credentials: self.use_credentials,
            user_name: self.user_name.clone(),
            password: self.password.clone(),
        }
    }
}

impl PartialEq for SubscribeHis {
    fn eq(&self, other: &Self) -> bool {
        self.broker_id == other.broker_id && self.topic == other.topic && self.qos == other.qos
    }
}
