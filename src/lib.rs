#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]
// #![allow(dead_code)]
// #![allow(unused_imports)]
// #![allow(unused_variables)]
// pub mod config;
pub mod config;
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
    #[error("Only 0/1/2")]
    InvalidQos,
    #[error("Not Empty")]
    NotEmpty,
}
