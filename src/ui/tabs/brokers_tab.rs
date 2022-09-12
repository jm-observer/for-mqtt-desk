use crate::data::db::Broker;
use crate::data::hierarchy::AppData;
use crate::data::lens::BrokerIndex;
use crate::ui::tabs::broker_tab::BrokerTabPolicy;
use druid::widget::{Axis, Label, TabInfo, Tabs, TabsEdge, TabsPolicy, TabsTransition};
use druid::{Data, Env};
use druid::{Widget, WidgetExt};
use log::debug;
use std::sync::Arc;

#[derive(Clone, Data)]
pub struct BrokersTabs;

impl TabsPolicy for BrokersTabs {
    type Key = usize;
    type Build = ();
    type Input = AppData;
    type LabelWidget = impl Widget<AppData>;
    type BodyWidget = impl Widget<AppData>;

    fn tabs_changed(&self, old_data: &Self::Input, data: &Self::Input) -> bool {
        !(data.broker_tabs == old_data.broker_tabs)
    }

    fn tabs(&self, data: &Self::Input) -> Vec<Self::Key> {
        data.broker_tabs.iter().map(|x| *x).collect()
    }

    fn tab_info(&self, key: Self::Key, data: &Self::Input) -> TabInfo<Self::Input> {
        if let Some(tabs) = data.brokers.iter().find(|x| (*x).id == key) {
            debug!("{}", tabs.name);
            return TabInfo::new(format!("{}", tabs.name), true);
        }
        unreachable!()
    }

    fn tab_body(&self, _key: Self::Key, _data: &Self::Input) -> Self::BodyWidget {
        debug!("tab_body");
        Tabs::for_policy(BrokerTabPolicy(_key))
            .with_axis(Axis::Horizontal)
            .with_edge(TabsEdge::Leading)
            .with_transition(TabsTransition::Instant)
    }

    fn close_tab(&self, key: Self::Key, data: &mut Self::Input) {
        if let Some((index, _)) = data
            .broker_tabs
            .iter()
            .enumerate()
            .map(|x| (x.0, *x.1))
            .find(|x| (*x).1 == key)
        {
            data.broker_tabs.remove(index);
            return;
        }
        unreachable!()
    }

    fn tab_label(
        &self,
        _key: Self::Key,
        _info: TabInfo<Self::Input>,
        _data: &Self::Input,
    ) -> Self::LabelWidget {
        Label::dynamic(|data: &Arc<Broker>, _: &Env| format!("{}", data.name))
            .lens(BrokerIndex(_key))
    }
}
