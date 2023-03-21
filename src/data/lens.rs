use crate::data::common::{Broker, QoS};
use crate::data::common::{Msg, PublicInput, SubscribeHis, SubscribeInput, SubscribeTopic};
use crate::data::hierarchy::AppData;
use crate::data::AString;
use crate::util::consts::QosToString;
use druid::im::Vector;
use druid::Lens;

use std::sync::Arc;

pub struct BrokerSelectedOrZero;

impl druid::Lens<AppData, Broker> for BrokerSelectedOrZero {
    fn with<V, F: FnOnce(&Broker) -> V>(&self, data: &AppData, f: F) -> V {
        f(data.get_selected_broker_or_zero())
    }
    fn with_mut<V, F: FnOnce(&mut Broker) -> V>(&self, data: &mut AppData, f: F) -> V {
        f(data.get_mut_selected_broker_or_zero())
    }
}

pub struct BrokerId(pub usize);

impl druid::Lens<AppData, Broker> for BrokerId {
    fn with<V, F: FnOnce(&Broker) -> V>(&self, data: &AppData, f: F) -> V {
        f(data.find_broker_by_id(self.0).unwrap())
    }
    fn with_mut<V, F: FnOnce(&mut Broker) -> V>(&self, data: &mut AppData, f: F) -> V {
        f(data.find_mut_broker_by_id(self.0).unwrap())
    }
}

// pub struct BrokerSelected;
//
// impl druid::Lens<AppData, Broker> for BrokerSelected {
//     fn with<V, F: FnOnce(&Broker) -> V>(&self, data: &AppData, f: F) -> V {
//         f(match data.get_selected_broker_or_zero() {
//             Ok(broker) => broker,
//             Err(_) => unreachable!(),
//         })
//     }
//     fn with_mut<V, F: FnOnce(&mut Broker) -> V>(&self, data: &mut AppData, f: F) -> V {
//         f(match data.get_mut_selected_broker_or_zero() {
//             Ok(broker) => broker,
//             Err(e) => unreachable!(),
//         })
//     }
// }
// pub struct BrokerIndexLensVecSubscribeHis(pub usize);
//
// impl druid::Lens<AppData, Vector<SubscribeHis>> for BrokerIndexLensVecSubscribeHis {
//     fn with<V, F: FnOnce(&Vector<SubscribeHis>) -> V>(&self, data: &AppData, f: F) -> V {
//         f(match data.subscribe_hises.get(&self.0) {
//             Some(broker) => broker,
//             None => unreachable!(""),
//         })
//     }
//     fn with_mut<V, F: FnOnce(&mut Vector<SubscribeHis>) -> V>(
//         &self,
//         data: &mut AppData,
//         f: F,
//     ) -> V {
//         f(match data.subscribe_hises.get_mut(&self.0) {
//             Some(broker) => broker,
//             None => unreachable!(""),
//         })
//     }
// }
//
// pub struct LensSelectedSubscribeHis;
//
// impl druid::Lens<AppData, Vector<SubscribeHis>> for LensSelectedSubscribeHis {
//     fn with<V, F: FnOnce(&Vector<SubscribeHis>) -> V>(&self, data: &AppData, f: F) -> V {
//         if let Some(broker) = data.get_selected_broker() {
//             f(&broker.subscribe_hises)
//         } else {
//             let datas = Vector::new();
//             f(&datas)
//         }
//     }
//     fn with_mut<V, F: FnOnce(&mut Vector<SubscribeHis>) -> V>(
//         &self,
//         data: &mut AppData,
//         f: F,
//     ) -> V {
//         if let Some(broker) = data.get_selected_mut_broker() {
//             f(&mut broker.subscribe_hises)
//         } else {
//             error!("unreached");
//             let mut datas = Vector::new();
//             f(&mut datas)
//         }
//     }
// }
pub struct BrokerIndexLensVecSubscribeTopic(pub usize);

impl druid::Lens<AppData, Vector<SubscribeTopic>> for BrokerIndexLensVecSubscribeTopic {
    fn with<V, F: FnOnce(&Vector<SubscribeTopic>) -> V>(&self, data: &AppData, f: F) -> V {
        f(match data.find_broker_by_id(self.0) {
            Ok(broker) => &broker.subscribe_topics,
            Err(_e) => unreachable!(""),
        })
    }
    fn with_mut<V, F: FnOnce(&mut Vector<SubscribeTopic>) -> V>(
        &self,
        data: &mut AppData,
        f: F,
    ) -> V {
        f(match data.find_mut_broker_by_id(self.0) {
            Ok(broker) => &mut broker.subscribe_topics,
            Err(_e) => unreachable!(""),
        })
    }
}
pub struct BrokerIndexLensVecMsg(pub usize);

