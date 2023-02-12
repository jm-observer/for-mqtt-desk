use crate::data::hierarchy::AppData;
use crate::data::AppEvent;
use crate::ui::broker_list::init_broker_list;
use crate::ui::common::{label_dy_expand_width, label_static, LABLE_PADDING};
use crate::ui::tabs::init_brokers_tabs;
use crossbeam_channel::Sender;
use druid::theme::{BORDER_LIGHT, TEXTBOX_BORDER_WIDTH};
use druid::widget::{Container, CrossAxisAlignment, Flex, Label, Padding, Split};
use druid::{Env, UnitPoint, Widget, WidgetExt};

mod broker_info;
mod broker_list;
pub mod common;
mod connection;
mod debug;
pub mod formatter;
pub mod icons;
pub mod ids;
mod payload_ty;
pub mod qos;
pub mod tabs;

pub fn init_layout(tx: Sender<AppEvent>) -> impl Widget<AppData> {
    let hint = Label::dynamic(|data: &AppData, _: &Env| format!("{}", data.hint))
        .with_text_size(12.0)
        .align_vertical(UnitPoint::LEFT)
        .padding(LABLE_PADDING)
        .fix_height(20.0)
        .expand_width();
    let flex = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_flex_child(
            Container::new(
                Split::columns(init_broker_list(tx.clone()), init_brokers_tabs(tx))
                    .split_point(0.25)
                    .draggable(true)
                    .bar_size(0.5),
            ),
            1.0,
        )
        .with_child(hint)
        .expand_height();

    Padding::new(5.0, flex)
        .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH)
        .fix_width(600.0)
        .fix_height(800.0)
}
