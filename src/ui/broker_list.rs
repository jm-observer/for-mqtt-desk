use crate::data::common::TabStatus;
use crate::data::db::Broker;
use crate::data::hierarchy::AppData;
use crate::data::lens::BrokerStoredList;
use druid::im::Vector;
use druid::widget::{Button, CrossAxisAlignment, Flex, Label, List, Scroll};
use druid::{theme, Env, EventCtx};
use druid::{UnitPoint, WidgetExt};
use log::debug;

pub fn init_connect() -> Flex<AppData> {
    let name = || {
        Label::dynamic(|data: &Broker, _: &Env| format!("{}", data.name))
            .align_vertical(UnitPoint::LEFT)
            .padding(1.0)
        // .fix_width(80f64)
    };
    let addr = || {
        Label::dynamic(|data: &Broker, _: &Env| format!("{}:{}", data.addr, data.port))
            .align_vertical(UnitPoint::LEFT)
            .padding(1.0)
            .expand_width()
    };

    let list: List<Broker> = List::new(move || {
        Flex::row()
            .with_child(name())
            .with_flex_child(addr(), 1.0)
            .on_click(|_ctx: &mut EventCtx, data: &mut Broker, _env: &Env| {
                debug!("onlick: {}", data.id)
            })
    });

    let scroll = Scroll::<Vector<Broker>, List<Broker>>::new(list);

    let buttons = Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(
            Label::new("新增")
                .with_text_size(12.)
                // .border(Color::WHITE, 10.)
                .on_click(move |_ctx, data: &mut AppData, _env| {
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
                }),
        )
        .with_child(Button::new("删"))
        .with_child(Button::new("复制"))
        .background(theme::PLACEHOLDER_COLOR);

    let flex = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    let flex = flex.with_child(buttons).with_child(
        scroll
            .vertical()
            .lens(BrokerStoredList)
            .fix_height(200.0)
            .fix_width(300.0),
    );
    flex
}
