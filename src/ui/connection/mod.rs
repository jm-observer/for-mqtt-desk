use crate::data::click_ty::ClickTy;
use crate::data::common::{Broker, Msg, PublicInput, SubscribeInput, SubscribeTopic};

use crate::data::lens::{
    LensQoSAString, MsgMsgLens, MsgPayloadTyLens, MsgQosLens, MsgTimeLens, MsgTopicLens,
    SubscribeTopicPayloadLens,
};
use crate::data::{AString, AppEvent};
use crate::ui::auto_scroll::AutoScrollController;
use crate::ui::common::{
    error_display_widget, label_static, svg, RightClickToCopy, BUTTON_PADDING, B_BOXTEXT,
};

use crate::ui::icons::removed_icon;
use crate::ui::ids::{
    CLEAR_ERROR, ID_PUBLISH_MSG, ID_PUBLISH_QOS, ID_PUBLISH_TOPIC, ID_SUBSCRIBE_QOS,
    ID_SUBSCRIBE_TOPIC, SCROLL_MSG_ID, SCROLL_SUBSCRIBE_ID, SHOW_ERROR,
};
use crate::ui::payload_ty::{down_select_payload_ty, payload_ty_init};
use crate::ui::qos::{down_select_qos, qos_init, qos_success};
use crate::ForError;

use crossbeam_channel::Sender;
use druid::im::Vector;
use druid::text::{EditableText, ValidationError};
use druid::theme::{BORDER_LIGHT, TEXTBOX_BORDER_WIDTH};
use druid::widget::{Align, Button, Container, Either, Flex, List, Scroll, Split, TextBox};
use druid::{LensExt, LocalizedString};
use druid::{UnitPoint, Widget, WidgetExt};
use log::{error, warn};

pub fn display_connection(tx: Sender<AppEvent>) -> Container<Broker> {
    let subscribe_list = Container::new(
        init_subscribe_list(tx.clone()), // Split::rows(init_subscribe_list(id), init_subscribe_his_list(id, tx))
                                         //     .split_point(0.75)
                                         //     .bar_size(1.0),
    )
    .rounded(8.0)
    .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH)
    .padding(0.5);
    let subscribe = Container::new(
        Split::rows(subscribe_list, init_subscribe_input(tx.clone()))
            .split_point(0.65)
            .bar_size(1.0),
    )
    .rounded(8.0)
    .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH)
    .padding(1.0);

    let msg = Container::new(
        Split::rows(
            Align::centered(init_msgs_list(tx.clone())),
            Align::centered(init_public_input(tx.clone())),
        )
        .split_point(0.65)
        .bar_size(1.0),
    )
    .rounded(8.0)
    .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH)
    .padding(1.0);
    Container::new(
        Split::columns(subscribe, msg)
            .split_point(0.3)
            .draggable(true)
            .bar_size(0.5),
    )
    // .debug_paint_layout()
}

fn init_subscribe_list(tx: Sender<AppEvent>) -> impl Widget<Broker> {
    let list: List<SubscribeTopic> = List::new(move || {
        let tx = tx.clone();
        let tx1 = tx.clone();
        Flex::row()
            .with_child(svg(removed_icon()).on_click(
                move |_ctx, data: &mut SubscribeTopic, _env| {
                    if let Err(_) = tx.send(AppEvent::TouchUnSubscribe {
                        broker_id: data.broker_id,
                        trace_id: data.trace_id,
                    }) {
                        error!("fail to send event")
                    }
                },
            ))
            .with_child(Either::new(
                |data: &SubscribeTopic, _env| data.is_sucess(),
                qos_success(SubscribeTopic::qos.then(LensQoSAString)),
                qos_init(SubscribeTopic::qos.then(LensQoSAString)),
            ))
            .with_child(payload_ty_init(SubscribeTopicPayloadLens))
            .with_child(
                TextBox::new()
                    .controller(RightClickToCopy)
                    .disabled_if(|_, _| true)
                    .lens(SubscribeTopic::topic)
                    .fix_width(150.0),
            )
            .align_left()
            // .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH)
            .expand_width()
            .on_click(move |_ctx, data: &mut SubscribeTopic, _env| {
                if let Err(_) = tx1.send(AppEvent::TouchClick(ClickTy::SubscribeTopic(
                    data.broker_id,
                    data.trace_id,
                ))) {
                    error!("fail to send event")
                }
            })
    });

    let scroll = Scroll::<Vector<SubscribeTopic>, List<SubscribeTopic>>::new(list)
        .vertical()
        .controller(AutoScrollController)
        .with_id(SCROLL_SUBSCRIBE_ID)
        .lens(Broker::subscribe_topics)
        .align_vertical(UnitPoint::TOP)
        .expand_width()
        .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH);
    scroll
}

