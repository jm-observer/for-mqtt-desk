use crate::data::common::{Msg, PublicInput, QoS, SubscribeInput, SubscribeTopic};
use crate::data::hierarchy::AppData;
use crate::data::lens::{
    BrokerIndexLensPublicInput, BrokerIndexLensSubscribeInput, BrokerIndexLensVecMsg,
    BrokerIndexLensVecSubscribeTopic, DbIndex, Index, MsgMsgLens, MsgQosLens, MsgTopicLens,
};
use crate::data::{AString, AppEvent};
use crate::ui::common::{
    error_display_widget, label_static, svg, BUTTON_PADDING, GREEN, MSG, QOS, SILVER, TOPIC, YELLOW,
};
use crate::ui::formatter::{check_no_empty, check_qos, MustInput};
use crate::ui::icons::removed_icon;
use crate::ui::ids::{
    TextBoxErrorDelegate, ID_PUBLISH_MSG, ID_PUBLISH_QOS, ID_PUBLISH_TOPIC, ID_SUBSCRIBE_QOS,
    ID_SUBSCRIBE_TOPIC,
};
use crossbeam_channel::Sender;
use druid::im::Vector;
use druid::text::EditableText;
use druid::theme::{BORDER_LIGHT, TEXTBOX_BORDER_WIDTH};
use druid::widget::{
    Align, Button, Container, CrossAxisAlignment, Either, Flex, List, Padding, Scroll, Split, Svg,
    TextBox,
};
use druid::{LensExt, LocalizedString};
use druid::{UnitPoint, Widget, WidgetExt};
use log::{debug, error};

pub fn display_connection(id: usize, tx: Sender<AppEvent>) -> Container<AppData> {
    let subscribe_list = Padding::new(
        0.5,
        Container::new(
            init_subscribe_list(id, tx.clone()), // Split::rows(init_subscribe_list(id), init_subscribe_his_list(id, tx))
                                                 //     .split_point(0.75)
                                                 //     .bar_size(1.0),
        ), // .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH),
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
        0.5,
        Container::new(
            Split::rows(
                Align::centered(init_msgs_list(id, tx.clone())),
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
            .draggable(true)
            .bar_size(0.5),
    )
    // .debug_paint_layout()
}

fn init_subscribe_list(id: usize, tx: Sender<AppEvent>) -> impl Widget<AppData> {
    let list: List<SubscribeTopic> = List::new(move || {
        let tx = tx.clone();
        Flex::row()
            .with_child(svg(removed_icon()).on_click(
                move |_ctx, data: &mut SubscribeTopic, _env| {
                    if let Err(_) = tx.send(AppEvent::ToUnSubscribe {
                        broker_id: id,
                        pk_id: data.trace_id,
                    }) {
                        error!("fail to send event")
                    }
                },
            ))
            .with_child(Either::new(
                |data: &SubscribeTopic, _env| data.is_sucess(),
                QOS().background(GREEN).lens(SubscribeTopic::qos),
                QOS().background(YELLOW).lens(SubscribeTopic::qos),
            ))
            .with_child(TextBox::new().lens(SubscribeTopic::topic).fix_width(150.0))
            .align_left()
            // .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH)
            .expand_width()
    });

    let scroll = Scroll::<Vector<SubscribeTopic>, List<SubscribeTopic>>::new(list)
        .vertical()
        .lens(BrokerIndexLensVecSubscribeTopic(id))
        .align_vertical(UnitPoint::TOP)
        .expand_width()
        .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH);
    scroll
}

fn init_msgs_list(id: usize, tx: Sender<AppEvent>) -> impl Widget<AppData> {
    let list: List<Msg> = List::new(move || {
        Either::new(
            |data: &Msg, _env| data.is_public(),
            Flex::row()
                .with_child(
                    Flex::column()
                        .with_child(
                            Flex::row()
                                .with_child(
                                    TextBox::<String>::new()
                                        .fix_width(15.0)
                                        .padding(1.0)
                                        .lens(MsgQosLens),
                                )
                                .with_flex_child(
                                    Either::new(
                                        |data: &Msg, _env| data.is_sucess(),
                                        TextBox::<AString>::new()
                                            .background(GREEN)
                                            .lens(MsgTopicLens),
                                        TextBox::<AString>::new()
                                            .background(SILVER)
                                            .lens(MsgTopicLens),
                                    )
                                    .expand_width(),
                                    1.0,
                                )
                                .expand_width(),
                        )
                        .with_child(
                            TextBox::multiline()
                                // .fix_height(50.0)
                                .expand_width()
                                .lens(MsgMsgLens)
                                .padding(1.5),
                        )
                        .border(BORDER_LIGHT, 1.0)
                        .fix_width(250.),
                )
                .align_horizontal(UnitPoint::TOP_RIGHT)
                .expand_width(),
            Flex::column()
                .with_child(
                    Flex::row()
                        .with_child(QOS().background(GREEN).lens(MsgTopicLens))
                        .with_child(TextBox::<AString>::new().lens(MsgTopicLens))
                        .align_horizontal(UnitPoint::LEFT),
                )
                .with_child(
                    TextBox::multiline()
                        .lens(MsgMsgLens)
                        .align_horizontal(UnitPoint::LEFT),
                )
                .fix_width(200.),
        )
        .expand_width()
        .padding(5.0)
        .align_horizontal(UnitPoint::LEFT)
    });
    let scroll = Scroll::<Vector<Msg>, List<Msg>>::new(list)
        .vertical()
        .lens(BrokerIndexLensVecMsg(id))
        .align_vertical(UnitPoint::TOP)
        .expand_width()
        .expand_height()
        .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH);
    let clear_tx = tx.clone();
    let tools = Flex::row()
        .with_child(Button::new("Clear").on_click(move |_, _, _| {
            if clear_tx.send(AppEvent::ClearMsg(id)).is_err() {
                error!("could not to send clear command");
            }
        }))
        .with_child(Button::new("hex").on_click(move |_, _, _| {}))
        .align_left();
    Flex::column()
        .with_child(tools)
        .with_flex_child(scroll, 1.0)
}

