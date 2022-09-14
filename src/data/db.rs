use crate::data::AString;
use druid::{Data, Lens};
use serde::{Deserialize, Serialize};
use std::mem::size_of;
use std::slice;

#[derive(Debug, Clone, Data, Lens, Serialize, Deserialize)]
pub struct Broker {
    pub id: usize,
    pub client_id: AString,
    pub name: AString,
    pub addr: AString,
    pub port: AString,
    pub params: AString,
    pub use_credentials: bool,
    pub user_name: AString,
    pub password: AString,
    // #[serde(skip)]
    // #[data(ignore)]
    // #[lens(ignore)]
    // pub tx: Sender<AppEvent>,
}

// #[derive(Debug, Data, Clone, Eq, PartialEq, Default, Lens, Serialize, Deserialize)]
// pub struct SubscribeHises {
//     pub id: usize,
//     pub topics: Vector<SubscribeHis>,
// }
#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(transparent)]
pub struct SubscribeHisesKey {
    pub id: usize,
}

impl AsRef<[u8]> for SubscribeHisesKey {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                (self as *const SubscribeHisesKey) as *const u8,
                size_of::<usize>(),
            )
        }
    }
}
impl From<usize> for SubscribeHisesKey {
    fn from(id: usize) -> Self {
        Self { id }
    }
}

#[cfg(test)]
mod test {
    use crate::data::db::SubscribeHisesKey;
    use core::mem::size_of;
    use core::slice;

    #[test]
    pub fn test_ptr() {
        let val: SubscribeHisesKey = 16usize.into();
        assert_eq!(size_of::<SubscribeHisesKey>(), size_of::<usize>());
        let u8_slice = val.as_ref();
        let mut data = [0u8; size_of::<usize>()];
        for (index, u8_tmp) in u8_slice.iter().enumerate() {
            data[index] = *u8_tmp;
        }
        assert_eq!(usize::from_ne_bytes(data), 16);
    }
}