fn init_msgs_list(tx: Sender<AppEvent>) -> impl Widget<Broker> {
    let list: List<Msg> = List::new(move || {
        Either::new(
            |data: &Msg, _env| data.is_public(),
            Flex::row()
                .with_child(
                    Container::new(
                        Flex::column()
                            .with_child(
                                TextBox::new()
                                    .expand_width()
                                    .lens(MsgTimeLens)
                                    .padding(1.5)
                                    .disabled_if(|_, _| true),
                            )
                            .with_child(
                                Flex::row()
                                    .with_child(Either::new(
                                        |data: &Msg, _env| data.is_sucess(),
                                        qos_success(MsgQosLens),
                                        qos_init(MsgQosLens),
                                    ))
                                    .with_child(payload_ty_init(MsgPayloadTyLens))
                                    .with_flex_child(
                                        TextBox::<AString>::new()
                                            .controller(RightClickToCopy)
                                            .disabled_if(|_, _| true)
                                            .lens(MsgTopicLens)
                                            .expand_width(),
                                        1.0,
                                    )
                                    .expand_width(),
                            )
                            .with_child(
                                TextBox::multiline()
                                    .controller(RightClickToCopy)
                                    .disabled_if(|_, _| true)
                                    // .fix_height(50.0)
                                    .expand_width()
                                    .lens(MsgMsgLens)
                                    .padding(1.5),
                            )
                            .fix_width(250.),
                    )
                    .rounded(8.0)
                    .border(BORDER_LIGHT, 1.0),
                )
                .align_horizontal(UnitPoint::TOP_RIGHT)
                .expand_width(),
            Flex::row()
                .with_child(
                    Container::new(
                        Flex::column()
                            .with_child(
                                TextBox::new()
                                    .expand_width()
                                    .lens(MsgTimeLens)
                                    .padding(1.5)
                                    .disabled_if(|_, _| true),
                            )
                            .with_child(
                                Flex::row()
                                    .with_child(qos_success(MsgQosLens))
                                    .with_child(payload_ty_init(MsgPayloadTyLens))
                                    .with_flex_child(
                                        TextBox::<AString>::new()
                                            .controller(RightClickToCopy)
                                            .disabled_if(|_, _| true)
                                            .lens(MsgTopicLens)
                                            .expand_width(),
                                        1.0,
                                    )
                                    .expand_width(),
                            )
                            .with_child(
                                TextBox::multiline()
                                    .controller(RightClickToCopy)
                                    .disabled_if(|_, _| true)
                                    .expand_width()
                                    .lens(MsgMsgLens)
                                    .padding(1.5),
                            ),
                    )
                    .rounded(8.0)
                    .border(BORDER_LIGHT, 1.0)
                    .fix_width(250.),
                )
                .align_horizontal(UnitPoint::TOP_LEFT)
                .expand_width(),
        )
        .expand_width()
        .padding(5.0)
        .align_horizontal(UnitPoint::LEFT)
    });
    let scroll = Scroll::<Vector<Msg>, List<Msg>>::new(list)
        .vertical()
        .controller(AutoScrollController)
        .with_id(SCROLL_MSG_ID)
        .lens(Broker::msgs)
        .align_vertical(UnitPoint::TOP)
        .expand_width()
        .expand_height()
        .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH);
    let clear_tx = tx.clone();
    let tools = Flex::row()
        .with_child(
            Button::new("Clear").on_click(move |_, data: &mut Broker, _| {
                if clear_tx.send(AppEvent::TouchClearMsg(data.id)).is_err() {
                    error!("could not to send clear command");
                }
            }),
        )
        .align_left();
    Flex::column()
        .with_child(tools)
        .with_flex_child(scroll, 1.0)
}

