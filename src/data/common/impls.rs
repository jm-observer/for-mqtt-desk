use crate::data::common::{
    Id, Msg, PublicInput, PublicMsg, PublicStatus, QoS, SubscribeHis, SubscribeInput, SubscribeMsg,
    SubscribeStatus, SubscribeTopic,
};
use crate::data::AString;
use crate::mqtt;
use crate::util::consts::QosToString;
use crate::util::now_time;
use druid::Data;
use std::sync::Arc;

impl SubscribeTopic {
    pub fn from(val: SubscribeInput, packet_id: u32) -> Self {
        Self {
            trace_id: packet_id,
            topic: val.topic.clone(),
            qos: val.qos.qos_to_string(),
            status: SubscribeStatus::SubscribeIng,
            payload_ty: val.payload_ty,
        }
    }
    pub fn from_his(val: SubscribeHis, trace_id: u32) -> Self {
        Self {
            trace_id,
            topic: val.topic.clone(),
            qos: val.qos.qos_to_string(),
            status: SubscribeStatus::SubscribeIng,
            payload_ty: val.payload_ty,
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
    pub fn from(val: PublicInput, trace_id: u32) -> Self {
        Self {
            trace_id,
            topic: val.topic.clone(),
            msg: val.msg.clone(),
            qos: val.qos.qos_to_string(),
            status: PublicStatus::Ing,
            payload_ty: val.payload_ty.to_arc_string(),
            time: Arc::new(now_time()),
        }
    }
}

impl From<SubscribeInput> for SubscribeHis {
    fn from(val: SubscribeInput) -> Self {
        Self {
            id: Id::default(),
            broker_id: val.broker_id,
            selected: false,
            topic: val.topic.clone(),
            qos: val.qos.clone(),
            payload_ty: val.payload_ty,
        }
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
            payload_ty: Default::default(),
        }
    }
}
impl Msg {
    // pub fn qos(&self) -> &QoS {
    //     match self {
    //         Msg::Subscribe(msg) => &msg.qos,
    //         Msg::Public(msg) => &msg.qos,
    //     }
    // }
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

impl From<mqtt::QoS> for QoS {
    fn from(qos: mqtt::QoS) -> Self {
        match qos {
            mqtt::QoS::AtLeastOnce => Self::AtLeastOnce,
            mqtt::QoS::AtMostOnce => Self::AtMostOnce,
            mqtt::QoS::ExactlyOnce => Self::ExactlyOnce,
        }
    }
}
impl From<mqtt::QoSWithPacketId> for QoS {
    fn from(qos: mqtt::QoSWithPacketId) -> Self {
        match qos {
            mqtt::QoSWithPacketId::AtLeastOnce(_) => Self::AtLeastOnce,
            mqtt::QoSWithPacketId::AtMostOnce => Self::AtMostOnce,
            mqtt::QoSWithPacketId::ExactlyOnce(_) => Self::ExactlyOnce,
        }
    }
}
impl From<QoS> for mqtt::QoS {
    fn from(qos: QoS) -> Self {
        match qos {
            QoS::AtLeastOnce => Self::AtLeastOnce,
            QoS::AtMostOnce => Self::AtMostOnce,
            QoS::ExactlyOnce => Self::ExactlyOnce,
        }
    }
}
