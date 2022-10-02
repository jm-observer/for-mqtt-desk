use crate::data::hierarchy::AppData;
use crate::data::AppEvent;
use crate::ui::tabs::broker_tab::BrokerTabPolicy;
use druid::widget::{Axis, TabInfo, Tabs, TabsEdge, TabsPolicy, TabsTransition};
use druid::Widget;
use druid::{Data, Env};
use log::{debug, error};
use std::sync::mpsc::Sender;

#[derive(Clone)]
pub struct BrokersTabs(pub Sender<AppEvent>);

impl Data for BrokersTabs {
    fn same(&self, _other: &Self) -> bool {
        true
    }
}
impl TabsPolicy for BrokersTabs {
    type Key = usize;
    type Build = ();
    type Input = AppData;
    type LabelWidget = impl Widget<AppData>;
    type BodyWidget = impl Widget<AppData>;

    fn tabs_changed(&self, old_data: &Self::Input, data: &Self::Input) -> bool {
        if !(data.broker_tabs == old_data.broker_tabs) {
            return true;
        }
        for id in &old_data.broker_tabs {
            let old_broker = old_data.brokers.iter().find(|x| x.id == *id).unwrap();
            let broker = data.brokers.iter().find(|x| x.id == *id).unwrap();
            if !old_broker.same(broker) {
                return true;
            }
        }
        false
    }

    fn tabs(&self, data: &Self::Input) -> Vec<Self::Key> {
        data.broker_tabs.iter().map(|x| *x).collect()
    }

    fn tab_info(&self, key: Self::Key, _data: &Self::Input) -> TabInfo<Self::Input> {
        // if let Some(tabs) = data.brokers.iter().find(|x| (*x).id == key) {
        //     debug!("{}", tabs.name);
        return TabInfo::new(
            move |data: &AppData, _: &Env| {
                if let Some(tabs) = data.brokers.iter().find(|x| (*x).id == key) {
                    format!("{}", tabs.name)
                } else {
                    "".to_string()
                }
                // debug!("data.name={}", data.name);
            },
            true,
        );
        // }
        // unreachable!()
    }

    fn tab_body(&self, _key: Self::Key, _data: &Self::Input) -> Self::BodyWidget {
        debug!("tab_body");
        Tabs::for_policy(BrokerTabPolicy(_key, self.0.clone()))
            .with_axis(Axis::Horizontal)
            .with_edge(TabsEdge::Leading)
            .with_transition(TabsTransition::Instant)
    }

    fn close_tab(&self, key: Self::Key, data: &mut Self::Input) {
        if let Err(_) = data.db.tx.send(AppEvent::CloseBrokerTab(key)) {
            error!("fail to send event");
        }
        // if let Err(e) = data.close_tab(key) {
        //     error!("{:?}", e);
        // }
        // if let Some((index, _)) = data
        //     .broker_tabs
        //     .iter()
        //     .enumerate()
        //     .map(|x| (x.0, *x.1))
        //     .find(|x| (*x).1 == key)
        // {
        //     data.broker_tabs.remove(index);
        //     return;
        // }
        // unreachable!()
    }

    fn tab_label(
        &self,
        _key: Self::Key,
        _info: TabInfo<Self::Input>,
        _data: &Self::Input,
    ) -> Self::LabelWidget {
        Self::default_make_label(_info)
        // Label::dynamic(|data: &Broker, _: &Env| {
        //     debug!("data.name={}", data.name);
        //     format!("{}", data.name)
        // })
        // .lens(BrokerIndex(_key))
        // if let Some(tabs) = _data.brokers.iter().find(|x| (*x).id == _key) {
        //     debug!("{}", tabs.name);
        //     return Label::new(tabs.name.as_str());
        // }
        // unreachable!()
    }
}
