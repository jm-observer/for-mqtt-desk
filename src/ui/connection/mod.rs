use crate::data::common::{Msg, PublicInput, SubscribeHis, SubscribeInput, SubscribeTopic};
use crate::data::hierarchy::AppData;
use crate::data::lens::{BrokerIndex, DbIndex, Index};
use crate::data::AppEvent;
use druid::im::Vector;
use druid::widget::{
    Align, Button, Container, CrossAxisAlignment, Flex, Label, List, Padding, Scroll, Split,
    TextBox,
};
use druid::LensExt;
use druid::{Color, Env, UnitPoint, Widget, WidgetExt};
use log::{debug, error};
use std::sync::Arc;

pub fn display_connection(id: usize) -> Container<AppData> {
    let subscribe_list = Padding::new(
        10.0,
        Container::new(
            Split::rows(
                Align::centered(init_subscribe_list(id)),
                Align::centered(init_subscribe_his_list(id)),
            )
            .split_point(0.75)
            .bar_size(3.0),
        )
        .border(Color::WHITE, 1.0),
    );
    let subscribe = Padding::new(
        10.0,
        Container::new(
            Split::rows(subscribe_list, Align::centered(init_subscribe_input(id)))
                .split_point(0.75)
                .bar_size(3.0),
        )
        .border(Color::WHITE, 1.0),
    );

    let msg = Padding::new(
        10.0,
        Container::new(
            Split::rows(
                Align::centered(init_msgs_list(id)),
                Align::centered(init_public_input(id)),
            )
            .split_point(0.75)
            .bar_size(3.0),
        )
        .border(Color::WHITE, 1.0),
    );
    Container::new(
        Split::columns(subscribe, msg)
            .split_point(0.2)
            .draggable(true),
    )
    // .debug_paint_layout()
}

fn init_subscribe_list(id: usize) -> impl Widget<AppData> {
    let topic = || {
        Label::dynamic(|data: &Arc<SubscribeTopic>, _: &Env| {
            debug!("{:?}", data);
            format!("{}", data.topic)
        })
        .align_vertical(UnitPoint::LEFT)
        .fix_width(20f64)
    };
    let qos = || {
        Label::dynamic(|data: &Arc<SubscribeTopic>, _: &Env| format!("{:?}", data.qos))
            .align_vertical(UnitPoint::LEFT)
            .fix_width(20f64)
    };
    let status = || {
        Label::dynamic(|data: &Arc<SubscribeTopic>, _: &Env| format!("{:?}", data.status))
            .align_vertical(UnitPoint::LEFT)
            .fix_width(20f64)
    };

    let list: List<Arc<SubscribeTopic>> = List::new(move || {
        Flex::row()
            .with_flex_child(topic(), 1.0)
            .with_child(qos())
            .with_child(status())
    });

    let scroll = Scroll::<Vector<Arc<SubscribeTopic>>, List<Arc<SubscribeTopic>>>::new(list);

    let flex = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    let flex = flex.with_child(
        scroll
            .vertical()
            .fix_height(200.0)
            .fix_width(300.0)
            .lens(BrokerIndex(id)),
    );
    flex
}

fn init_msgs_list(id: usize) -> impl Widget<AppData> {
    let topic = || {
        Label::dynamic(|data: &Arc<Msg>, _: &Env| {
            debug!("{:?}", data);
            format!("{}", data.topic())
        })
        .align_vertical(UnitPoint::LEFT)
        .fix_width(20f64)
    };
    let qos = || {
        Label::dynamic(|data: &Arc<Msg>, _: &Env| format!("{:?}", data.qos()))
            .align_vertical(UnitPoint::LEFT)
            .fix_width(20f64)
    };
    let msg = || {
        Label::dynamic(|data: &Arc<Msg>, _: &Env| format!("{:?}", data.msg()))
            .align_vertical(UnitPoint::LEFT)
            .fix_width(20f64)
    };

    let list: List<Arc<Msg>> = List::new(move || {
        Flex::row()
            .with_flex_child(topic(), 1.0)
            .with_child(qos())
            .with_child(msg())
    });

    let scroll = Scroll::<Vector<Arc<Msg>>, List<Arc<Msg>>>::new(list);

    let flex = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    let flex = flex.with_child(
        scroll
            .vertical()
            .fix_height(200.0)
            .fix_width(300.0)
            .lens(BrokerIndex(id)),
    );
    flex
}

