mod impls;

use crate::data::db::BrokerDB;
use crate::data::{AString, AppEvent};
use crate::util::consts::{TY_HEX, TY_JSON, TY_TEXT};
use bytes::Bytes;
use crossbeam_channel::Sender;
use druid::{Data, Lens};
use log::{debug, error, warn};
use pretty_hex::simple_hex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

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
    pub trace_id: u32,
    #[data(ignore)]
    pub topic: AString,
    #[data(ignore)]
    pub qos: AString,
    pub status: SubscribeStatus,
    pub payload_ty: PayloadTy,
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
    pub payload_ty: PayloadTy,
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
    pub trace_id: u32,
    pub topic: AString,
    pub msg: AString,
    pub qos: AString,
    pub status: PublicStatus,
    pub payload_ty: AString,
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
    pub payload_ty: PayloadTy,
}

#[derive(Data, Clone, Debug, Eq, PartialEq)]
pub struct SubscribeMsg {
    pub topic: AString,
    pub msg: AString,
    pub qos: AString,
    pub payload_ty: AString,
}

#[derive(Data, Debug, Clone, Eq, PartialEq, Lens)]
pub struct SubscribeInput {
    pub broker_id: usize,
    pub(crate) topic: AString,
    pub(crate) qos: QoS,
    pub(crate) payload_ty: PayloadTy,
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
impl QoS {
    pub fn to_u8(&self) -> u8 {
        match self {
            QoS::AtMostOnce => 0,
            QoS::AtLeastOnce => 1,
            QoS::ExactlyOnce => 2,
        }
    }
}
#[derive(Data, Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
/// 消息的格式：普通字符串、json字符串、hex
pub enum PayloadTy {
    Text,
    Json,
    Hex,
}

impl PayloadTy {
    pub fn to_arc_string(&self) -> Arc<String> {
        match self {
            PayloadTy::Text => TY_TEXT.clone(),
            PayloadTy::Json => TY_JSON.clone(),
            PayloadTy::Hex => TY_HEX.clone(),
        }
    }
    pub fn format(&self, data: Arc<Bytes>) -> String {
        match self {
            PayloadTy::Text => String::from_utf8_lossy(data.as_ref()).to_string(),
            PayloadTy::Json => match to_pretty_json(&data) {
                Ok(json_str) => json_str,
                Err(err) => {
                    warn!("format to json error: {}", err.to_string());
                    String::from_utf8_lossy(data.as_ref()).to_string()
                }
            },
            PayloadTy::Hex => simple_hex(data.as_ref()),
        }
    }
}

fn to_pretty_json(data: &Arc<Bytes>) -> anyhow::Result<String> {
    let json = serde_json::from_slice::<Value>(data.as_ref())?;
    Ok(serde_json::to_string_pretty(&json)?)
}

impl Default for PayloadTy {
    fn default() -> Self {
        Self::Text
    }
}
