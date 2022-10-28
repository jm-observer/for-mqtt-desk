use crate::data::hierarchy::AppData;
use crate::data::AppEvent;
use crate::ui::tabs::brokers_tab::BrokersTabs;
use crossbeam_channel::Sender;
use druid::theme::{BORDER_LIGHT, TEXTBOX_BORDER_WIDTH};
use druid::widget::{Axis, Controller, Tabs, TabsEdge, TabsTransition};
use druid::{Env, Event, EventCtx, Selector, Widget, WidgetExt, WidgetId};

mod broker_tab;
mod brokers_tab;

pub fn init_brokers_tabs(tx: Sender<AppEvent>) -> impl Widget<AppData> {
    let tabs = Tabs::for_policy(BrokersTabs(tx))
        .with_axis(Axis::Horizontal)
        .with_edge(TabsEdge::Leading)
        .with_transition(TabsTransition::Instant)
        .controller(TabsControler)
        .with_id(ID_ONE)
        .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH)
        .padding(5.0);

    // tabs.fix_width(600.0).fix_height(700.0)
    tabs
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
                    child.set_tab_index(*index);
                }
            }
            _ => child.event(_ctx, event, _data, _env),
        }
    }
}
