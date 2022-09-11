pub mod common;
pub mod db;
pub mod hierarchy;

use std::sync::Arc;

pub type AString = Arc<String>;

pub enum AppEvent {
    Connect(AString),
}