//
fn init_subscribe_input(id: usize) -> impl Widget<AppData> {
    let connection = Flex::column()
        .with_child(
            Flex::row()
                .with_child(label_static("topic", UnitPoint::RIGHT))
                .with_child(
                    TextBox::new()
                        .with_formatter(MustInput)
                        .update_data_while_editing(true)
                        .validate_while_editing(true)
                        .delegate(
                            TextBoxErrorDelegate::new(ID_SUBSCRIBE_TOPIC, check_no_empty)
                                .sends_partial_errors(true),
                        )
                        .lens(BrokerIndexLensSubscribeInput(id).then(SubscribeInput::topic))
                        .fix_width(150.),
                )
                .with_child(error_display_widget(ID_SUBSCRIBE_TOPIC))
                .align_left(),
        )
        .with_child(
            Flex::row()
                .with_child(label_static("qos", UnitPoint::RIGHT))
                .with_child(
                    TextBox::new()
                        .with_placeholder("0/1/2")
                        .with_formatter(MustInput)
                        .update_data_while_editing(true)
                        .validate_while_editing(true)
                        .delegate(
                            TextBoxErrorDelegate::new(ID_SUBSCRIBE_QOS, check_qos)
                                .sends_partial_errors(true),
                        )
                        .lens(BrokerIndexLensSubscribeInput(id).then(SubscribeInput::qos))
                        .fix_width(150.),
                )
                .with_child(error_display_widget(ID_SUBSCRIBE_QOS))
                .align_left(),
        )
        .with_child(
            Flex::row().with_child(
                Button::new(LocalizedString::new("Subscribe"))
                    .on_click(move |_ctx, data: &mut DbIndex, _env| {
                        debug!("{:?}", data.data);
                        if let Some(input) = data.data.subscribe_input.get(&data.id) {
                            if input.topic.is_empty() {
                                debug!("topic is empty");
                                return;
                            }
                            if let Err(e) = data
                                .data
                                .db
                                .tx
                                .send(AppEvent::Subscribe(input.clone(), data.id))
                            {
                                error!("{:?}", e);
                            }
                        } else {
                            error!("can't get the broker");
                        }
                    })
                    .disabled_if(|data: &DbIndex, _env| {
                        if let Some(broker) = data.data.tab_statuses.get(&data.id) {
                            !broker.connected
                        } else {
                            true
                        }
                    })
                    .padding(BUTTON_PADDING)
                    .lens(Index(id)),
            ),
        );
    connection
}

fn init_public_input(id: usize) -> impl Widget<AppData> {
    let connection = Flex::column()
        .with_child(
            Flex::row()
                .with_child(label_static("topic", UnitPoint::RIGHT))
                .with_child(
                    TextBox::new()
                        .with_formatter(MustInput)
                        .update_data_while_editing(true)
                        .validate_while_editing(true)
                        .delegate(
                            TextBoxErrorDelegate::new(ID_PUBLISH_TOPIC, check_no_empty)
                                .sends_partial_errors(true),
                        )
                        .lens(BrokerIndexLensPublicInput(id).then(PublicInput::topic))
                        .fix_width(300.),
                )
                .with_child(error_display_widget(ID_PUBLISH_TOPIC))
                .align_left(),
        )
        .with_child(
            Flex::row()
                .with_child(label_static("qos", UnitPoint::RIGHT))
                .with_child(
                    TextBox::new()
                        .with_placeholder("0/1/2")
                        .with_formatter(MustInput)
                        .validate_while_editing(false)
                        .delegate(
                            TextBoxErrorDelegate::new(ID_PUBLISH_QOS, check_qos)
                                .sends_partial_errors(true),
                        )
                        .lens(BrokerIndexLensPublicInput(id).then(PublicInput::qos))
                        .fix_width(300.),
                )
                .with_child(error_display_widget(ID_PUBLISH_QOS))
                .align_left(),
        )
        .with_child(
            Flex::row()
                .with_child(label_static("msg", UnitPoint::RIGHT))
                .with_child(
                    TextBox::multiline()
                        .with_formatter(MustInput)
                        .update_data_while_editing(true)
                        .validate_while_editing(true)
                        .delegate(
                            TextBoxErrorDelegate::new(ID_PUBLISH_MSG, check_no_empty)
                                .sends_partial_errors(true),
                        )
                        .fix_height(60.)
                        .fix_width(300.)
                        .lens(BrokerIndexLensPublicInput(id).then(PublicInput::msg)),
                )
                .with_child(error_display_widget(ID_PUBLISH_MSG))
                .align_left(),
        )
        .with_child(
            Flex::row().with_child(
                Button::new(LocalizedString::new("Publish"))
                    .on_click(move |_ctx, data: &mut DbIndex, _env| {
                        if let Some(broker) = data.data.public_input.get(&data.id) {
                            if broker.topic.is_empty() || broker.msg.is_empty() {
                                debug!("topic or msg is empty");
                                return;
                            }
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
                    .disabled_if(|data: &DbIndex, _env| {
                        if let Some(broker) = data.data.tab_statuses.get(&data.id) {
                            !broker.connected
                        } else {
                            true
                        }
                    })
                    .padding(BUTTON_PADDING)
                    .lens(Index(id)),
            ),
        );
    connection
}
