use crate::data::common::{Broker, Protocol, SubscribeHis};
use crate::data::hierarchy::AppData;
use crate::data::lens::{
    BrokerStoredList, LensSelectedSubscribeHis, LensSubscribeHisQoS, SubscribeHisPayloadLens,
};
use crate::data::AppEvent;
use crate::ui::auto_scroll::AutoScrollController;
use crate::ui::common::{
    label_dy, label_dy_expand_width, label_static, label_static_expand_width, svg, title,
    RightClickToCopy, QOS, SILVER, TOPIC,
};
use crate::ui::icons::{added_icon, connect_icon, copy_icon, modified_icon, removed_icon};
use crate::ui::ids::SCROLL_SUBSCRIBE_ID;
use crate::ui::payload_ty::payload_ty_init;
use crate::ui::qos::qos_init;
use crossbeam_channel::Sender;
use druid::im::Vector;
use druid::theme::{BORDER_LIGHT, TEXTBOX_BORDER_WIDTH};
use druid::widget::Svg;
use druid::widget::{
    Button, Container, CrossAxisAlignment, Either, Flex, Label, List, Padding, Scroll, Split,
};
use druid::{Env, EventCtx, UnitPoint};
use druid::{Widget, WidgetExt};
use log::error;

pub fn init_broker_list(tx: Sender<AppEvent>) -> impl Widget<AppData> {
    Split::rows(init_connect(tx.clone()), init_subscribe_his_list(tx))
        .split_point(0.55)
        .draggable(true)
        .bar_size(3.0)
        .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH)
        .padding(5.0)
}

fn init_subscribe_his_list(tx: Sender<AppEvent>) -> impl Widget<AppData> {
    let his_fn = move || {
        let tx_click = tx.clone();
        Flex::row()
            .with_child(qos_init(LensSubscribeHisQoS))
            .with_child(payload_ty_init(SubscribeHisPayloadLens))
            .with_child(TOPIC().lens(SubscribeHis::topic))
            .expand_width()
            .on_click(move |_ctx, data: &mut SubscribeHis, _env| {
                if let Err(_) = tx_click.send(AppEvent::ClickSubscribeHis(data.clone())) {
                    error!("fail to send event")
                }
            })
    };

    let list: List<SubscribeHis> = List::new(move || {
        Either::new(
            |data: &SubscribeHis, _env| data.selected,
            his_fn().background(SILVER),
            his_fn(),
        )
    });
    let scroll = Scroll::<Vector<SubscribeHis>, List<SubscribeHis>>::new(list)
        .vertical()
        .lens(LensSelectedSubscribeHis);
    let buttons = Flex::row()
        // .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_flex_child(
            title("Subscribe History", UnitPoint::LEFT).expand_width(),
            1.0,
        )
        .with_child(
            svg(removed_icon()).on_click(move |_ctx, data: &mut AppData, _env| {
                if let Err(_) = data.db.tx.send(AppEvent::RemoveSubscribeHis) {
                    error!("fail to send event")
                }
            }),
        )
        .with_child(
            svg(connect_icon()).on_click(move |_ctx, data: &mut AppData, _env| {
                if let Some(his) = data.get_selected_subscribe_his() {
                    if let Err(_) = data.db.tx.send(AppEvent::SubscribeFromHis(his)) {
                        error!("fail to send event");
                    }
                }
            }),
        );

    let flex = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    let flex = flex
        .with_child(
            buttons
                .expand_width()
                .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH),
        )
        .with_flex_child(scroll.expand_width(), 1.0);
    flex
}

pub fn init_connect(_tx: Sender<AppEvent>) -> Flex<AppData> {
    let version = || {
        label_dy(|data: &Broker, _: &Env| match data.protocol {
            Protocol::V4 => "v3".to_string(),
            Protocol::V5 => "v5".to_string(),
        })
    };
    let name = || label_dy(|data: &Broker, _: &Env| format!("{}", data.name));
    let addr = || {
        label_dy_expand_width(|data: &Broker, _: &Env| match data.port {
            None => {
                format!("{}", data.addr)
            }
            Some(port) => format!("{}:{:?}", data.addr, port),
        })
    };

    let list: List<Broker> = List::new(move || {
        Either::new(
            |data: &Broker, _env| data.selected,
            Flex::row()
                .with_child(version().fix_width(20.0))
                .with_child(name())
                .with_flex_child(addr(), 1.0)
                .on_click(|_ctx: &mut EventCtx, data: &mut Broker, _env: &Env| {
                    if let Err(_e) = data.tx.send(AppEvent::ClickBroker(data.id)) {
                        error!("fail to send");
                    }
                })
                .background(SILVER),
            Flex::row()
                .with_child(version().fix_width(20.0))
                .with_child(name())
                .with_flex_child(addr(), 1.0)
                .on_click(|_ctx: &mut EventCtx, data: &mut Broker, _env: &Env| {
                    if let Err(_e) = data.tx.send(AppEvent::ClickBroker(data.id)) {
                        error!("fail to send");
                    }
                }),
        )
    });
    let scroll = Scroll::<Vector<Broker>, List<Broker>>::new(list);

    let buttons = Flex::row()
        // .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_flex_child(title("Broker List", UnitPoint::LEFT).expand_width(), 1.0)
        .with_child(
            svg(added_icon()).on_click(move |_ctx, data: &mut AppData, _env| {
                if let Err(_) = data.db.tx.send(AppEvent::AddBroker) {
                    error!("fail to send event")
                }
            }),
        )
        .with_child(
            svg(modified_icon()).on_click(move |_ctx, data: &mut AppData, _env| {
                if let Err(_) = data.db.tx.send(AppEvent::EditBroker) {
                    error!("fail to send event")
                }
            }),
        )
        .with_child(
            svg(removed_icon()).on_click(move |_ctx, data: &mut AppData, _env| {
                if let Err(_) = data.db.tx.send(AppEvent::DeleteBroker) {
                    error!("fail to send event")
                }
            }),
        )
        .with_child(
            svg(connect_icon()).on_click(move |_ctx, data: &mut AppData, _env| {
                if let Err(_) = data.db.tx.send(AppEvent::ConnectBroker) {
                    error!("fail to send event")
                }
            }),
        );

    let flex = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    let flex = flex
        // .with_child(label_static("Broker List", UnitPoint::LEFT))
        .with_child(
            buttons
                // .fix_height(16.0)
                .expand_width()
                // .padding(2.0)
                .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH),
        )
        .with_flex_child(scroll.vertical().expand().lens(BrokerStoredList), 1.0);
    flex
}
