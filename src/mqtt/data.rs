use crate::data::common::{PublicInput, QoS, SubscribeHis, SubscribeInput};

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
impl From<SubscribeHis> for MqttSubscribeInput {
    fn from(val: SubscribeHis) -> Self {
        Self {
            topic: val.topic.as_ref().clone(),
            qos: val.qos.into(),
        }
    }
}
