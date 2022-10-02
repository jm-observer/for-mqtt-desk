use crate::data::common::Broker;
use crate::data::hierarchy::AppData;
use crate::data::lens::BrokerIndex;
use crate::data::AppEvent;
use crate::ui::common::label_static;
use druid::widget::{Container, Either, Flex, TextBox};
use druid::WidgetExt;
use druid::{Env, LensExt};
use log::error;

pub fn display_broker(id: usize) -> Container<AppData> {
    let connection = Flex::column()
        .with_child(
            Flex::row()
                .with_child(label_static("name"))
                .with_child(TextBox::new().lens(BrokerIndex(id).then(Broker::name)))
                .align_left(),
        )
        .with_child(
            Flex::row()
                .with_child(label_static("client id"))
                .with_child(TextBox::new().lens(BrokerIndex(id).then(Broker::client_id)))
                .align_left(),
        )
        .with_child(
            Flex::row()
                .with_child(label_static("addr"))
                .with_child(TextBox::new().lens(BrokerIndex(id).then(Broker::addr)))
                .align_left(),
        )
        .with_child(
            Flex::row()
                .with_child(label_static("port"))
                .with_child(TextBox::new().lens(BrokerIndex(id).then(Broker::port)))
                .align_left(),
        )
        .with_child(Either::new(
            move |data: &AppData, _: &Env| {
                if let Some(broker) = data.tab_statuses.get(&id) {
                    broker.connected
                } else {
                    false
                }
            },
            Flex::row()
                .with_child(
                    label_static("保存").on_click(move |_ctx, data: &mut AppData, _env| {
                        if let Err(e) = data.db.tx.send(AppEvent::SaveBroker(id)) {
                            error!("{:?}", e);
                        }
                    }),
                )
                .with_child(
                    label_static("重连").on_click(move |_ctx, data: &mut AppData, _env| {
                        if let Err(e) = data.db.tx.send(AppEvent::ReConnect(id)) {
                            error!("{:?}", e);
                        }
                    }),
                )
                .with_child(
                    label_static("断开").on_click(move |_ctx, data: &mut AppData, _env| {
                        if let Err(e) = data.db.tx.send(AppEvent::Disconnect(id)) {
                            error!("{:?}", e);
                        }
                    }),
                )
                .align_left(),
            Flex::row()
                .with_child(
                    label_static("保存").on_click(move |_ctx, data: &mut AppData, _env| {
                        if let Err(e) = data.db.tx.send(AppEvent::SaveBroker(id)) {
                            error!("{:?}", e);
                        }
                    }),
                )
                .with_child(
                    label_static("连接").on_click(move |_ctx, data: &mut AppData, _env| {
                        if let Some(broker) = data.brokers.iter().find(|x| x.id == id) {
                            if let Err(e) = data.db.tx.send(AppEvent::Connect(broker.clone())) {
                                error!("{:?}", e);
                            }
                        } else {
                            error!("can't get the broker");
                        }
                    }),
                )
                .align_left(),
        ))
        .with_child(
            Flex::row()
                .with_child(label_static("params"))
                .with_flex_child(
                    TextBox::multiline()
                        .with_placeholder("Multi")
                        .lens(BrokerIndex(id).then(Broker::params))
                        .fix_height(180.)
                        .expand_width(),
                    1.0,
                )
                .align_left(),
        );
    Container::new(connection)
}
