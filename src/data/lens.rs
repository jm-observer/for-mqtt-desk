use crate::data::common::{
    Msg, PublicMsgInput, SubscribeHis, SubscribeInput, SubscribeTopic, TabStatus,
};
use crate::data::db::Broker;
use crate::data::hierarchy::AppData;
use druid::im::Vector;
use std::sync::Arc;

pub struct BrokerIndex(pub usize);

impl druid::Lens<AppData, Arc<Broker>> for BrokerIndex {
    fn with<V, F: FnOnce(&Arc<Broker>) -> V>(&self, data: &AppData, f: F) -> V {
        f(match data.brokers.iter().find(|x| x.id == self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
    fn with_mut<V, F: FnOnce(&mut Arc<Broker>) -> V>(&self, data: &mut AppData, f: F) -> V {
        f(match data.brokers.iter_mut().find(|x| x.id == self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
}

impl druid::Lens<AppData, Vector<Arc<SubscribeHis>>> for BrokerIndex {
    fn with<V, F: FnOnce(&Vector<Arc<SubscribeHis>>) -> V>(&self, data: &AppData, f: F) -> V {
        f(match data.subscribe_hises.get(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
    fn with_mut<V, F: FnOnce(&mut Vector<Arc<SubscribeHis>>) -> V>(
        &self,
        data: &mut AppData,
        f: F,
    ) -> V {
        f(match data.subscribe_hises.get_mut(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
}

impl druid::Lens<AppData, Vector<Arc<SubscribeTopic>>> for BrokerIndex {
    fn with<V, F: FnOnce(&Vector<Arc<SubscribeTopic>>) -> V>(&self, data: &AppData, f: F) -> V {
        f(match data.subscribe_topics.get(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
    fn with_mut<V, F: FnOnce(&mut Vector<Arc<SubscribeTopic>>) -> V>(
        &self,
        data: &mut AppData,
        f: F,
    ) -> V {
        f(match data.subscribe_topics.get_mut(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
}

impl druid::Lens<AppData, Vector<Arc<Msg>>> for BrokerIndex {
    fn with<V, F: FnOnce(&Vector<Arc<Msg>>) -> V>(&self, data: &AppData, f: F) -> V {
        f(match data.msgs.get(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
    fn with_mut<V, F: FnOnce(&mut Vector<Arc<Msg>>) -> V>(&self, data: &mut AppData, f: F) -> V {
        f(match data.msgs.get_mut(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
}

impl druid::Lens<AppData, Arc<SubscribeInput>> for BrokerIndex {
    fn with<V, F: FnOnce(&Arc<SubscribeInput>) -> V>(&self, data: &AppData, f: F) -> V {
        f(match data.subscribe_ing.get(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
    fn with_mut<V, F: FnOnce(&mut Arc<SubscribeInput>) -> V>(&self, data: &mut AppData, f: F) -> V {
        f(match data.subscribe_ing.get_mut(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
}

impl druid::Lens<AppData, Arc<PublicMsgInput>> for BrokerIndex {
    fn with<V, F: FnOnce(&Arc<PublicMsgInput>) -> V>(&self, data: &AppData, f: F) -> V {
        f(match data.public_ing.get(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
    fn with_mut<V, F: FnOnce(&mut Arc<PublicMsgInput>) -> V>(&self, data: &mut AppData, f: F) -> V {
        f(match data.public_ing.get_mut(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
}

impl druid::Lens<AppData, TabStatus> for BrokerIndex {
    fn with<V, F: FnOnce(&TabStatus) -> V>(&self, data: &AppData, f: F) -> V {
        f(match data.tab_statuses.get(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
    fn with_mut<V, F: FnOnce(&mut TabStatus) -> V>(&self, data: &mut AppData, f: F) -> V {
        f(match data.tab_statuses.get_mut(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
}

// pub struct DbIndex {
//     db: ArcDb,
//     index: usize,
// }
//
// pub struct Index(pub usize);
//
// impl druid::Lens<AppData, DbIndex> for Index {
//     fn with<V, F: FnOnce(&DbIndex) -> V>(&self, data: &AppData, f: F) -> V {
//         let db_index = DbIndex {
//             db: data.db.clone(),
//             index: self.0,
//         };
//         f(&db_index)
//     }
//     fn with_mut<V, F: FnOnce(&mut DbIndex) -> V>(&self, data: &mut AppData, f: F) -> V {
//         let mut db_index = DbIndex {
//             db: data.db.clone(),
//             index: self.0,
//         };
//         f(&mut db_index)
//     }
// }
