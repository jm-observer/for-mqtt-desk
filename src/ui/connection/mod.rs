use crate::data::common::{Msg, PublicInput, SubscribeHis, SubscribeInput, SubscribeTopic};
use crate::data::hierarchy::AppData;
use crate::data::lens::{BrokerIndex, DbIndex, Index};
use crate::data::AppEvent;
use crate::ui::common::label_static;
use druid::im::Vector;
use druid::theme::{BORDER_LIGHT, TEXTBOX_BORDER_WIDTH};
use druid::widget::{
    Align, Button, Container, CrossAxisAlignment, Flex, Label, List, Padding, Scroll, Split,
    TextBox,
};
use druid::LensExt;
use druid::{Env, UnitPoint, Widget, WidgetExt};
use log::{debug, error};

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
        .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH),
    );
    let subscribe = Padding::new(
        10.0,
        Container::new(
            Split::rows(subscribe_list, Align::centered(init_subscribe_input(id)))
                .split_point(0.75)
                .bar_size(3.0),
        )
        .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH),
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
        .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH),
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
        Label::dynamic(|data: &SubscribeTopic, _: &Env| {
            debug!("{:?}", data);
            format!("{}", data.topic)
        })
        .align_vertical(UnitPoint::LEFT)
        .fix_width(20f64)
    };
    let qos = || {
        Label::dynamic(|data: &SubscribeTopic, _: &Env| format!("{:?}", data.qos))
            .align_vertical(UnitPoint::LEFT)
            .fix_width(20f64)
    };
    let status = || {
        Label::dynamic(|data: &SubscribeTopic, _: &Env| format!("{:?}", data.status))
            .align_vertical(UnitPoint::LEFT)
            .fix_width(20f64)
    };

    let list: List<SubscribeTopic> = List::new(move || {
        Flex::row()
            .with_flex_child(topic(), 1.0)
            .with_child(qos())
            .with_child(status())
    });

    let scroll = Scroll::<Vector<SubscribeTopic>, List<SubscribeTopic>>::new(list);

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
        Label::dynamic(|data: &Msg, _: &Env| {
            debug!("{:?}", data);
            format!("{}", data.topic())
        })
        .align_vertical(UnitPoint::LEFT)
        .fix_width(20f64)
    };
    let qos = || {
        Label::dynamic(|data: &Msg, _: &Env| format!("{:?}", data.qos()))
            .align_vertical(UnitPoint::LEFT)
            .fix_width(20f64)
    };
    let msg = || {
        Label::dynamic(|data: &Msg, _: &Env| format!("{:?}", data.msg()))
            .align_vertical(UnitPoint::LEFT)
            .fix_width(20f64)
    };

    let list: List<Msg> = List::new(move || {
        Flex::row()
            .with_flex_child(topic(), 1.0)
            .with_child(qos())
            .with_child(msg())
    });

    let scroll = Scroll::<Vector<Msg>, List<Msg>>::new(list);

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
        Label::dynamic(|data: &SubscribeHis, _: &Env| {
            debug!("{:?}", data);
            format!("{}", data.topic)
        })
        .align_vertical(UnitPoint::LEFT)
        .fix_width(20f64)
    };
    let qos = || {
        Label::dynamic(|data: &SubscribeHis, _: &Env| format!("{:?}", data.qos))
            .align_vertical(UnitPoint::LEFT)
            .fix_width(20f64)
    };

    let list: List<SubscribeHis> =
        List::new(move || Flex::row().with_flex_child(topic(), 1.0).with_child(qos()));

    let scroll = Scroll::<Vector<SubscribeHis>, List<SubscribeHis>>::new(list);

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
    let connection = Flex::column()
        .with_child(
            Flex::row()
                .with_child(label_static("topic"))
                .with_child(TextBox::new().lens(BrokerIndex(id).then(SubscribeInput::topic)))
                .align_left(),
        )
        .with_child(
            Flex::row()
                .with_child(label_static("qos"))
                .with_child(TextBox::new().lens(BrokerIndex(id).then(SubscribeInput::qos)))
                .align_left(),
        )
        .with_child(
            Flex::row().with_child(
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
    let connection = Flex::column()
        .with_child(
            Flex::row()
                .with_child(label_static("topic"))
                .with_child(TextBox::new().lens(BrokerIndex(id).then(PublicInput::topic)))
                .align_left(),
        )
        .with_child(
            Flex::row()
                .with_child(label_static("qos"))
                .with_child(TextBox::new().lens(BrokerIndex(id).then(PublicInput::qos)))
                .align_left(),
        )
        .with_child(
            Flex::row()
                .with_child(label_static("msg"))
                .with_child(TextBox::new().lens(BrokerIndex(id).then(PublicInput::msg)))
                .align_left(),
        )
        .with_child(
            Flex::row().with_child(
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
