use crate::data::click_ty::ClickTy;
use crate::data::common::{Broker, Protocol, SubscribeHis};
use crate::data::hierarchy::AppData;
use crate::data::lens::{
    BrokerSelectedOrZero, BrokerStoredList, LensSubscribeHisQoS, SubscribeHisPayloadLens,
};
use crate::data::AppEvent;

use crate::ui::common::{svg, title, topic, QOS_COMMON, SILVER};
use crate::ui::icons::{added_icon, connect_icon, modified_icon, removed_icon};

use crate::ui::payload_ty::payload_ty_init;

use crossbeam_channel::Sender;
use druid::im::Vector;
use druid::theme::{BORDER_LIGHT, TEXTBOX_BORDER_WIDTH};
use druid::widget::{Container, CrossAxisAlignment, Either, Flex, List, Scroll, Split};
use druid::widget::{Label, SizedBox};
use druid::{Env, EventCtx, UnitPoint};
use druid::{Widget, WidgetExt};
use log::error;

pub fn init_broker_list(tx: Sender<AppEvent>) -> impl Widget<AppData> {
    Split::rows(
        Container::new(init_broker_list_1(tx.clone()))
            .rounded(8.0)
            .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH)
            .expand_height(),
        Either::<AppData>::new(
            |x, _env| {
                if let Ok(broker) = x.get_selected_broker() {
                    broker.stored
                } else {
                    false
                }
            },
            Container::new(init_subscribe_his_list(tx.clone()).lens(BrokerSelectedOrZero))
                .rounded(8.0)
                .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH),
            SizedBox::empty(),
        ),
    )
    .split_point(0.55)
    .bar_size(0.0)
    .draggable(true)
}

fn init_subscribe_his_list(tx: Sender<AppEvent>) -> impl Widget<Broker> {
    let tx_click = tx.clone();
    let his_fn = move || {
        let tx_click = tx_click.clone();
        Flex::row()
            .with_child(QOS_COMMON().lens(LensSubscribeHisQoS))
            .with_child(payload_ty_init(SubscribeHisPayloadLens))
            .with_child(topic().lens(SubscribeHis::topic))
            .expand_width()
            .on_click(move |_ctx, data: &mut SubscribeHis, _env| {
                if let Err(_) =
                    tx_click.send(AppEvent::TouchClick(ClickTy::SubscribeHis(data.clone())))
                {
                    error!("fail to send event")
                }
                if tx_click.send(AppEvent::TouchClickBrokerList).is_err() {
                    error!("fail to send event");
                }
            })
            .padding(1.0)
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
        .lens(Broker::subscribe_hises);

    let tx_removed_icon = tx.clone();
    let tx_connect_icon = tx.clone();
    let buttons = Flex::<Broker>::row()
        // .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_flex_child(
            title("Subscribe History", UnitPoint::LEFT).expand_width(),
            1.0,
        )
        .with_child(
            svg(removed_icon()).on_click(move |_ctx, data: &mut Broker, _env| {
                if let Err(_) = tx_removed_icon.send(AppEvent::TouchRemoveSubscribeHis(data.id)) {
                    error!("fail to send event")
                }
                if tx_removed_icon
                    .send(AppEvent::TouchClickBrokerList)
                    .is_err()
                {
                    error!("fail to send event");
                }
            }),
        )
        .with_child(
            svg(connect_icon()).on_click(move |_ctx, data: &mut Broker, _env| {
                if let Some(his) = data.subscribe_hises.iter().find(|x| x.selected) {
                    if let Err(_) =
                        tx_connect_icon.send(AppEvent::TouchSubscribeFromHis(his.clone()))
                    {
                        error!("fail to send event");
                    }
                    if tx_connect_icon
                        .send(AppEvent::TouchClickBrokerList)
                        .is_err()
                    {
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

pub fn init_broker_list_1(_tx: Sender<AppEvent>) -> Flex<AppData> {
    let version = || {
        Label::dynamic(|data: &Broker, _: &Env| match data.protocol {
            Protocol::V4 => "v3".to_string(),
            Protocol::V5 => "v5".to_string(),
        })
        .fix_width(24.0)
        .padding((2.0, 5.0))
        .border(SILVER, 0.1)
    };
    let name = || {
        Label::dynamic(|data: &Broker, _: &Env| format!("{}", data.name))
            .padding((2.0, 5.0))
            .align_vertical(UnitPoint::LEFT)
            .fix_width(80.0)
            .border(SILVER, 0.1)
    };
    let addr = || {
        Label::dynamic(|data: &Broker, _: &Env| match data.port {
            None => {
                format!("{}", data.addr)
            }
            Some(port) => format!("{}:{:?}", data.addr, port),
        })
        .align_vertical(UnitPoint::LEFT)
        .expand_width()
        .padding((2.0, 5.0))
        .border(SILVER, 0.1)
    };

    let list: List<Broker> = List::new(move || {
        Either::new(
            |data: &Broker, _env| data.selected,
            Flex::row()
                .with_child(version())
                .with_child(name())
                .with_flex_child(addr(), 1.0)
                .on_click(|_ctx: &mut EventCtx, data: &mut Broker, _env: &Env| {
                    if let Err(_e) = data.tx.send(AppEvent::TouchClick(ClickTy::Broker(data.id))) {
                        error!("fail to send");
                    }
                    if data.tx.send(AppEvent::TouchClickBrokerList).is_err() {
                        error!("fail to send event");
                    }
                })
                .background(SILVER),
            Flex::row()
                .with_child(version())
                .with_child(name())
                .with_flex_child(addr(), 1.0)
                .on_click(|_ctx: &mut EventCtx, data: &mut Broker, _env: &Env| {
                    if let Err(_e) = data.tx.send(AppEvent::TouchClick(ClickTy::Broker(data.id))) {
                        error!("fail to send");
                    }
                    if data.tx.send(AppEvent::TouchClickBrokerList).is_err() {
                        error!("fail to send event");
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
                if let Err(_) = data.db.tx.send(AppEvent::TouchAddBroker) {
                    error!("fail to send event")
                }
                if data.tx.send(AppEvent::TouchClickBrokerList).is_err() {
                    error!("fail to send event");
                }
            }),
        )
        .with_child(
            svg(modified_icon()).on_click(move |_ctx, data: &mut AppData, _env| {
                if let Err(_) = data.db.tx.send(AppEvent::TouchEditBrokerSelected) {
                    error!("fail to send event")
                }
                if data.tx.send(AppEvent::TouchClickBrokerList).is_err() {
                    error!("fail to send event");
                }
            }),
        )
        .with_child(
            svg(removed_icon()).on_click(move |_ctx, data: &mut AppData, _env| {
                if let Err(_) = data.db.tx.send(AppEvent::TouchDeleteBrokerSelected) {
                    error!("fail to send event")
                }
                if data.tx.send(AppEvent::TouchClickBrokerList).is_err() {
                    error!("fail to send event");
                }
            }),
        )
        .with_child(
            svg(connect_icon()).on_click(move |_ctx, data: &mut AppData, _env| {
                if let Err(_) = data.db.tx.send(AppEvent::TouchConnectBrokerSelected) {
                    error!("fail to send event")
                }
                if data.tx.send(AppEvent::TouchClickBrokerList).is_err() {
                    error!("fail to send event");
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