impl druid::Lens<AppData, Vector<Msg>> for BrokerIndexLensVecMsg {
    fn with<V, F: FnOnce(&Vector<Msg>) -> V>(&self, data: &AppData, f: F) -> V {
        f(match data.find_broker_by_id(self.0) {
            Ok(broker) => &broker.msgs,
            Err(_e) => unreachable!(""),
        })
    }
    fn with_mut<V, F: FnOnce(&mut Vector<Msg>) -> V>(&self, data: &mut AppData, f: F) -> V {
        f(match data.find_mut_broker_by_id(self.0) {
            Ok(broker) => &mut broker.msgs,
            Err(_e) => unreachable!(""),
        })
    }
}
pub struct BrokerIndexLensSubscribeInput(pub usize);

impl druid::Lens<AppData, SubscribeInput> for BrokerIndexLensSubscribeInput {
    fn with<V, F: FnOnce(&SubscribeInput) -> V>(&self, data: &AppData, f: F) -> V {
        f(match data.find_broker_by_id(self.0) {
            Ok(broker) => &broker.subscribe_input,
            Err(_e) => unreachable!(""),
        })
    }
    fn with_mut<V, F: FnOnce(&mut SubscribeInput) -> V>(&self, data: &mut AppData, f: F) -> V {
        f(match data.find_mut_broker_by_id(self.0) {
            Ok(broker) => &mut broker.subscribe_input,
            Err(_e) => unreachable!(""),
        })
    }
}

pub struct BrokerIndexLensPublicInput(pub usize);

impl druid::Lens<AppData, PublicInput> for BrokerIndexLensPublicInput {
    fn with<V, F: FnOnce(&PublicInput) -> V>(&self, data: &AppData, f: F) -> V {
        f(match data.find_broker_by_id(self.0) {
            Ok(broker) => &broker.public_input,
            Err(_e) => unreachable!(""),
        })
    }
    fn with_mut<V, F: FnOnce(&mut PublicInput) -> V>(&self, data: &mut AppData, f: F) -> V {
        f(match data.find_mut_broker_by_id(self.0) {
            Ok(broker) => &mut broker.public_input,
            Err(_e) => unreachable!(""),
        })
    }
}

// pub struct BrokerIndexLensTabStatus(pub usize);
//
// impl druid::Lens<AppData, TabStatus> for BrokerIndexLensTabStatus {
//     fn with<V, F: FnOnce(&TabStatus) -> V>(&self, data: &AppData, f: F) -> V {
//         f(match data.tab_statuses.get(&self.0) {
//             Some(broker) => broker,
//             None => unreachable!(""),
//         })
//     }
//     fn with_mut<V, F: FnOnce(&mut TabStatus) -> V>(&self, data: &mut AppData, f: F) -> V {
//         f(match data.tab_statuses.get_mut(&self.0) {
//             Some(broker) => broker,
//             None => unreachable!(""),
//         })
//     }
// }

// #[derive(Clone)]
// pub struct DbIndex {
//     pub data: AppData,
//     pub id: usize,
// }
// impl druid::Data for DbIndex {
//     fn same(&self, _other: &Self) -> bool {
//         let self_status = match self.data.tab_statuses.get(&self.id) {
//             Some(status) => status,
//             None => return false,
//         };
//         let other_status = match _other.data.tab_statuses.get(&self.id) {
//             Some(status) => status,
//             None => return false,
//         };
//         Data::same(self_status, other_status)
//     }
// }

pub struct Index(pub usize);

// impl druid::Lens<AppData, DbIndex> for Index {
//     fn with<V, F: FnOnce(&DbIndex) -> V>(&self, data: &AppData, f: F) -> V {
//         let db_index = DbIndex {
//             data: data.clone(),
//             id: self.0,
//         };
//         f(&db_index)
//     }
//     fn with_mut<V, F: FnOnce(&mut DbIndex) -> V>(&self, data: &mut AppData, f: F) -> V {
//         let mut db_index = DbIndex {
//             data: data.clone(),
//             id: self.0,
//         };
//         f(&mut db_index)
//     }
// }

pub struct BrokerStoredList;

impl druid::Lens<AppData, Vector<Broker>> for BrokerStoredList {
    fn with<V, F: FnOnce(&Vector<Broker>) -> V>(&self, data: &AppData, f: F) -> V {
        let broker_list: Vector<Broker> = data
            .brokers
            .iter()
            .filter(|x| x.stored)
            .map(|x| x.clone())
            .collect();
        f(&broker_list)
    }
    fn with_mut<V, F: FnOnce(&mut Vector<Broker>) -> V>(&self, data: &mut AppData, f: F) -> V {
        let mut broker_list: Vector<Broker> = data
            .brokers
            .iter()
            .filter(|x| x.stored)
            .map(|x| x.clone())
            .collect();
        f(&mut broker_list)
    }
}

pub struct MsgMsgLens;

impl Lens<Msg, AString> for MsgMsgLens {
    fn with<V, F: FnOnce(&AString) -> V>(&self, data: &Msg, f: F) -> V {
        f(match data {
            Msg::Public(msg) => &msg.msg,
            Msg::Subscribe(msg) => &msg.msg,
        })
    }

    fn with_mut<V, F: FnOnce(&mut AString) -> V>(&self, data: &mut Msg, f: F) -> V {
        f(match data {
            Msg::Public(msg) => &mut msg.msg,
            Msg::Subscribe(msg) => &mut msg.msg,
        })
    }
}

