use crate::data::common::{Msg, PublicMsgInput, SubscribeInput, SubscribeTopic};
use crate::data::db::{Broker, SubscribeHises};
use crate::data::AString;
use crate::util::db::ArcDb;
use druid::im::Vector;
use druid::{im::HashMap, Data, Lens};
use std::sync::Arc;
#[derive(Debug, Clone, Lens, Data)]
pub struct AppData {
    pub brokers: HashMap<usize, Arc<Broker>>,
    pub broker_tabs: HashMap<usize, AString>,
    pub subscribe_hises: HashMap<usize, Arc<SubscribeHises>>,
    pub subscribe_topics: HashMap<usize, Vector<Arc<SubscribeTopic>>>,
    pub msgs: HashMap<usize, Vector<Arc<Msg>>>,
    pub subscribe_ing: HashMap<usize, Arc<SubscribeInput>>,
    pub public_ing: HashMap<usize, Arc<PublicMsgInput>>,
    #[data(ignore)]
    #[lens(ignore)]
    pub db: ArcDb,
}
