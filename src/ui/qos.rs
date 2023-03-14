use crate::data::common::QoS;
use crate::ui::common::GREEN;
use druid::widget::TextBox;
use druid::{Data, Lens, Widget, WidgetExt};
use druid_widget_nursery::DropdownSelect;
use for_mqtt_client::QoSWithPacketId;
use std::sync::Arc;

pub fn qos_init<T: Data>(data: impl Lens<T, Arc<String>>) -> impl Widget<T> {
    TextBox::<Arc<String>>::new()
        .fix_width(15.0)
        .padding(1.0)
        .disabled_if(|_, _| true)
        .lens(data)
}
pub fn qos_success<T: Data>(data: impl Lens<T, Arc<String>>) -> impl Widget<T> {
    TextBox::<Arc<String>>::new()
        .background(GREEN)
        .fix_width(15.0)
        .padding(1.0)
        .disabled_if(|_, _| true)
        .lens(data)
}

pub fn down_select_qos() -> impl Widget<QoS> {
    DropdownSelect::new(vec![
        ("0", QoS::AtMostOnce),
        ("1", QoS::AtLeastOnce),
        ("2", QoS::ExactlyOnce),
    ])
}
