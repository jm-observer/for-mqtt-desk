use crate::data::common::QoS;
use for_mqtt_client::QoSWithPacketId;
use lazy_static::lazy_static;
use std::sync::Arc;
lazy_static! {
    pub static ref QOS0: Arc<String> = Arc::new("0".to_string());
}
lazy_static! {
    pub static ref QOS1: Arc<String> = Arc::new("1".to_string());
}
lazy_static! {
    pub static ref QOS2: Arc<String> = Arc::new("2".to_string());
}

pub trait QosToString {
    fn qos_to_string(&self) -> Arc<String>;
}
impl QosToString for QoS {
    fn qos_to_string(&self) -> Arc<String> {
        match self {
            QoS::AtMostOnce => QOS0.clone(),
            QoS::AtLeastOnce => QOS1.clone(),
            QoS::ExactlyOnce => QOS2.clone(),
        }
    }
}
impl QosToString for for_mqtt_client::QoS {
    fn qos_to_string(&self) -> Arc<String> {
        match self {
            for_mqtt_client::QoS::AtMostOnce => QOS0.clone(),
            for_mqtt_client::QoS::AtLeastOnce => QOS1.clone(),
            for_mqtt_client::QoS::ExactlyOnce => QOS2.clone(),
        }
    }
}
impl QosToString for QoSWithPacketId {
    fn qos_to_string(&self) -> Arc<String> {
        match self {
            QoSWithPacketId::AtMostOnce => QOS0.clone(),
            QoSWithPacketId::AtLeastOnce(_) => QOS1.clone(),
            QoSWithPacketId::ExactlyOnce(_) => QOS2.clone(),
        }
    }
}
