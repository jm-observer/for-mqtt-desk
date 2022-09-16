use crate::data::common::{
    Msg, PublicInput, SubscribeHis, SubscribeInput, SubscribeTopic, TabStatus,
};
use crate::data::db::Broker;
use crate::data::hierarchy::AppData;
use druid::im::Vector;

pub struct BrokerIndex(pub usize);

impl druid::Lens<AppData, Broker> for BrokerIndex {
    fn with<V, F: FnOnce(&Broker) -> V>(&self, data: &AppData, f: F) -> V {
        f(match data.find_broker(self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
    fn with_mut<V, F: FnOnce(&mut Broker) -> V>(&self, data: &mut AppData, f: F) -> V {
        f(match data.brokers.iter_mut().find(|x| x.id == self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
}

impl druid::Lens<AppData, Vector<SubscribeHis>> for BrokerIndex {
    fn with<V, F: FnOnce(&Vector<SubscribeHis>) -> V>(&self, data: &AppData, f: F) -> V {
        f(match data.subscribe_hises.get(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
    fn with_mut<V, F: FnOnce(&mut Vector<SubscribeHis>) -> V>(
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

impl druid::Lens<AppData, Vector<SubscribeTopic>> for BrokerIndex {
    fn with<V, F: FnOnce(&Vector<SubscribeTopic>) -> V>(&self, data: &AppData, f: F) -> V {
        f(match data.subscribe_topics.get(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
    fn with_mut<V, F: FnOnce(&mut Vector<SubscribeTopic>) -> V>(
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

impl druid::Lens<AppData, Vector<Msg>> for BrokerIndex {
    fn with<V, F: FnOnce(&Vector<Msg>) -> V>(&self, data: &AppData, f: F) -> V {
        f(match data.msgs.get(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
    fn with_mut<V, F: FnOnce(&mut Vector<Msg>) -> V>(&self, data: &mut AppData, f: F) -> V {
        f(match data.msgs.get_mut(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
}

impl druid::Lens<AppData, SubscribeInput> for BrokerIndex {
    fn with<V, F: FnOnce(&SubscribeInput) -> V>(&self, data: &AppData, f: F) -> V {
        f(match data.subscribe_ing.get(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
    fn with_mut<V, F: FnOnce(&mut SubscribeInput) -> V>(&self, data: &mut AppData, f: F) -> V {
        f(match data.subscribe_ing.get_mut(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
}

impl druid::Lens<AppData, PublicInput> for BrokerIndex {
    fn with<V, F: FnOnce(&PublicInput) -> V>(&self, data: &AppData, f: F) -> V {
        f(match data.public_ing.get(&self.0) {
            Some(broker) => broker,
            None => unreachable!(""),
        })
    }
    fn with_mut<V, F: FnOnce(&mut PublicInput) -> V>(&self, data: &mut AppData, f: F) -> V {
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

#[derive(Clone)]
pub struct DbIndex {
    pub data: AppData,
    pub index: usize,
}
impl druid::Data for DbIndex {
    fn same(&self, _other: &Self) -> bool {
        true
    }
}

pub struct Index(pub usize);

impl druid::Lens<AppData, DbIndex> for Index {
    fn with<V, F: FnOnce(&DbIndex) -> V>(&self, data: &AppData, f: F) -> V {
        let db_index = DbIndex {
            data: data.clone(),
            index: self.0,
        };
        f(&db_index)
    }
    fn with_mut<V, F: FnOnce(&mut DbIndex) -> V>(&self, data: &mut AppData, f: F) -> V {
        let mut db_index = DbIndex {
            data: data.clone(),
            index: self.0,
        };
        f(&mut db_index)
    }
}

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
