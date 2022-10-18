use crate::data::common::{
    Id, Msg, PublicInput, PublicMsg, PublicStatus, QoS, SubscribeHis, SubscribeInput, SubscribeMsg,
    SubscribeStatus, SubscribeTopic,
};
use crate::data::AString;
use druid::Data;
use std::sync::Arc;

impl SubscribeTopic {
    pub fn from(val: SubscribeInput, pkid: u16) -> Self {
        Self {
            pkid,
            topic: val.topic.clone(),
            qos: QoS::AtLeastOnce,
            status: SubscribeStatus::SubscribeIng,
        }
    }
    pub fn from_his(val: SubscribeHis, pkid: u16) -> Self {
        Self {
            pkid,
            topic: val.topic.clone(),
            qos: val.qos,
            status: SubscribeStatus::SubscribeIng,
        }
    }
    pub fn is_sucess(&self) -> bool {
        if self.status == SubscribeStatus::SubscribeSuccess {
            true
        } else {
            false
        }
    }
}

impl PublicMsg {
    pub fn from(val: PublicInput, pkid: u16) -> Self {
        Self {
            pkid,
            topic: val.topic.clone(),
            msg: val.msg.clone(),
            qos: QoS::AtLeastOnce,
            status: PublicStatus::Ing,
        }
    }
}

impl From<SubscribeInput> for SubscribeHis {
    fn from(val: SubscribeInput) -> Self {
        Self {
            id: Id::default(),
            broker_id: val.broker_id,
            topic: val.topic.clone(),
            qos: QoS::AtMostOnce,
        }
    }
}

impl Data for SubscribeHis {
    fn same(&self, _other: &Self) -> bool {
        true
    }
}

impl From<PublicMsg> for Msg {
    fn from(val: PublicMsg) -> Self {
        Self::Public(val)
    }
}
impl From<SubscribeMsg> for Msg {
    fn from(val: SubscribeMsg) -> Self {
        Self::Subscribe(val)
    }
}

impl SubscribeInput {
    pub fn init(broker_id: usize) -> Self {
        Self {
            broker_id,
            topic: Arc::new("".to_string()),
            qos: QoS::AtMostOnce,
        }
    }
}
impl Msg {
    pub fn qos(&self) -> &QoS {
        match self {
            Msg::Subscribe(msg) => &msg.qos,
            Msg::Public(msg) => &msg.qos,
        }
    }
    pub fn msg(&self) -> &AString {
        match self {
            Msg::Subscribe(msg) => &msg.msg,
            Msg::Public(msg) => &msg.msg,
        }
    }
    pub fn topic(&self) -> &AString {
        match self {
            Msg::Subscribe(msg) => &msg.topic,
            Msg::Public(msg) => &msg.topic,
        }
    }
}

impl From<rumqttc::v5::mqttbytes::QoS> for QoS {
    fn from(qos: rumqttc::v5::mqttbytes::QoS) -> Self {
        match qos {
            rumqttc::v5::mqttbytes::QoS::AtLeastOnce => Self::AtLeastOnce,
            rumqttc::v5::mqttbytes::QoS::AtMostOnce => Self::AtMostOnce,
            rumqttc::v5::mqttbytes::QoS::ExactlyOnce => Self::ExactlyOnce,
        }
    }
}
impl From<QoS> for rumqttc::v5::mqttbytes::QoS {
    fn from(qos: QoS) -> Self {
        match qos {
            QoS::AtLeastOnce => Self::AtLeastOnce,
            QoS::AtMostOnce => Self::AtMostOnce,
            QoS::ExactlyOnce => Self::ExactlyOnce,
        }
    }
}
