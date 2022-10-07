use crate::data::common::Broker;
use crate::data::{AString, AppEvent};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::Sender;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DbKey {
    Broker(usize),
    SubscribeHis(usize),
}

impl DbKey {
    pub fn broker_key(id: usize) -> Self {
        Self::Broker(id)
    }
    pub fn subscribe_his_key(id: usize) -> Self {
        Self::SubscribeHis(id)
    }
    pub fn as_bytes(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec(self)?)
    }
}

// #[derive(Debug, Clone, Serialize, Deserialize, FromBytes, AsBytes)]
// #[repr(C)]
// pub struct BrokerKey {
//     pub id: usize,
// }
// #[derive(Debug, Clone, Serialize, Deserialize, FromBytes, AsBytes)]
// #[repr(C)]
// pub struct SubscribeHisesKey {
//     pub id: usize,
// }
// impl From<usize> for SubscribeHisesKey {
//     fn from(id: usize) -> Self {
//         Self { id }
//     }
// }
// impl From<usize> for BrokerKey {
//     fn from(id: usize) -> Self {
//         Self { id }
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokerDB {
    pub id: usize,
    pub client_id: AString,
    pub name: AString,
    pub addr: AString,
    pub port: u16,
    pub params: AString,
    pub use_credentials: bool,
    pub user_name: AString,
    pub password: AString,
}

impl BrokerDB {
    pub fn to_broker(self, tx: Sender<AppEvent>) -> Broker {
        let Self {
            id,
            client_id,
            name,
            addr,
            port,
            params,
            use_credentials,
            user_name,
            password,
        } = self;
        Broker {
            id,
            client_id,
            name,
            addr,
            port,
            params,
            use_credentials,
            user_name,
            password,
            stored: true,
            tx,
            selected: false,
        }
    }
}

// #[cfg(test)]
// mod test {
//     use crate::data::db::SubscribeHisesKey;
//     use core::mem::size_of;
//     use core::slice;
//
//     #[test]
//     pub fn test_ptr() {
//         let val: SubscribeHisesKey = 16usize.into();
//         assert_eq!(size_of::<SubscribeHisesKey>(), size_of::<usize>());
//         let u8_slice = val.as_ref();
//         let mut data = [0u8; size_of::<usize>()];
//         for (index, u8_tmp) in u8_slice.iter().enumerate() {
//             data[index] = *u8_tmp;
//         }
//         assert_eq!(usize::from_ne_bytes(data), 16);
//     }
// }
