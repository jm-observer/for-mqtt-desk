use crate::data::common::{PublicInput, SubscribeInput};
use rumqttc::v5::QoS;
use std::sync::Arc;

pub struct MqttPublicInput {
    pub topic: String,
    pub msg: String,
    pub qos: QoS,
    pub retain: bool,
}

pub struct MqttSubscribeInput {
    pub topic: String,
    pub qos: QoS,
}

impl From<Arc<PublicInput>> for MqttPublicInput {
    fn from(val: Arc<PublicInput>) -> Self {
        Self {
            topic: val.topic.as_ref().clone(),
            msg: val.msg.as_ref().clone(),
            qos: QoS::AtLeastOnce,
            retain: val.retain,
        }
    }
}
impl From<Arc<SubscribeInput>> for MqttSubscribeInput {
    fn from(val: Arc<SubscribeInput>) -> Self {
        Self {
            topic: val.topic.as_ref().clone(),
            qos: QoS::AtLeastOnce,
        }
    }
}
