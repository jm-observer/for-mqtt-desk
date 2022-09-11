use anyhow::Result;
use sled::{Config, Db};
use std::sync::mpsc::Sender;
use std::sync::Arc;

use crate::data::db::{Broker, SubscribeHises, SubscribeHisesKey};
use crate::data::hierarchy::AppData;
use crate::data::AppEvent;
use druid::im::{HashMap, Vector};

#[derive(Clone, Debug)]
pub struct ArcDb {
    pub db: Arc<Db>,
    pub tx: Sender<AppEvent>,
}

const BROKERS: &[u8; 7] = b"brokers";
const KEYS: &[u8; 4] = b"keys";
impl ArcDb {
    pub fn init_db(tx: Sender<AppEvent>) -> Result<Self> {
        let config = Config::new();
        Ok(ArcDb {
            db: Arc::new(config.open()?),
            tx,
        })
    }

    pub fn read_app_data(&self) -> Result<AppData> {
        let (brokers, subscribe_hises) = if let Some(val) = self.db.remove(BROKERS)? {
            let db_brokers: Vector<Broker> = serde_json::from_slice(&val)?;
            let mut brokers = HashMap::new();
            let mut subscribe_hises = HashMap::new();
            for (index, mut broker) in db_brokers.into_iter().enumerate() {
                let old_id = broker.id;
                broker.id = index;
                brokers.insert(index, broker.into());
                let hises = if let Some(val) = self.db.remove(SubscribeHisesKey::from(old_id))? {
                    let mut hises: SubscribeHises = serde_json::from_slice(&val)?;
                    hises.id = index;
                    hises
                } else {
                    SubscribeHises::from(index)
                };
                subscribe_hises.insert(index, hises.into());
            }
            (brokers, subscribe_hises)
        } else {
            (HashMap::new(), HashMap::new())
        };
        Ok(AppData {
            brokers,
            broker_tabs: Default::default(),
            subscribe_hises,
            subscribe_topics: Default::default(),
            msgs: Default::default(),
            subscribe_ing: Default::default(),
            public_ing: Default::default(),
            db: self.clone(),
        })
    }
}
