use crate::data::common::TabKind;
use crate::data::hierarchy::AppData;
use crate::ui::broker_info::display_broker;
use crate::ui::connection::display_connection;
use druid::widget::{TabInfo, TabsPolicy};
use druid::{Data, Widget};

#[derive(Data, Clone)]
pub struct BrokerTabPolicy(pub usize);

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
            if status.try_connect {
                keys.push(TabKind::Connection);
            }
        }
        keys.push(TabKind::Broker);
        keys
    }

    fn tab_info(&self, key: Self::Key, _data: &AppData) -> TabInfo<AppData> {
        match key {
            TabKind::Connection => TabInfo::new(format!("Connection"), true),
            TabKind::Broker => TabInfo::new(format!("Broker"), false),
        }
    }

    fn tab_body(&self, key: Self::Key, _data: &AppData) -> Self::BodyWidget {
        match key {
            TabKind::Connection => display_connection(self.0),
            TabKind::Broker => display_broker(self.0),
        }
    }

    fn close_tab(&self, _key: Self::Key, _data: &mut AppData) {
        todo!()
    }

    fn tab_label(
        &self,
        _key: Self::Key,
        info: TabInfo<Self::Input>,
        _data: &Self::Input,
    ) -> Self::LabelWidget {
        Self::default_make_label(info)
    }
}