fn init_subscribe_his_list(id: usize) -> impl Widget<AppData> {
    let topic = || {
        Label::dynamic(|data: &Arc<SubscribeHis>, _: &Env| {
            debug!("{:?}", data);
            format!("{}", data.topic)
        })
        .align_vertical(UnitPoint::LEFT)
        .fix_width(20f64)
    };
    let qos = || {
        Label::dynamic(|data: &Arc<SubscribeHis>, _: &Env| format!("{:?}", data.qos))
            .align_vertical(UnitPoint::LEFT)
            .fix_width(20f64)
    };

    let list: List<Arc<SubscribeHis>> =
        List::new(move || Flex::row().with_flex_child(topic(), 1.0).with_child(qos()));

    let scroll = Scroll::<Vector<Arc<SubscribeHis>>, List<Arc<SubscribeHis>>>::new(list);

    let flex = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    let flex = flex.with_child(
        scroll
            .vertical()
            .fix_height(200.0)
            .fix_width(300.0)
            .lens(BrokerIndex(id)),
    );
    flex
}

//
pub fn init_subscribe_input(id: usize) -> Container<AppData> {
    let connection = Flex::row()
        .with_child(
            Flex::column()
                .with_child(Label::new("topic").fix_width(80.0))
                .with_child(Label::new("qos").fix_width(80.0)),
        )
        .with_child(
            Flex::column()
                .with_child(
                    TextBox::new().lens(BrokerIndex(id).then(SubscribeInput::topic.in_arc())),
                )
                .with_child(TextBox::new().lens(BrokerIndex(id).then(SubscribeInput::qos.in_arc())))
                .with_child(
                    Button::new("订阅")
                        .on_click(move |_ctx, data: &mut DbIndex, _env| {
                            if let Some(broker) = data.data.subscribe_ing.get(&data.index) {
                                if let Err(e) = data
                                    .data
                                    .db
                                    .tx
                                    .send(AppEvent::Subscribe(broker.clone(), data.index))
                                {
                                    error!("{:?}", e);
                                }
                            } else {
                                error!("can't get the broker");
                            }
                        })
                        .lens(Index(id)),
                ),
        );
    Container::new(connection)
}

pub fn init_public_input(id: usize) -> Container<AppData> {
    let connection = Flex::row()
        .with_child(
            Flex::column()
                .with_child(Label::new("topic").fix_width(80.0))
                .with_child(Label::new("qos").fix_width(80.0))
                .with_child(Label::new("msg").fix_width(80.0)),
        )
        .with_child(
            Flex::column()
                .with_child(TextBox::new().lens(BrokerIndex(id).then(PublicInput::topic.in_arc())))
                .with_child(TextBox::new().lens(BrokerIndex(id).then(PublicInput::msg.in_arc())))
                .with_child(TextBox::new().lens(BrokerIndex(id).then(PublicInput::qos.in_arc())))
                .with_child(
                    Button::new("发布")
                        .on_click(move |_ctx, data: &mut DbIndex, _env| {
                            if let Some(broker) = data.data.public_ing.get(&data.index) {
                                if let Err(e) = data
                                    .data
                                    .db
                                    .tx
                                    .send(AppEvent::Public(broker.clone(), data.index))
                                {
                                    error!("{:?}", e);
                                }
                            } else {
                                error!("can't get the broker");
                            }
                        })
                        .lens(Index(id)),
                ),
        );
    Container::new(connection)
}
