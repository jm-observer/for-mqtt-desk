use crate::data::common::Broker;
use crate::data::hierarchy::AppData;
use crate::data::lens::BrokerIndex;
use crate::data::AppEvent;
use crate::ui::common::{error_display_widget, label_static, TEXTBOX_MULTI_WIDTH, TEXTBOX_WIDTH};
use crate::ui::formatter::{check_addr, check_no_empty, check_port, MustInput};
use crate::ui::ids::{
    TextBoxErrorDelegate, ID_ADDR, ID_BUTTON_CONNECT, ID_BUTTON_RECONNECT, ID_CLIENT_ID, ID_PORT,
};
use crate::util::general_id;
use druid::widget::{Container, Either, Flex, TextBox};
use druid::{Env, LensExt};
use druid::{LocalizedString, WidgetExt};
use log::{debug, error};

pub fn display_broker(id: usize) -> Container<AppData> {
    let connection = Flex::column()
        .with_child(
            Flex::row()
                .with_child(label_static("name"))
                .with_child(
                    TextBox::new()
                        .fix_width(TEXTBOX_WIDTH)
                        .lens(BrokerIndex(id).then(Broker::name)),
                )
                .align_left(),
        )
        .with_child(
            Flex::row()
                .with_child(label_static("client id"))
                .with_child(
                    TextBox::new()
                        .fix_width(TEXTBOX_WIDTH)
                        .lens(BrokerIndex(id).then(Broker::client_id)),
                )
                .align_left(),
        )
        .with_child(
            Flex::row()
                .with_child(label_static("addr"))
                .with_child(
                    TextBox::new()
                        .with_formatter(MustInput)
                        .validate_while_editing(false)
                        .update_data_while_editing(true)
                        .delegate(
                            TextBoxErrorDelegate::new(ID_ADDR, check_no_empty)
                                .sends_partial_errors(true),
                        )
                        .fix_width(TEXTBOX_WIDTH)
                        .lens(BrokerIndex(id).then(Broker::addr)),
                )
                .with_child(error_display_widget(ID_ADDR))
                .align_left(),
        )
        .with_child(
            Flex::row()
                .with_child(label_static("port"))
                .with_child(
                    TextBox::new()
                        .with_formatter(MustInput)
                        .validate_while_editing(false)
                        .delegate(
                            TextBoxErrorDelegate::new(ID_PORT, check_port)
                                .sends_partial_errors(true),
                        )
                        .fix_width(TEXTBOX_WIDTH)
                        .lens(BrokerIndex(id).then(Broker::port)),
                )
                .with_child(error_display_widget(ID_PORT))
                .align_left(),
        )
        .with_child(Either::new(
            move |data: &AppData, _: &Env| {
                if let Some(broker) = data.tab_statuses.get(&id) {
                    broker.connected
                } else {
                    false
                }
            },
            Flex::row()
                .with_child(label_static(LocalizedString::new("button-save")).on_click(
                    move |_ctx, data: &mut AppData, _env| {
                        if let Err(e) = data.db.tx.send(AppEvent::SaveBroker(id)) {
                            error!("{:?}", e);
                        }
                    },
                ))
                .with_child(
                    label_static(LocalizedString::new("button-reconnect")).on_click(
                        move |_ctx, data: &mut AppData, _env| {
                            _ctx.set_focus(ID_BUTTON_RECONNECT);
                            if let Err(e) = data.db.tx.send(AppEvent::ReConnect(id)) {
                                error!("{:?}", e);
                            }
                        },
                    ),
                )
                .with_child(
                    label_static(LocalizedString::new("button-disconnect")).on_click(
                        move |_ctx, data: &mut AppData, _env| {
                            if let Err(e) = data.db.tx.send(AppEvent::Disconnect(id)) {
                                error!("{:?}", e);
                            }
                        },
                    ),
                )
                .align_left(),
            Flex::row()
                .with_child(label_static(LocalizedString::new("button-save")).on_click(
                    move |_ctx, data: &mut AppData, _env| {
                        if let Err(e) = data.db.tx.send(AppEvent::SaveBroker(id)) {
                            error!("{:?}", e);
                        }
                    },
                ))
                .with_child(
                    label_static(LocalizedString::new("button-connect")).on_click(
                        move |_ctx, data: &mut AppData, _env| {
                            if let Some(broker) = data.brokers.iter_mut().find(|x| x.id == id) {
                                debug!("{:?}", broker);
                                _ctx.set_focus(ID_BUTTON_CONNECT);
                                if broker.client_id.as_str().is_empty() {
                                    broker.client_id = general_id().into();
                                }
                                if let Err(e) = data.db.tx.send(AppEvent::Connect(broker.clone())) {
                                    error!("{:?}", e);
                                }
                            } else {
                                error!("can't get the broker");
                            }
                        },
                    ),
                )
                .align_left(),
        ))
        .with_child(
            Flex::row()
                .with_child(label_static("params"))
                .with_flex_child(
                    TextBox::multiline()
                        // .with_placeholder("Multi")
                        .lens(BrokerIndex(id).then(Broker::params))
                        .fix_height(180.)
                        .fix_width(TEXTBOX_MULTI_WIDTH),
                    1.0,
                )
                .align_left(),
        );
    Container::new(connection)
}
