use crate::data::common::QoS;
use crate::data::AString;
use crate::ui::ids::{ID_ADDR, SHOW_ERROR};
use crate::ForError;
use druid::text::Formatter;
use druid::text::{Selection, Validation, ValidationError};
use druid::EventCtx;
use log::debug;
use std::fmt::Display;

#[derive(Debug)]
pub struct MustInputError;

// impl std::error::Error for MustInputError {}
//
// impl Display for MustInputError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Must input")
//     }
// }
pub struct MustInput;
impl Formatter<AString> for MustInput {
    fn format(&self, value: &AString) -> String {
        value.as_str().to_string()
    }

    fn validate_partial_input(&self, input: &str, sel: &Selection) -> Validation {
        parse_to_no_empty(input).to_validation()
    }
    fn value(&self, input: &str) -> Result<AString, ValidationError> {
        parse_to_no_empty(input).to_validation_error()
    }
}

impl Formatter<u16> for MustInput {
    fn format(&self, value: &u16) -> String {
        value.to_string()
    }

    fn validate_partial_input(&self, input: &str, sel: &Selection) -> Validation {
        parse_to_port(input).to_validation()
    }
    fn value(&self, input: &str) -> Result<u16, ValidationError> {
        debug!("Value: {}", input);
        parse_to_port(input).to_validation_error()
    }
}
impl Formatter<QoS> for MustInput {
    fn format(&self, value: &QoS) -> String {
        value.to_string()
    }

    fn validate_partial_input(&self, input: &str, sel: &Selection) -> Validation {
        parse_to_qos(input).to_validation()
    }
    fn value(&self, input: &str) -> Result<QoS, ValidationError> {
        parse_to_qos(input).to_validation_error()
    }
}
pub trait Portable<T> {
    fn to_validation(self) -> Validation;
    fn to_validation_error(self) -> Result<T, ValidationError>;
}

impl<T: ToString> Portable<T> for Result<T, ForError> {
    fn to_validation(self) -> Validation {
        match self {
            Ok(val) => Validation::success().change_text(val.to_string()),
            Err(e) => Validation::failure(e),
        }
    }

    fn to_validation_error(self) -> Result<T, ValidationError> {
        self.map_err(|x| ValidationError::new(x))
    }
}

pub fn parse_to_port(input: &str) -> Result<u16, ForError> {
    if input.is_empty() {
        return Err(ForError::NotEmpty);
    }
    input.parse().map_err(|_| ForError::InvalidPort)
}
pub fn parse_to_no_empty(input: &str) -> Result<AString, ForError> {
    // debug!("{}", input);
    if input.is_empty() {
        return Err(ForError::NotEmpty);
    }
    Ok(input.to_string().into())
}
pub fn parse_to_qos(input: &str) -> Result<QoS, ForError> {
    if input.is_empty() {
        return Err(ForError::NotEmpty);
    }
    match input {
        "0" => Ok(QoS::AtMostOnce),
        "1" => Ok(QoS::AtLeastOnce),
        "2" => Ok(QoS::ExactlyOnce),
        _ => Err(ForError::InvalidQos),
    }
}
pub fn check_no_empty(input: &str) -> bool {
    if parse_to_no_empty(input).is_err() {
        return false;
    }
    true
}
pub fn check_qos(input: &str) -> bool {
    if parse_to_qos(input).is_err() {
        return false;
    }
    true
}
pub fn check_addr(input: &str) -> bool {
    if parse_to_no_empty(input).is_err() {
        return false;
    }
    true
}
pub fn check_port(input: &str) -> bool {
    if parse_to_port(input).is_err() {
        return false;
    }
    true
}
