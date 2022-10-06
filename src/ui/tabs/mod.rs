use crate::data::hierarchy::AppData;
use crate::data::AppEvent;
use crate::ui::tabs::brokers_tab::BrokersTabs;
use druid::widget::{Axis, Controller, Tabs, TabsEdge, TabsTransition};
use druid::{Env, Event, EventCtx, Selector, Widget, WidgetExt, WidgetId};
use std::sync::mpsc::Sender;

mod broker_tab;
mod brokers_tab;

pub fn init_brokers_tabs(tx: Sender<AppEvent>) -> impl Widget<AppData> {
    let tabs = Tabs::for_policy(BrokersTabs(tx))
        .with_axis(Axis::Horizontal)
        .with_edge(TabsEdge::Leading)
        .with_transition(TabsTransition::Instant)
        .controller(TabsControler)
        .with_id(ID_ONE);

    tabs.fix_width(600.0).fix_height(700.0)
}

struct TabsControler;
pub const INCREMENT: Selector<usize> = Selector::new("identity-example.increment");
pub const ID_ONE: WidgetId = WidgetId::reserved(1);

impl Controller<AppData, Tabs<BrokersTabs>> for TabsControler {
    fn event(
        &mut self,
        child: &mut Tabs<BrokersTabs>,
        _ctx: &mut EventCtx,
        event: &Event,
        _data: &mut AppData,
        _env: &Env,
    ) {
        match event {
            Event::Command(cmd) => {
                if let Some(index) = cmd.get(INCREMENT) {
                    // error!("{} {}", index, _data.broker_tabs.len());
                    child.set_tab_index(*index);
                }
            }
            _ => child.event(_ctx, event, _data, _env),
        }
    }
}
