use crate::data::common::{
    Msg, PublicMsgInput, SubscribeHis, SubscribeInput, SubscribeTopic, TabStatus,
};
use crate::data::db::Broker;
use crate::util::db::ArcDb;
use druid::im::Vector;
use druid::{im::HashMap, Data, Lens};
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
}
