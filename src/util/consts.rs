use crate::data::common::QoS;
use for_mqtt_client::QoSWithPacketId;
use lazy_static::lazy_static;
use std::sync::Arc;
lazy_static! {
    pub static ref QOS_0: Arc<String> = Arc::new("0".to_string());
}
lazy_static! {
    pub static ref QOS_1: Arc<String> = Arc::new("1".to_string());
}
lazy_static! {
    pub static ref QOS_2: Arc<String> = Arc::new("2".to_string());
}
lazy_static! {
    pub static ref TY_TEXT: Arc<String> = Arc::new("T".to_string());
}
lazy_static! {
    pub static ref TY_JSON: Arc<String> = Arc::new("J".to_string());
}
lazy_static! {
    pub static ref TY_HEX: Arc<String> = Arc::new("H".to_string());
}

pub trait QosToString {
    fn qos_to_string(&self) -> Arc<String>;
}
impl QosToString for QoS {
    fn qos_to_string(&self) -> Arc<String> {
        match self {
            QoS::AtMostOnce => QOS_0.clone(),
            QoS::AtLeastOnce => QOS_1.clone(),
            QoS::ExactlyOnce => QOS_2.clone(),
        }
    }
}
impl QosToString for for_mqtt_client::QoS {
    fn qos_to_string(&self) -> Arc<String> {
        match self {
            for_mqtt_client::QoS::AtMostOnce => QOS_0.clone(),
            for_mqtt_client::QoS::AtLeastOnce => QOS_1.clone(),
            for_mqtt_client::QoS::ExactlyOnce => QOS_2.clone(),
        }
    }
}
impl QosToString for QoSWithPacketId {
    fn qos_to_string(&self) -> Arc<String> {
        match self {
            QoSWithPacketId::AtMostOnce => QOS_0.clone(),
            QoSWithPacketId::AtLeastOnce(_) => QOS_1.clone(),
            QoSWithPacketId::ExactlyOnce(_) => QOS_2.clone(),
        }
    }
}
