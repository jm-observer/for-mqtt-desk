use crate::data::common::Broker;
use crate::data::common::TabStatus;
use crate::data::hierarchy::AppData;
use crate::data::lens::BrokerStoredList;
use crate::data::AppEvent;
use crate::ui::common::{label_dy, label_dy_expand_width, SILVER};
use druid::im::Vector;
use druid::widget::{Button, CrossAxisAlignment, Either, Flex, Label, List, Scroll};
use druid::WidgetExt;
use druid::{Env, EventCtx};
use log::{debug, error};

pub fn init_connect() -> Flex<AppData> {
    let name = || label_dy(|data: &Broker, _: &Env| format!("{}", data.name));
    let addr =
        || label_dy_expand_width(|data: &Broker, _: &Env| format!("{}:{}", data.addr, data.port));

    let list: List<Broker> = List::new(move || {
        Either::new(
            |data: &Broker, _env| data.selected,
            Flex::row()
                .with_child(name())
                .with_flex_child(addr(), 1.0)
                .on_click(|_ctx: &mut EventCtx, data: &mut Broker, _env: &Env| {
                    if let Err(e) = data.tx.send(AppEvent::ClickBroker(data.id)) {
                        error!("fail to send");
                    }
                })
                .background(SILVER),
            Flex::row()
                .with_child(name())
                .with_flex_child(addr(), 1.0)
                .on_click(|_ctx: &mut EventCtx, data: &mut Broker, _env: &Env| {
                    if let Err(e) = data.tx.send(AppEvent::ClickBroker(data.id)) {
                        error!("fail to send");
                    }
                }),
        )
    });
    let scroll = Scroll::<Vector<Broker>, List<Broker>>::new(list);

    let buttons = Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("新增").with_text_size(12.).on_click(
            move |_ctx, data: &mut AppData, _env| {
                let broker = data.db.new_broker();
                debug!("{:?}", broker);
                data.broker_tabs.push_front(broker.id);
                data.tab_statuses.insert(
                    broker.id,
                    TabStatus {
                        id: broker.id,
                        try_connect: false,
                        connected: false,
                        db: data.db.clone(),
                    },
                );
                data.brokers.push_back(broker.into());
            },
        ))
        .with_child(Button::new("删"))
        .with_child(Button::new("复制"));

    let flex = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    let flex = flex
        .with_child(buttons)
        .with_flex_child(scroll.vertical().expand().lens(BrokerStoredList), 1.0);
    flex
}
