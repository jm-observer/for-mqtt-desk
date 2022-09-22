use crate::data::db::Broker;
use crate::data::hierarchy::AppData;
use crate::data::lens::{BrokerIndex, DbIndex, Index};
use crate::data::AppEvent;
use crate::ui::common::label_static;
use druid::widget::{Container, Flex, TextBox};
use druid::LensExt;
use druid::WidgetExt;
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
        .with_child(
            Flex::row()
                .with_child(
                    label_static("连接").on_click(move |_ctx, data: &mut DbIndex, _env| {
                        if let Some(broker) = data.data.brokers.get(data.index) {
                            if let Err(e) = data.data.db.tx.send(AppEvent::Connect(broker.clone()))
                            {
                                error!("{:?}", e);
                            }
                        } else {
                            error!("can't get the broker");
                        }
                    }),
                )
                .lens(Index(id))
                .align_left(),
        )
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
