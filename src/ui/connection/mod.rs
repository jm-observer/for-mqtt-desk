use crate::data::common::{Broker, Msg, PublicInput, SubscribeHis, SubscribeInput, SubscribeTopic};
use crate::data::hierarchy::AppData;
use crate::data::lens::{
    BrokerIndex, BrokerIndexLensPublicInput, BrokerIndexLensSubscribeInput, BrokerIndexLensVecMsg,
    BrokerIndexLensVecSubscribeHis, BrokerIndexLensVecSubscribeTopic, DbIndex, Index, MsgMsgLens,
    MsgTopicLens,
};
use crate::data::AppEvent;
use crate::ui::common::{label_static, GREEN, MSG, QOS, TOPIC, YELLOW};
use druid::im::Vector;
use druid::theme::{BORDER_LIGHT, TEXTBOX_BORDER_WIDTH};
use druid::widget::{
    Align, Button, Container, CrossAxisAlignment, Either, Flex, List, Padding, Scroll, Split,
    TextBox,
};
use druid::LensExt;
use druid::{UnitPoint, Widget, WidgetExt};
use log::error;

pub fn display_connection(id: usize) -> Container<AppData> {
    let subscribe_list = Padding::new(
        1.0,
        Container::new(
            Split::rows(init_subscribe_list(id), init_subscribe_his_list(id))
                .split_point(0.75)
                .bar_size(1.0),
        )
        .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH),
    );
    let subscribe = Padding::new(
        1.0,
        Container::new(
            Split::rows(subscribe_list, init_subscribe_input(id))
                .split_point(0.65)
                .bar_size(1.0),
        )
        .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH),
    );

    let msg = Padding::new(
        1.0,
        Container::new(
            Split::rows(
                Align::centered(init_msgs_list(id)),
                Align::centered(init_public_input(id)),
            )
            .split_point(0.65)
            .bar_size(1.0),
        )
        .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH),
    );
    Container::new(
        Split::columns(subscribe, msg)
            .split_point(0.3)
            .draggable(true),
    )
    // .debug_paint_layout()
}

fn init_subscribe_list(id: usize) -> impl Widget<AppData> {
    let list: List<SubscribeTopic> = List::new(move || {
        Flex::row()
            .with_child(Either::new(
                |data: &SubscribeTopic, _env| data.is_sucess(),
                QOS().background(GREEN).lens(SubscribeTopic::qos),
                QOS().background(YELLOW).lens(SubscribeTopic::qos),
            ))
            .with_child(TOPIC().lens(SubscribeTopic::topic))
            .align_left()
            .expand_width()
    });

    let scroll = Scroll::<Vector<SubscribeTopic>, List<SubscribeTopic>>::new(list)
        .vertical()
        .lens(BrokerIndexLensVecSubscribeTopic(id));
    scroll
    // let flex = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    // let flex = flex
    //     .with_child(scroll)
    //     .expand_width()
    //     .align_vertical(UnitPoint::TOP);
    // flex
}

fn init_subscribe_his_list(id: usize) -> impl Widget<AppData> {
    let list: List<SubscribeHis> = List::new(move || {
        Flex::row()
            .with_child(QOS().lens(SubscribeHis::qos))
            .with_child(TOPIC().lens(SubscribeHis::topic))
            .expand_width()
    });
    let scroll = Scroll::<Vector<SubscribeHis>, List<SubscribeHis>>::new(list)
        .vertical()
        .lens(BrokerIndexLensVecSubscribeHis(id));
    scroll

    // let flex = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    // let flex = flex
    //     .with_child(scroll.vertical().lens(BrokerIndex(id)))
    //     .align_vertical(UnitPoint::TOP);
    // flex
}

fn init_msgs_list(id: usize) -> impl Widget<AppData> {
    let list: List<Msg> = List::new(move || {
        Either::new(
            |data: &Msg, _env| data.is_public(),
            Flex::column()
                .with_child(
                    Flex::row()
                        .with_child(Either::new(
                            |data: &Msg, _env| data.is_sucess(),
                            QOS().background(GREEN).lens(MsgTopicLens),
                            QOS().background(YELLOW).lens(MsgTopicLens),
                        ))
                        .with_child(TOPIC().lens(MsgTopicLens))
                        .align_horizontal(UnitPoint::RIGHT),
                )
                .with_child(MSG().lens(MsgMsgLens).align_horizontal(UnitPoint::RIGHT)),
            Flex::column()
                .with_child(
                    Flex::row()
                        .with_child(QOS().background(GREEN).lens(MsgTopicLens))
                        .with_child(TOPIC().lens(MsgTopicLens))
                        .align_horizontal(UnitPoint::LEFT),
                )
                .with_child(MSG().lens(MsgMsgLens).align_horizontal(UnitPoint::LEFT)),
        )
        .fix_width(380.)
        .align_horizontal(UnitPoint::LEFT)
    });
    let scroll = Scroll::<Vector<Msg>, List<Msg>>::new(list);
    let flex = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    let flex = flex
        .with_child(scroll.vertical().lens(BrokerIndexLensVecMsg(id)))
        .align_vertical(UnitPoint::TOP)
        .expand_width()
        .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH);
    flex
}

//
pub fn init_subscribe_input(id: usize) -> impl Widget<AppData> {
    let connection = Flex::column()
        .with_child(
            Flex::row()
                .with_child(label_static("topic"))
                .with_child(
                    TextBox::new()
                        .lens(BrokerIndexLensSubscribeInput(id).then(SubscribeInput::topic)),
                )
                .align_left(),
        )
        .with_child(
            Flex::row()
                .with_child(label_static("qos"))
                .with_child(
                    TextBox::new()
                        .lens(BrokerIndexLensSubscribeInput(id).then(SubscribeInput::qos)),
                )
                .align_left(),
        )
        .with_child(
            Flex::row().with_child(
                Button::new("订阅")
                    .on_click(move |_ctx, data: &mut DbIndex, _env| {
                        if let Some(broker) = data.data.subscribe_ing.get(&data.id) {
                            if let Err(e) = data
                                .data
                                .db
                                .tx
                                .send(AppEvent::Subscribe(broker.clone(), data.id))
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
    connection
}

pub fn init_public_input(id: usize) -> impl Widget<AppData> {
    let connection = Flex::column()
        .with_child(
            Flex::row()
                .with_child(label_static("topic"))
                .with_child(
                    TextBox::new()
                        .lens(BrokerIndexLensPublicInput(id).then(PublicInput::topic))
                        .fix_width(300.),
                )
                .align_left(),
        )
        .with_child(
            Flex::row()
                .with_child(label_static("qos"))
                .with_child(
                    TextBox::new()
                        .lens(BrokerIndexLensPublicInput(id).then(PublicInput::qos))
                        .fix_width(300.),
                )
                .align_left(),
        )
        .with_child(
            Flex::row()
                .with_child(label_static("msg"))
                .with_child(
                    TextBox::multiline()
                        .fix_height(60.)
                        .fix_width(300.)
                        .lens(BrokerIndexLensPublicInput(id).then(PublicInput::msg)),
                )
                .align_left(),
        )
        .with_child(
            Flex::row().with_child(
                Button::new("发布")
                    .on_click(move |_ctx, data: &mut DbIndex, _env| {
                        if let Some(broker) = data.data.public_ing.get(&data.id) {
                            if let Err(e) = data
                                .data
                                .db
                                .tx
                                .send(AppEvent::Public(broker.clone(), data.id))
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
    connection
}
