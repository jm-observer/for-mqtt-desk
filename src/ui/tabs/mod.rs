use crate::data::hierarchy::AppData;
use crate::data::localized::Locale;
use crate::data::AppEvent;
use crate::ui::ids::{SELECTOR_TABS_SELECTED, TABS_ID};
use crate::ui::tabs::brokers_tab::BrokersTabs;
use crossbeam_channel::Sender;
use druid::theme::{BORDER_LIGHT, TEXTBOX_BORDER_WIDTH};
use druid::widget::{Axis, Container, Controller, Tabs, TabsEdge, TabsTransition};
use druid::{Env, Event, EventCtx, Widget, WidgetExt};

mod broker_tab;
mod brokers_tab;

pub fn init_brokers_tabs(tx: Sender<AppEvent>, locale: Locale) -> impl Widget<AppData> {
    let tabs = Container::new(
        Tabs::for_policy(BrokersTabs(tx, locale))
            .with_axis(Axis::Horizontal)
            .with_edge(TabsEdge::Leading)
            .with_transition(TabsTransition::Instant)
            .controller(TabsControler)
            .with_id(TABS_ID)
            .padding(2.0),
    )
    .rounded(8.0)
    .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH);
    tabs
}

struct TabsControler;

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
                if let Some(index) = cmd.get(SELECTOR_TABS_SELECTED) {
                    child.set_tab_index(*index);
                    // } else if let Some(index) = cmd.get(SELECTOR_TABS_CLOSE) {
                    //     child.set_tab_index(*index);
                }
            }
            _ => child.event(_ctx, event, _data, _env),
        }
    }
}
