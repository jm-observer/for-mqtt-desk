use crate::data::common::{PublicInput, SubscribeInput};
use rumqttc::v5::QoS;

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

impl From<PublicInput> for MqttPublicInput {
    fn from(val: PublicInput) -> Self {
        Self {
            topic: val.topic.as_ref().clone(),
            msg: val.msg.as_ref().clone(),
            qos: QoS::AtLeastOnce,
            retain: val.retain,
        }
    }
}
impl From<SubscribeInput> for MqttSubscribeInput {
    fn from(val: SubscribeInput) -> Self {
        Self {
            topic: val.topic.as_ref().clone(),
            qos: QoS::AtLeastOnce,
        }
    }
}
