use crate::data::hierarchy::AppData;
use crate::ui::broker_list::init_connect;
use crate::ui::tabs::init_brokers_tabs;
use druid::widget::{Container, Padding, Split};
use druid::{Color, Widget};

mod broker_info;
mod broker_list;
mod connection;
mod tabs;

pub fn init_layout() -> impl Widget<AppData> {
    Padding::new(
        5.0,
        Container::new(
            Split::columns(init_connect(), init_brokers_tabs())
                .split_point(0.25)
                .draggable(true),
        )
        .border(Color::WHITE, 1.0),
    )
}