use crate::data::db::Broker;
use crate::data::hierarchy::AppData;
use crate::data::lens::{BrokerIndex, DbIndex, Index};
use crate::data::AppEvent;
use druid::widget::{Container, Flex, Label, TextBox};
use druid::LensExt;
use druid::WidgetExt;
use log::error;

pub fn display_broker(id: usize) -> Container<AppData> {
    let connection = Flex::column()
        .with_child(
            Flex::row()
                .with_child(Label::new("name").fix_width(80.0))
                .with_child(TextBox::new().lens(BrokerIndex(id).then(Broker::name.in_arc()))),
        )
        .with_child(
            Flex::row()
                .with_child(Label::new("client id").fix_width(80.0))
                .with_child(TextBox::new().lens(BrokerIndex(id).then(Broker::client_id.in_arc()))),
        )
        .with_child(
            Flex::row()
                .with_child(Label::new("addr").fix_width(80.0))
                .with_child(TextBox::new().lens(BrokerIndex(id).then(Broker::addr.in_arc()))),
        )
        .with_child(
            Flex::row()
                .with_child(Label::new("port").fix_width(80.0))
                .with_child(TextBox::new().lens(BrokerIndex(id).then(Broker::port.in_arc()))),
        )
        .with_child(
            Flex::row()
                .with_child(Label::new("连接").with_text_size(12.).on_click(
                    move |_ctx, data: &mut DbIndex, _env| {
                        if let Some(broker) = data.data.brokers.get(data.index) {
                            if let Err(e) = data.data.db.tx.send(AppEvent::Connect(broker.clone()))
                            {
                                error!("{:?}", e);
                            }
                        } else {
                            error!("can't get the broker");
                        }
                    },
                ))
                .lens(Index(id)),
        )
        .with_child(
            Flex::row()
                .with_child(Label::new("params").fix_width(50.0))
                .with_flex_child(
                    TextBox::multiline()
                        .with_placeholder("Multi")
                        .lens(BrokerIndex(id).then(Broker::params.in_arc()))
                        .fix_height(100.)
                        .expand_width(),
                    1.0,
                ),
        );
    Container::new(connection)
}
