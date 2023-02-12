use crate::data::common::QoS;
use crate::ui::common::GREEN;
use druid::widget::TextBox;
use druid::{Data, Lens, Widget, WidgetExt};
use for_mqtt_client::QoSWithPacketId;
use std::sync::Arc;

pub fn qos_init<T: Data>(data: impl Lens<T, String>) -> impl Widget<T> {
    TextBox::<String>::new()
        .fix_width(15.0)
        .padding(1.0)
        .lens(data)
}
pub fn qos_success<T: Data>(data: impl Lens<T, String>) -> impl Widget<T> {
    TextBox::<String>::new()
        .fix_width(15.0)
        .padding(1.0)
        .background(GREEN)
        .lens(data)
}
