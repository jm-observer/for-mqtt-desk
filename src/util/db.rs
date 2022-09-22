use anyhow::Result;
use sled::{Config, Db};
use std::sync::mpsc::Sender;
use std::sync::Arc;

use crate::data::common::{Broker, SubscribeHis};
use crate::data::db::{BrokerDB, BrokerKey, SubscribeHisesKey};
use crate::data::hierarchy::AppData;
use crate::data::AppEvent;
use druid::im::{HashMap, Vector};
use log::warn;
use zerocopy::AsBytes;

#[derive(Clone, Debug)]
pub struct ArcDb {
    pub index: usize,
    pub db: Db,
    pub tx: Sender<AppEvent>,
    pub ids: Vector<usize>,
}

const BROKERS: &[u8; 7] = b"brokers";
impl ArcDb {
    pub fn init_db(tx: Sender<AppEvent>) -> Result<Self> {
        let config = Config::new().path("./resources/db");
        Ok(ArcDb {
            index: 0,
            db: config.open()?,
            tx,
            ids: Default::default(),
        })
    }

    pub fn read_app_data(&mut self) -> Result<AppData> {
        let (db_brokers, subscribe_hises) = if let Some(val) = self.db.remove(BROKERS)? {
            let db_brokers_ids: Vector<usize> = serde_json::from_slice(&val)?;
            let mut brokers = Vector::new();
            let mut subscribe_hises = HashMap::new();
            self.index = db_brokers_ids.len();
            for (index, id) in db_brokers_ids.into_iter().enumerate() {
                if let Some(val) = self.db.remove(BrokerKey::from(id).as_bytes())? {
                    let mut broker: BrokerDB = serde_json::from_slice(&val)?;
                    broker.id = index;
                    let hises = if let Some(val) =
                        self.db.remove(SubscribeHisesKey::from(id).as_bytes())?
                    {
                        let hises: Vector<SubscribeHis> = serde_json::from_slice(&val)?;
                        hises
                    } else {
                        Vector::new()
                    };
                    brokers.push_back(broker);
                    subscribe_hises.insert(index, hises);
                    self.ids.push_back(index);
                } else {
                    warn!("can't find id: {}", id);
                };
            }
            (brokers, subscribe_hises)
        } else {
            (Vector::new(), HashMap::new())
        };
        let mut brokers = Vector::new();
        {
            self.db.insert(BROKERS, serde_json::to_vec(&self.ids)?)?;
            for (index, his_tmp) in subscribe_hises.iter() {
                self.db.insert(
                    SubscribeHisesKey::from(*index).as_bytes(),
                    serde_json::to_vec(&his_tmp)?,
                )?;
            }

            for tmp_broker in db_brokers.into_iter() {
                self.db.insert(
                    BrokerKey::from(tmp_broker.id).as_bytes(),
                    serde_json::to_vec(&tmp_broker)?,
                )?;
                brokers.push_back(tmp_broker.to_broker(self.tx.clone()));
            }
        }
        Ok(AppData {
            broker_selected: 0,
            brokers,
            broker_tabs: Default::default(),
            tab_statuses: Default::default(),
            subscribe_hises,
            subscribe_topics: Default::default(),
            msgs: Default::default(),
            subscribe_ing: Default::default(),
            public_ing: Default::default(),
            db: self.clone(),
            mqtt_clients: Default::default(),
        })
    }

    pub fn new_broker(&mut self) -> Broker {
        let id = self.index;
        self.index += 1;
        Broker {
            id,
            client_id: Arc::new("".to_string()),
            name: Arc::new("".to_string()),
            addr: Arc::new("broker-cn.emqx.io".to_string()),
            port: Arc::new("1883".to_string()),
            params: Arc::new(OPTION.to_string()),
            use_credentials: false,
            user_name: Arc::new("".to_string()),
            password: Arc::new("".to_string()),
            stored: false,
            tx: self.tx.clone(),
            selected: false,
        }
    }

    pub fn save_broker(&mut self, id: usize, broker: &Broker) -> Result<()> {
        if self.ids.iter().find(|x| **x == id).is_none() {
            self.ids.push_back(id);
            self.db.insert(BROKERS, serde_json::to_vec(&self.ids)?)?;
        }
        self.db.insert(
            BrokerKey::from(id).as_bytes(),
            serde_json::to_vec(&broker.clone_to_db())?,
        )?;
        Ok(())
    }
}

const OPTION: &str = r#"{
	"keep_alive": 60,
	"clean_session": true,
	"max_incoming_packet_size": 10240,
	"max_outgoing_packet_size": 10240,
	"inflight": 100,
	"conn_timeout": 5
}
        "#;

#[cfg(test)]
mod test {
    use crate::data::common::Broker;
    use crate::util::db::BROKERS;
    use druid::im::vector;
    use sled::Config;
    use std::sync::Arc;

    #[test]
    fn insert_broker() {
        let db = Config::new().path("./resource/db").open().unwrap();
        let broker = vector![Broker {
            id: 0,
            client_id: Arc::new("id_5678".to_string()),
            name: Arc::new("emq".to_string()),
            addr: Arc::new("192.168.199.188".to_string()),
            port: Arc::new("1883".to_string()),
            params: Arc::new("{abc,jiofewki, iowoere}".to_string()),
        }];
        let broker = serde_json::to_vec(&broker).unwrap();
        db.insert(BROKERS, broker).unwrap();
    }
}