//
fn init_subscribe_input(tx: Sender<AppEvent>) -> impl Widget<Broker> {
    let subscribe_tx = tx.clone();
    let connection = Flex::column()
        .with_child(
            Flex::row()
                .with_child(label_static("topic", UnitPoint::RIGHT))
                .with_child(
                    TextBox::new()
                        .lens(Broker::subscribe_input.then(SubscribeInput::topic))
                        .fix_width(150.),
                )
                .with_child(error_display_widget(ID_SUBSCRIBE_TOPIC))
                .align_left(),
        )
        .with_child(
            Flex::row()
                .with_child(label_static("QoS", UnitPoint::RIGHT))
                .with_child(
                    down_select_qos()
                        .lens(Broker::subscribe_input.then(SubscribeInput::qos))
                        .fix_width(150.),
                )
                .with_child(error_display_widget(ID_SUBSCRIBE_QOS))
                .align_left(),
        )
        .with_child(
            Flex::row()
                .with_child(label_static("Byte Type", UnitPoint::RIGHT))
                .with_child(
                    down_select_payload_ty()
                        .lens(Broker::subscribe_input.then(SubscribeInput::payload_ty)),
                )
                .align_left(),
        )
        .with_child(
            Flex::row().with_child(
                Button::new(LocalizedString::new("Subscribe"))
                    .on_click(move |ctx, data: &mut Broker, _env| {
                        if data.subscribe_input.topic.is_empty() {
                            warn!("topic is empty");
                            ctx.submit_command(
                                SHOW_ERROR
                                    .with(ValidationError::new(ForError::NotEmpty))
                                    .to(ID_SUBSCRIBE_TOPIC),
                            );
                            return;
                        }
                        ctx.submit_command(CLEAR_ERROR.to(ID_SUBSCRIBE_TOPIC));
                        if let Err(e) = subscribe_tx.send(AppEvent::TouchSubscribeByInput(data.id))
                        {
                            error!("{:?}", e);
                        }
                    })
                    .disabled_if(|data: &Broker, _env| !data.tab_status.connected)
                    .padding(BUTTON_PADDING),
            ),
        );
    connection
}

fn init_public_input(tx: Sender<AppEvent>) -> impl Widget<Broker> {
    let public_tx = tx.clone();
    let connection = Flex::column()
        .with_child(
            Flex::row()
                .with_child(label_static("topic", UnitPoint::RIGHT))
                .with_child(
                    TextBox::new()
                        .lens(Broker::public_input.then(PublicInput::topic))
                        .fix_width(300.)
                        .background(B_BOXTEXT),
                )
                .with_child(error_display_widget(ID_PUBLISH_TOPIC))
                .align_left(),
        )
        .with_child(
            Flex::row()
                .with_child(label_static("QoS", UnitPoint::RIGHT))
                .with_child(
                    down_select_qos()
                        .lens(Broker::public_input.then(PublicInput::qos))
                        .fix_width(300.),
                )
                .with_child(error_display_widget(ID_PUBLISH_QOS))
                .align_left(),
        )
        .with_child(
            Flex::row()
                .with_child(label_static("Byte Type", UnitPoint::RIGHT))
                .with_child(
                    down_select_payload_ty()
                        .lens(Broker::public_input.then(PublicInput::payload_ty)),
                )
                // .with_child(error_display_widget(ID_PUBLISH_QOS))
                .align_left(),
        )
        .with_child(
            Flex::row()
                .with_child(label_static("msg", UnitPoint::RIGHT))
                .with_child(
                    TextBox::multiline()
                        .fix_height(60.)
                        .fix_width(300.)
                        .lens(Broker::public_input.then(PublicInput::msg)),
                )
                .with_child(error_display_widget(ID_PUBLISH_MSG))
                .align_left(),
        )
        .with_child(
            Flex::row().with_child(
                Button::new(LocalizedString::new("Publish"))
                    .on_click(move |ctx, broker: &mut Broker, _env| {
                        if broker.public_input.topic.is_empty()
                            || broker.public_input.msg.is_empty()
                        {
                            warn!("topic or msg is empty");
                            if broker.public_input.topic.is_empty() {
                                ctx.submit_command(
                                    SHOW_ERROR
                                        .with(ValidationError::new(ForError::NotEmpty))
                                        .to(ID_PUBLISH_TOPIC),
                                );
                            } else {
                                ctx.submit_command(CLEAR_ERROR.to(ID_PUBLISH_TOPIC));
                            }
                            if broker.public_input.msg.is_empty() {
                                ctx.submit_command(
                                    SHOW_ERROR
                                        .with(ValidationError::new(ForError::NotEmpty))
                                        .to(ID_PUBLISH_MSG),
                                );
                            } else {
                                ctx.submit_command(CLEAR_ERROR.to(ID_PUBLISH_MSG));
                            }
                            return;
                        }
                        ctx.submit_command(CLEAR_ERROR.to(ID_PUBLISH_TOPIC));
                        ctx.submit_command(CLEAR_ERROR.to(ID_PUBLISH_MSG));
                        if let Err(e) = public_tx.send(AppEvent::TouchPublic(broker.id)) {
                            error!("{:?}", e);
                        }
                    })
                    .disabled_if(|broker: &Broker, _env| !broker.tab_status.connected)
                    .padding(BUTTON_PADDING),
            ),
        );
    connection
}
