use crate::data::hierarchy::AppData;
use crate::data::AppEvent;
use crate::ui::tabs::brokers_tab::BrokersTabs;
use druid::widget::{Axis, Tabs, TabsEdge, TabsTransition};
use druid::{Widget, WidgetExt};
use std::sync::mpsc::Sender;

mod broker_tab;
mod brokers_tab;

pub fn init_brokers_tabs(tx: Sender<AppEvent>) -> impl Widget<AppData> {
    Tabs::for_policy(BrokersTabs(tx))
        .with_axis(Axis::Horizontal)
        .with_edge(TabsEdge::Leading)
        .with_transition(TabsTransition::Instant)
        .fix_width(600.0)
        .fix_height(700.0)
}
