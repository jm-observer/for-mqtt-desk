use crate::data::common::{
    Msg, PublicMsgInput, SubscribeHis, SubscribeInput, SubscribeTopic, TabStatus,
};
use crate::data::db::Broker;
use crate::util::db::ArcDb;
use druid::im::Vector;
use druid::{im::HashMap, Data, Lens};
use rumqttc::v5::AsyncClient;
use std::sync::Arc;

#[derive(Debug, Clone, Lens, Data)]
pub struct AppData {
    pub brokers: Vector<Arc<Broker>>,
    pub broker_tabs: Vector<usize>,
    pub tab_statuses: HashMap<usize, TabStatus>,
    pub subscribe_hises: HashMap<usize, Vector<Arc<SubscribeHis>>>,
    pub subscribe_topics: HashMap<usize, Vector<Arc<SubscribeTopic>>>,
    pub msgs: HashMap<usize, Vector<Arc<Msg>>>,
    pub subscribe_ing: HashMap<usize, Arc<SubscribeInput>>,
    pub public_ing: HashMap<usize, Arc<PublicMsgInput>>,
    #[data(ignore)]
    #[lens(ignore)]
    pub db: ArcDb,
    #[data(ignore)]
    #[lens(ignore)]
    pub mqtt_clients: HashMap<usize, AsyncClient>,
}

impl AppData {
    pub fn find_broker(&self, id: usize) -> Option<&Arc<Broker>> {
        self.brokers.iter().find(|x| (*x).id == id)
    }
    pub fn init_connection(&mut self, id: usize) {
        if let Some(status) = self.tab_statuses.get_mut(&id) {
            status.try_connect = true;
        }
        if self.subscribe_hises.get_mut(&id).is_none() {
            self.subscribe_hises.insert(id, Vector::new());
        }
        self.subscribe_topics.insert(id, Vector::new());
        self.msgs.insert(id, Vector::new());
        self.subscribe_ing
            .insert(id, SubscribeInput::default().into());
        self.public_ing.insert(id, PublicMsgInput::default().into());
    }
}
