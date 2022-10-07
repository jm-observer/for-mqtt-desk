#![feature(type_alias_impl_trait)]
#![feature(let_else)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
// pub mod config;
pub mod data;
pub mod logic;
pub mod mqtt;
pub mod ui;
pub mod util;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ForError {
    #[error("Only 0~65535")]
    InvalidPort,
    #[error("Only qos0/qos1/qos2")]
    InvalidQos,
    #[error("NotEmpty")]
    NotEmpty,
}
