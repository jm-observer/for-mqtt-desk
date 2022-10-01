use crate::data::common::{TabKind, TabStatus};
use crate::data::hierarchy::AppData;
use crate::data::lens::BrokerIndexLensTabStatus;
use crate::data::AppEvent;
use crate::ui::broker_info::display_broker;
use crate::ui::common::{GREEN, RED, YELLOW};
use crate::ui::connection::display_connection;
use druid::widget::{Either, Label, TabInfo, TabsPolicy};
use druid::{Data, Env, Widget, WidgetExt};
use log::error;
use std::sync::mpsc::Sender;

#[derive(Clone)]
pub struct BrokerTabPolicy(pub usize, pub Sender<AppEvent>);
impl Data for BrokerTabPolicy {
    fn same(&self, _other: &Self) -> bool {
        true
    }
}
impl TabsPolicy for BrokerTabPolicy {
    type Key = TabKind;
    type Build = ();
    type Input = AppData;
    type LabelWidget = impl Widget<AppData>;
    type BodyWidget = impl Widget<AppData>;

    fn tabs_changed(&self, old_data: &Self::Input, data: &Self::Input) -> bool {
        if let (Some(old_status), Some(status)) = (
            old_data.tab_statuses.get(&self.0),
            data.tab_statuses.get(&self.0),
        ) {
            old_status.try_connect != status.try_connect
        } else {
            false
        }
    }

    fn tabs(&self, data: &AppData) -> Vec<Self::Key> {
        let mut keys = Vec::with_capacity(2);
        if let Some(status) = data.tab_statuses.get(&self.0) {
            if status.try_connect || status.connected {
                keys.push(TabKind::Connection);
            }
        }
        keys.push(TabKind::Broker);
        keys
    }

    fn tab_info(&self, key: Self::Key, _data: &AppData) -> TabInfo<AppData> {
        match key {
            TabKind::Connection => TabInfo::new(format!("Connection"), true),
            TabKind::Broker => TabInfo::new(format!("Options"), false),
        }
    }

    fn tab_body(&self, key: Self::Key, _data: &AppData) -> Self::BodyWidget {
        match key {
            TabKind::Connection => {
                display_connection(self.0)
                // debug_label_appdata()
            }
            TabKind::Broker => {
                // debug_label_appdata()
                display_broker(self.0)
            }
        }
    }

    fn close_tab(&self, _key: Self::Key, _data: &mut AppData) {
        if let Err(_) = _data.db.tx.send(AppEvent::CloseConnectionTab(self.0)) {
            error!("fail to send event")
        }
    }

    fn tab_label(
        &self,
        _key: Self::Key,
        _info: TabInfo<Self::Input>,
        _data: &Self::Input,
    ) -> Self::LabelWidget {
        match _key {
            TabKind::Connection => Either::new(
                |status: &TabStatus, _: &Env| status.connected,
                Label::new("Connected").background(GREEN),
                Either::new(
                    |status: &TabStatus, _: &Env| status.try_connect,
                    Label::new("Connecting").background(YELLOW),
                    Label::new("Disconnection").background(RED),
                ),
            )
            .lens(BrokerIndexLensTabStatus(self.0)),
            TabKind::Broker => Either::new(|_, _| true, Label::new("Option"), Label::new("Option"))
                .lens(BrokerIndexLensTabStatus(self.0)),
        }
    }
}
