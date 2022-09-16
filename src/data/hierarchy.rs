use crate::data::common::{
    Msg, PublicInput, PublicMsg, SubscribeHis, SubscribeInput, SubscribeMsg, SubscribeTopic,
    TabStatus,
};
use crate::data::db::Broker;
use crate::util::db::ArcDb;
use anyhow::Result;
use druid::im::Vector;
use druid::{im::HashMap, Data, Lens};
use rumqttc::v5::AsyncClient;

#[derive(Debug, Clone, Lens, Data)]
pub struct AppData {
    pub brokers: Vector<Broker>,
    pub broker_tabs: Vector<usize>,
    pub tab_statuses: HashMap<usize, TabStatus>,
    pub subscribe_hises: HashMap<usize, Vector<SubscribeHis>>,
    pub subscribe_topics: HashMap<usize, Vector<SubscribeTopic>>,
    pub msgs: HashMap<usize, Vector<Msg>>,
    pub subscribe_ing: HashMap<usize, SubscribeInput>,
    pub public_ing: HashMap<usize, PublicInput>,
    #[data(ignore)]
    #[lens(ignore)]
    pub db: ArcDb,
    #[data(ignore)]
    #[lens(ignore)]
    pub mqtt_clients: HashMap<usize, AsyncClient>,
}

impl AppData {
    pub fn find_broker(&self, id: usize) -> Option<&Broker> {
        self.brokers.iter().find(|x| (*x).id == id)
    }
    pub fn init_connection(&mut self, id: usize) -> Result<()> {
        if let Some(status) = self.tab_statuses.get_mut(&id) {
            status.try_connect = true;
        }
        if let Some(broker) = self.brokers.iter_mut().find(|x| (*x).id == id) {
            broker.stored = true;
            self.db.save_broker(id, broker)?;
        }
        if self.subscribe_hises.get_mut(&id).is_none() {
            self.subscribe_hises.insert(id, Vector::new());
        }
        self.subscribe_topics.insert(id, Vector::new());
        self.msgs.insert(id, Vector::new());
        self.subscribe_ing
            .insert(id, SubscribeInput::default().into());
        self.public_ing.insert(id, PublicInput::default().into());
        Ok(())
    }
    pub fn subscribe(&mut self, id: usize, input: SubscribeInput, pkid: u16) {
        if let Some(subscribe_topics) = self.subscribe_topics.get_mut(&id) {
            let sub = SubscribeTopic::from(input.clone(), pkid);
            subscribe_topics.push_back(sub.into());
        }
        if let Some(subscribe_hises) = self.subscribe_hises.get_mut(&id) {
            let his: SubscribeHis = input.into();
            if subscribe_hises.iter().find(|x| *x == &his).is_none() {
                subscribe_hises.push_back(his.into());
            }
        }
    }
    pub fn public(&mut self, id: usize, input: PublicInput, pkid: u16) {
        if let Some(msgs) = self.msgs.get_mut(&id) {
            let sub: Msg = PublicMsg::from(input.clone(), pkid).into();
            msgs.push_back(sub.into());
        }
    }
    pub fn receive_msg(&mut self, id: usize, input: SubscribeMsg) {
        if let Some(msgs) = self.msgs.get_mut(&id) {
            let sub: Msg = input.into();
            msgs.push_back(sub.into());
        }
    }
}