pub struct MsgTopicLens;

impl Lens<Msg, AString> for MsgTopicLens {
    fn with<V, F: FnOnce(&AString) -> V>(&self, data: &Msg, f: F) -> V {
        f(match data {
            Msg::Public(msg) => &msg.topic,
            Msg::Subscribe(msg) => &msg.topic,
        })
    }

    fn with_mut<V, F: FnOnce(&mut AString) -> V>(&self, data: &mut Msg, f: F) -> V {
        f(match data {
            Msg::Public(msg) => &mut msg.topic,
            Msg::Subscribe(msg) => &mut msg.topic,
        })
    }
}
pub struct MsgQosLens;
impl Lens<Msg, Arc<String>> for MsgQosLens {
    fn with<V, F: FnOnce(&Arc<String>) -> V>(&self, data: &Msg, f: F) -> V {
        let qos = match data {
            Msg::Public(msg) => &msg.qos,
            Msg::Subscribe(msg) => &msg.qos,
        };
        f(qos)
    }

    fn with_mut<V, F: FnOnce(&mut Arc<String>) -> V>(&self, data: &mut Msg, f: F) -> V {
        let qos = match data {
            Msg::Public(msg) => &mut msg.qos,
            Msg::Subscribe(msg) => &mut msg.qos,
        };
        f(qos)
    }
}
pub struct MsgPayloadTyLens;
impl Lens<Msg, AString> for MsgPayloadTyLens {
    fn with<V, F: FnOnce(&AString) -> V>(&self, data: &Msg, f: F) -> V {
        f(match data {
            Msg::Public(msg) => &msg.payload_ty,
            Msg::Subscribe(msg) => &msg.payload_ty,
        })
    }

    fn with_mut<V, F: FnOnce(&mut AString) -> V>(&self, data: &mut Msg, f: F) -> V {
        f(match data {
            Msg::Public(msg) => &mut msg.payload_ty,
            Msg::Subscribe(msg) => &mut msg.payload_ty,
        })
    }
}
pub struct MsgTimeLens;
impl Lens<Msg, AString> for MsgTimeLens {
    fn with<V, F: FnOnce(&AString) -> V>(&self, data: &Msg, f: F) -> V {
        f(match data {
            Msg::Public(msg) => &msg.time,
            Msg::Subscribe(msg) => &msg.time,
        })
    }

    fn with_mut<V, F: FnOnce(&mut AString) -> V>(&self, data: &mut Msg, f: F) -> V {
        f(match data {
            Msg::Public(msg) => &mut msg.time,
            Msg::Subscribe(msg) => &mut msg.time,
        })
    }
}

pub struct PortLens;

impl Lens<Broker, Option<u16>> for PortLens {
    fn with<V, F: FnOnce(&Option<u16>) -> V>(&self, data: &Broker, f: F) -> V {
        f(&data.port)
    }

    fn with_mut<V, F: FnOnce(&mut Option<u16>) -> V>(&self, data: &mut Broker, f: F) -> V {
        f(&mut data.port)
    }
}

pub struct LensSubscribeHisQoS;

impl Lens<SubscribeHis, Arc<String>> for LensSubscribeHisQoS {
    fn with<V, F: FnOnce(&Arc<String>) -> V>(&self, data: &SubscribeHis, f: F) -> V {
        f(&data.qos.qos_to_string())
    }

    fn with_mut<V, F: FnOnce(&mut Arc<String>) -> V>(&self, data: &mut SubscribeHis, f: F) -> V {
        f(&mut data.qos.qos_to_string())
    }
}

pub struct SubscribeTopicPayloadLens;
impl Lens<SubscribeTopic, Arc<String>> for SubscribeTopicPayloadLens {
    fn with<V, F: FnOnce(&Arc<String>) -> V>(&self, data: &SubscribeTopic, f: F) -> V {
        f(&data.payload_ty.to_arc_string())
    }

    fn with_mut<V, F: FnOnce(&mut Arc<String>) -> V>(&self, data: &mut SubscribeTopic, f: F) -> V {
        f(&mut data.payload_ty.to_arc_string())
    }
}

pub struct SubscribeHisPayloadLens;
impl Lens<SubscribeHis, Arc<String>> for SubscribeHisPayloadLens {
    fn with<V, F: FnOnce(&Arc<String>) -> V>(&self, data: &SubscribeHis, f: F) -> V {
        f(&data.payload_ty.to_arc_string())
    }

    fn with_mut<V, F: FnOnce(&mut Arc<String>) -> V>(&self, data: &mut SubscribeHis, f: F) -> V {
        f(&mut data.payload_ty.to_arc_string())
    }
}

pub struct LensQoSAString;
impl Lens<QoS, Arc<String>> for LensQoSAString {
    fn with<V, F: FnOnce(&Arc<String>) -> V>(&self, data: &QoS, f: F) -> V {
        f(&data.qos_to_string())
    }

    fn with_mut<V, F: FnOnce(&mut Arc<String>) -> V>(&self, data: &mut QoS, f: F) -> V {
        f(&mut data.qos_to_string())
    }
}
