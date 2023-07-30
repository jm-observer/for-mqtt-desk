use crate::data::click_ty::ClickTy;
use crate::data::hierarchy::AppData;
use crate::data::lens::BrokerIdForTab;
use crate::data::localized::Locale;
use crate::data::AppEvent;
use crate::ui::common::{GREEN, RED};
use crate::ui::connection::display_connection;
use crossbeam_channel::Sender;
use druid::widget::{Either, Label, TabInfo, TabsPolicy};
use druid::{Color, Data, Env};
use druid::{Widget, WidgetExt};
use log::{debug, error};

#[derive(Clone)]
pub struct BrokersTabs(pub Sender<AppEvent>, pub Locale);

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
        check_data(data, old_data)
    }

    fn tabs(&self, data: &Self::Input) -> Vec<Self::Key> {
        data.broker_tabs.iter().map(|x| *x).collect()
    }

    fn tab_info(&self, key: Self::Key, _data: &Self::Input) -> TabInfo<Self::Input> {
        return TabInfo::new(
            move |data: &AppData, _: &Env| {
                if let Some(tabs) = data.brokers.iter().find(|x| (*x).id == key) {
                    format!("{:width$}", tabs.name, width = 40)
                } else {
                    format!("{:width$}", "", width = 40)
                }
            },
            true,
        );
    }

    fn tab_body(&self, _key: Self::Key, _data: &Self::Input) -> Self::BodyWidget {
        debug!("tab_body");
        display_connection(self.0.clone(), self.1.clone()).lens(BrokerIdForTab(_key))
    }

    fn close_tab(&self, key: Self::Key, data: &mut Self::Input) {
        if let Err(_) = data.db.tx.send(AppEvent::TouchCloseBrokerTab(key)) {
            error!("fail to send event");
        }
    }

    fn tab_label(
        &self,
        key: Self::Key,
        _info: TabInfo<Self::Input>,
        _data: &Self::Input,
    ) -> Self::LabelWidget {
        let lable = if let Some(tabs) = _data.brokers.iter().find(|x| (*x).id == key) {
            tabs.name.as_str().to_string()
        } else {
            "".to_string()
        };
        let tx = self.0.clone();
        let lable = || {
            let tx = tx.clone();
            Label::new(lable.clone())
                .fix_width(80.0)
                .on_click(move |_, _, _| {
                    if tx
                        .send(AppEvent::TouchClick(ClickTy::ConnectTab(key)))
                        .is_err()
                    {
                        error!("fail to send event");
                    }
                })
        };

        Either::new(
            move |data: &AppData, _env| {
                if let Ok(broker) = data.find_broker_by_id(key) {
                    broker.tab_status.connected
                } else {
                    false
                }
            },
            lable()
                .background(Color::rgb8(0xc1, 0xff, 0xc1))
                .rounded(3.0),
            lable()
                .background(Color::rgb8(0xFF, 0xff, 0xc1))
                .rounded(3.0),
        )
    }
}

fn check_data(data: &AppData, old_data: &AppData) -> bool {
    if !(data.broker_tabs == old_data.broker_tabs) {
        return true;
    }
    for id in &old_data.broker_tabs {
        if let Some(old_broker) = old_data.brokers.iter().find(|x| x.id == *id) {
            if let Some(broker) = data.brokers.iter().find(|x| x.id == *id) {
                if !old_broker.same(broker) {
                    return true;
                } else {
                    continue;
                }
            }
        }
        return true;
    }
    false
}
