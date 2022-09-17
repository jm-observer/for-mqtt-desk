use crate::data::hierarchy::AppData;
use crate::ui::broker_list::init_connect;
use crate::ui::tabs::init_brokers_tabs;
use druid::theme::{BORDER_LIGHT, TEXTBOX_BORDER_WIDTH};
use druid::widget::{Container, Padding, Split};
use druid::Widget;

mod broker_info;
mod broker_list;
mod common;
mod connection;
mod tabs;

pub fn init_layout() -> impl Widget<AppData> {
    Padding::new(
        5.0,
        Container::new(
            Split::columns(init_connect(), init_brokers_tabs())
                .split_point(0.25)
                // .bar_size(1.0)
                .draggable(true),
        )
        .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH),
    )
}
