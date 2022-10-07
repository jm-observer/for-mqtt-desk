use crate::data::hierarchy::AppData;
use crate::data::AppEvent;
use crate::ui::broker_list::init_broker_list;
use crate::ui::tabs::init_brokers_tabs;
use druid::theme::{BORDER_LIGHT, TEXTBOX_BORDER_WIDTH};
use druid::widget::{Container, Padding, Split};
use druid::Widget;
use std::sync::mpsc::Sender;

mod broker_info;
mod broker_list;
mod common;
mod connection;
mod debug;
pub mod formatter;
mod ids;
pub mod tabs;

pub fn init_layout(tx: Sender<AppEvent>) -> impl Widget<AppData> {
    Padding::new(
        5.0,
        Container::new(
            Split::columns(init_broker_list(tx.clone()), init_brokers_tabs(tx))
                .split_point(0.25)
                // .bar_size(1.0)
                .draggable(true),
        )
        .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH),
    )
}
