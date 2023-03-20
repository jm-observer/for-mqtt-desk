use crate::data::hierarchy::AppData;
use crate::data::lens::BrokerSelectedOrZero;
use crate::data::AppEvent;
use crate::ui::broker_info::display_broker;
use crate::ui::broker_list::init_broker_list;
use crate::ui::common::{label_dy_expand_width, label_static, LABLE_PADDING};
use crate::ui::tabs::init_brokers_tabs;
use crossbeam_channel::Sender;
use druid::theme::{BORDER_LIGHT, TEXTBOX_BORDER_WIDTH};
use druid::widget::{
    Axis, Button, Container, CrossAxisAlignment, Either, Flex, Label, LabelText, Padding, Split,
    Tabs, TabsEdge, TabsTransition,
};
use druid::{Env, UnitPoint, Widget, WidgetExt};
use log::{debug, info};

pub mod auto_scroll;
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
pub mod tips;

pub fn init_layout(tx: Sender<AppEvent>) -> impl Widget<AppData> {
    let hint = Label::dynamic(|data: &AppData, _: &Env| format!("{}", data.hint))
        .with_text_size(12.0)
        .expand_width()
        .debug_paint_layout()
        .align_vertical(UnitPoint::LEFT)
        .fix_height(40.0)
        .padding(LABLE_PADDING);

    let history = Button::new("History")
        .on_click(|ctx, data: &mut bool, env| {
            info!("history click: {}", data);
            *data = !*data;
        })
        .lens(AppData::display_history);
    let info = Button::new("Info")
        .on_click(|ctx, data: &mut bool, env| {
            debug!("info: {}", data);
            *data = !*data;
        })
        .lens(AppData::display_broker_info);

    let content = Flex::row()
        .with_child(history)
        .with_flex_child(display_history(tx), 1.0)
        .with_child(info);

    let flex = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_flex_child(content, 1.0)
        .with_child(hint)
        .expand_height()
        .expand_width();
    flex.border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH)
    // history.debug_paint_layout()
    // Tabs::Padding::new(5.0, flex).border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH)
}

fn display_history(tx: Sender<AppEvent>) -> impl Widget<AppData> {
    Either::new(
        |data: &AppData, _env| data.display_history,
        Split::columns(
            Container::new(init_broker_list(tx.clone()))
                .rounded(8.0)
                .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH),
            Container::new(display_broker_info(tx.clone()))
                .rounded(8.0)
                .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH),
        )
        .split_point(0.25)
        .draggable(true)
        .bar_size(0.0)
        .padding(5.0),
        display_broker_info(tx.clone()),
    )
}

fn display_broker_info(tx: Sender<AppEvent>) -> impl Widget<AppData> {
    Either::new(
        |data: &AppData, _env| data.display_broker_info,
        Split::columns(
            Container::new(init_brokers_tabs(tx.clone()))
                .rounded(8.0)
                .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH),
            Container::new(
                display_broker(0, tx.clone())
                    .lens(BrokerSelectedOrZero)
                    .expand_height(),
            )
            .rounded(8.0)
            .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH),
        )
        .split_point(0.55)
        .bar_size(0.0)
        .draggable(true)
        // .bar_size(3.0)
        // .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH)
        .padding(5.0),
        init_brokers_tabs(tx.clone()),
    )
}
