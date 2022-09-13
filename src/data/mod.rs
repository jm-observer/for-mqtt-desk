pub mod common;
pub mod db;
pub mod hierarchy;
pub mod lens;

use crate::data::common::PublicMsg;
use crate::data::db::Broker;
use std::sync::Arc;

pub type AString = Arc<String>;

#[derive(Debug)]
pub enum AppEvent {
    Connect(Arc<Broker>),
    ConnectAckSuccess(usize),
    ConnectAckFail(usize, Arc<String>),
    Public(usize),
}
