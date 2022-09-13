use anyhow::Result;
use sled::{Config, Db};
use std::sync::mpsc::Sender;
use std::sync::Arc;

use crate::data::common::SubscribeHis;
use crate::data::db::{Broker, SubscribeHisesKey};
use crate::data::hierarchy::AppData;
use crate::data::AppEvent;
use druid::im::{HashMap, Vector};

#[derive(Clone, Debug)]
pub struct ArcDb {
    pub index: usize,
    pub db: Db,
    pub tx: Sender<AppEvent>,
}

const BROKERS: &[u8; 7] = b"brokers";
impl ArcDb {
    pub fn init_db(tx: Sender<AppEvent>) -> Result<Self> {
        let config = Config::new().path("./resource/db");
        Ok(ArcDb {
            index: 0,
            db: config.open()?,
            tx,
        })
    }

    pub fn read_app_data(&mut self) -> Result<AppData> {
        let (brokers_tmp, subscribe_hises_tmp) = if let Some(val) = self.db.remove(BROKERS)? {
            let db_brokers: Vector<Broker> = serde_json::from_slice(&val)?;
            let mut brokers = Vector::new();
            let mut subscribe_hises = HashMap::new();
            for (index, mut broker) in db_brokers.into_iter().enumerate() {
                let old_id = broker.id;
                broker.id = index;
                brokers.push_back(broker);
                let hises = if let Some(val) = self.db.remove(SubscribeHisesKey::from(old_id))? {
                    let hises: Vector<SubscribeHis> = serde_json::from_slice(&val)?;
                    hises
                } else {
                    Vector::new()
                };
                subscribe_hises.insert(index, hises);
            }
            (brokers, subscribe_hises)
        } else {
            (Vector::new(), HashMap::new())
        };
        self.db.insert(BROKERS, serde_json::to_vec(&brokers_tmp)?)?;
        self.index = brokers_tmp.len();
        let mut subscribe_hises = HashMap::new();
        for (index, his_tmp) in subscribe_hises_tmp.into_iter() {
            self.db.insert(
                SubscribeHisesKey::from(index),
                serde_json::to_vec(&his_tmp)?,
            )?;
            let mut his = Vector::new();
            for item in his_tmp {
                his.push_back(item.into());
            }
            subscribe_hises.insert(index, his);
        }
        let mut brokers = Vector::new();
        for broker in brokers_tmp.into_iter() {
            brokers.push_back(broker.into());
        }
        Ok(AppData {
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
        }
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
    use crate::data::db::Broker;
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
