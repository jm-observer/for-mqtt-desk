pub mod common;
pub mod db;
pub mod hierarchy;
pub mod lens;

use std::sync::Arc;

pub type AString = Arc<String>;

pub enum AppEvent {
    Connect(usize),
}
