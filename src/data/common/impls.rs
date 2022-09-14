use crate::data::common::{
    Msg, PublicInput, PublicMsg, PublicStatus, QoS, SubscribeHis, SubscribeInput, SubscribeMsg,
    SubscribeStatus, SubscribeTopic,
};
use crate::data::AString;
use druid::Data;
use std::sync::Arc;

impl SubscribeTopic {
    pub fn from(val: Arc<SubscribeInput>, pkid: u16) -> Self {
        Self {
            pkid,
            topic: val.topic.clone(),
            qos: QoS::AtLeastOnce,
            status: SubscribeStatus::Ing,
        }
    }
}

impl PublicMsg {
    pub fn from(val: Arc<PublicInput>, pkid: u16) -> Self {
        Self {
            pkid,
            topic: val.topic.clone(),
            msg: val.msg.clone(),
            qos: QoS::AtLeastOnce,
            status: PublicStatus::Ing,
        }
    }
}

impl From<Arc<SubscribeInput>> for SubscribeHis {
    fn from(val: Arc<SubscribeInput>) -> Self {
        Self {
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

impl From<rumqttc::v5::QoS> for QoS {
    fn from(qos: rumqttc::v5::QoS) -> Self {
        match qos {
            rumqttc::v5::QoS::AtLeastOnce => Self::AtLeastOnce,
            rumqttc::v5::QoS::AtMostOnce => Self::AtMostOnce,
            rumqttc::v5::QoS::ExactlyOnce => Self::ExactlyOnce,
        }
    }
}
