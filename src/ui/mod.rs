use crate::data::hierarchy::AppData;
use crate::data::lens::BrokerSelectedOrZero;
use crate::data::AppEvent;
use crate::ui::broker_info::display_broker;
use crate::ui::broker_list::init_broker_list;

use crate::data::localized::Locale;
use crate::ui::icons::{broker_info, broker_list, tips};
use crate::ui::ids::TIPS;
use crate::ui::tabs::init_brokers_tabs;
use crossbeam_channel::Sender;
use druid::theme::{BACKGROUND_DARK, BORDER_LIGHT, TEXTBOX_BORDER_WIDTH};
use druid::widget::{Container, CrossAxisAlignment, Either, Flex, Label, Split, Svg};
use druid::{Env, UnitPoint, Widget, WidgetExt};
use log::debug;

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
pub mod theme;
pub mod tips;

pub fn init_layout(tx: Sender<AppEvent>, locale: Locale) -> impl Widget<AppData> {
    let hint = Label::dynamic(|data: &AppData, _: &Env| format!("{}", data.hint))
        .with_text_size(12.0)
        .expand_width()
        // .debug_paint_layout()
        .align_vertical(UnitPoint::LEFT)
        .fix_height(18.0)
        .padding((35.0, 5.0));

    let history = Svg::new(broker_list())
        .fix_size(25.0, 25.0)
        .on_click(move |_ctx, data: &mut bool, _env| {
            *data = !*data;
        })
        .background(BACKGROUND_DARK)
        .lens(AppData::display_history);
    let info = Svg::new(broker_info())
        .fix_size(25.0, 25.0)
        .background(BACKGROUND_DARK)
        .on_click(|_ctx, data: &mut bool, _env| {
            debug!("info: {}", data);
            *data = !*data;
        })
        .lens(AppData::display_broker_info);
    let tips = Svg::new(tips())
        .fix_size(25.0, 25.0)
        .background(BACKGROUND_DARK)
        // .padding((0.0, 5.0))
        .on_click(|_ctx, _data: &mut AppData, _env| _ctx.submit_command(TIPS));
    let info = Flex::column()
        .with_child(info)
        // .with_child(info)
        .expand_height()
        .padding((0.0, 5.0));
    let icons = Flex::column()
        .with_child(history)
        // .with_child(info)
        .with_child(tips)
        .expand_height()
        .padding((0.0, 5.0));

    let content = Flex::row()
        .with_child(icons)
        .with_flex_child(display_history(tx, locale.clone()).padding((5.0, 0.0)), 1.0)
        .with_child(info)
        .padding((5.0, 1.0));

    let flex = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_flex_child(content, 1.0)
        .with_child(hint)
        .expand_height()
        .expand_width();
    flex
}

fn display_history(tx: Sender<AppEvent>, locale: Locale) -> impl Widget<AppData> {
    Either::new(
        |data: &AppData, _env| data.display_history,
        Split::columns(
            Container::new(init_broker_list(tx.clone()))
                .rounded(8.0)
                .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH),
            Container::new(display_broker_info(tx.clone(), locale.clone()))
                .rounded(8.0)
                .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH),
        )
        .split_point(0.22)
        .draggable(true)
        .bar_size(0.0),
        display_broker_info(tx.clone(), locale.clone()),
    )
}

fn display_broker_info(tx: Sender<AppEvent>, locale: Locale) -> impl Widget<AppData> {
    Either::new(
        |data: &AppData, _env| data.display_broker_info,
        Split::columns(
            init_brokers_tabs(tx.clone(), locale.clone()),
            Container::new(
                display_broker(0, tx.clone(), locale.clone())
                    .lens(BrokerSelectedOrZero)
                    .expand_height(),
            )
            .rounded(8.0)
            .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH),
        )
        .split_point(0.60)
        .bar_size(0.0)
        .draggable(true),
        init_brokers_tabs(tx.clone(), locale.clone()),
    )
}
