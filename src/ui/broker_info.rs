use crate::data::common::{Broker, Protocol, SignedTy};
use crate::data::lens::PortLens;
use crate::data::AppEvent;
use crate::ui::common::{error_display_widget, label_static, BUTTON_PADDING, TEXTBOX_WIDTH};
use crate::ui::formatter::{check_port, MustInput};
use crate::ui::ids::{
    TextBoxErrorDelegate, ID_ADDR, ID_BUTTON_CONNECT, ID_BUTTON_RECONNECT, ID_PORT,
    SELF_SIGNED_FILE,
};

use crate::data::localized::Locale;
use crossbeam_channel::Sender;
use druid::widget::{Button, Either, Flex, RadioGroup, Switch, TextBox};
use druid::WidgetExt;
use druid::{Env, FileDialogOptions, FileSpec, UnitPoint, Widget};
use log::error;

pub fn display_broker(id: usize, tx: Sender<AppEvent>, locale: Locale) -> impl Widget<Broker> {
    let save_tx_0 = tx.clone();
    let _save_tx_1 = tx.clone();
    let connect_tx_1 = tx.clone();
    let disconnect_tx_1 = tx.clone();
    let save_tx_1 = tx.clone();
    let reconnect_tx_1 = tx.clone();

    Flex::column()
        .with_child(
            Flex::row()
                .with_child(label_static("name", UnitPoint::RIGHT))
                .with_child(TextBox::new().fix_width(TEXTBOX_WIDTH).lens(Broker::name))
                .align_left(),
        )
        .with_child(
            Flex::row()
                .with_child(label_static("client id", UnitPoint::RIGHT))
                .with_child(
                    TextBox::new()
                        .fix_width(TEXTBOX_WIDTH)
                        .lens(Broker::client_id),
                )
                .align_left(),
        )
        .with_child(
            Flex::row()
                .with_child(label_static("addr", UnitPoint::RIGHT))
                .with_child(TextBox::new().fix_width(TEXTBOX_WIDTH).lens(Broker::addr))
                .with_child(error_display_widget(ID_ADDR))
                .align_left(),
        )
        .with_child(
            Flex::row()
                .with_child(label_static("port", UnitPoint::RIGHT))
                .with_child(
                    TextBox::new()
                        .with_formatter(MustInput)
                        .update_data_while_editing(true)
                        .validate_while_editing(true)
                        .delegate(
                            TextBoxErrorDelegate::new(ID_PORT, check_port)
                                .sends_partial_errors(true),
                        )
                        .fix_width(TEXTBOX_WIDTH)
                        .lens(PortLens),
                )
                .with_child(error_display_widget(ID_PORT))
                .align_left(),
        )
        .with_child(
            Flex::row()
                .with_child(label_static("auto connect", UnitPoint::RIGHT))
                .with_child(Switch::new().lens(Broker::auto_connect))
                .align_left(),
        )
        .with_child(display_credential(id))
        .with_child(
            Flex::row()
                .with_child(label_static("version", UnitPoint::RIGHT))
                .with_child(
                    RadioGroup::row(vec![("v3", Protocol::V4), ("v5", Protocol::V5)])
                        .lens(Broker::protocol),
                )
                .align_left(),
        )
        .with_child(display_tls(id, locale.clone()))
        .with_child(Either::new(
            move |data: &Broker, _: &Env| data.tab_status.connected,
            Flex::row()
                .with_child(save_button(save_tx_0, locale.clone()))
                .with_child(reconnect_button(reconnect_tx_1, locale.clone()))
                .with_child(disconnect_button(disconnect_tx_1, locale.clone()))
                .align_left(),
            Flex::row()
                .with_child(save_button(save_tx_1, locale.clone()))
                .with_child(connect_button(connect_tx_1, locale.clone()))
                .align_left(),
        ))
        .with_flex_child(
            Flex::row()
                .with_child(label_static("params", UnitPoint::RIGHT).expand_height())
                .with_flex_child(
                    TextBox::multiline()
                        // .background(B_BOXTEXT)
                        // .with_placeholder("Multi")
                        .lens(Broker::params)
                        .expand_width()
                        .expand_height()
                        .align_left(),
                    1.0,
                )
                .expand_height(),
            1.0,
        )
        .padding(5.0)
        .expand_height()
}

pub fn display_tls(id: usize, locale: Locale) -> impl Widget<Broker> {
    Either::new(
        move |data: &Broker, _: &Env| data.tls,
        Flex::column()
            .with_child(
                Flex::row()
                    .with_child(label_static("tls", UnitPoint::RIGHT))
                    .with_child(Switch::new().lens(Broker::tls))
                    .align_left(),
            )
            .with_child(display_signed_ty(id, locale))
            .align_left(),
        Flex::row()
            .with_child(label_static("tls", UnitPoint::RIGHT))
            .with_child(Switch::new().lens(Broker::tls))
            .align_left(),
    )
}

pub fn display_credential(_id: usize) -> impl Widget<Broker> {
    Either::new(
        move |data: &Broker, _: &Env| data.use_credentials,
        Flex::column()
            .with_child(
                Flex::row()
                    .with_child(label_static("credential", UnitPoint::RIGHT))
                    .with_child(Switch::new().lens(Broker::use_credentials))
                    .align_left(),
            )
            .with_child(
                Flex::row()
                    .with_child(label_static("user name", UnitPoint::RIGHT))
                    .with_child(
                        TextBox::new()
                            .fix_width(TEXTBOX_WIDTH)
                            .lens(Broker::user_name),
                    )
                    .align_left(),
            )
            .with_child(
                Flex::row()
                    .with_child(label_static("password", UnitPoint::RIGHT))
                    .with_child(
                        TextBox::new()
                            .fix_width(TEXTBOX_WIDTH)
                            .lens(Broker::password),
                    )
                    .align_left(),
            )
            .align_left(),
        Flex::row()
            .with_child(label_static("credential", UnitPoint::RIGHT))
            .with_child(Switch::new().lens(Broker::use_credentials))
            .align_left(),
    )
}

pub fn display_signed_ty(id: usize, locale: Locale) -> impl Widget<Broker> {
    Either::new(
        move |data: &Broker, _: &Env| data.signed_ty == SignedTy::Ca,
        Flex::row()
            .with_child(label_static("ca-type", UnitPoint::RIGHT))
            .with_child(
                RadioGroup::row(vec![
                    ("ca", SignedTy::Ca),
                    ("self-signed", SignedTy::SelfSigned),
                ])
                .lens(Broker::signed_ty),
            )
            .align_left(),
        Flex::row()
            .with_child(label_static("ca-type", UnitPoint::RIGHT))
            .with_child(
                RadioGroup::row(vec![
                    ("ca", SignedTy::Ca),
                    ("self-signed", SignedTy::SelfSigned),
                ])
                .lens(Broker::signed_ty),
            )
            .with_child(TextBox::new().lens(Broker::self_signed_ca))
            .with_child(open(id, locale.clone()))
            .align_left(),
    )
}

fn open(index: usize, locale: Locale) -> impl Widget<Broker> {
    let certifacate = FileSpec::new("Certificate file", &["crt", "pem"]);
    // let default_save_name = String::from("MyFile.txt");
    let open_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![certifacate])
        .default_type(certifacate)
        // .name_label("Target")
        .title("Choose a certifacate")
        .button_text("Open");

    let open = Button::new(locale.open).on_click(move |ctx, _, _| {
        ctx.submit_command(SELF_SIGNED_FILE.with(index));
        ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options.clone()))
    });
    open
}

fn save_button(save_tx_1: Sender<AppEvent>, locale: Locale) -> impl Widget<Broker> {
    Button::new(locale.save).on_click(move |_ctx, _data: &mut Broker, _env| {
        if let Err(e) = save_tx_1.send(AppEvent::TouchSaveBroker) {
            error!("{:?}", e);
        }
    })
}

fn disconnect_button(reconnect_tx_1: Sender<AppEvent>, locale: Locale) -> impl Widget<Broker> {
    Button::new(locale.disconnect).on_click(move |_ctx, _data: &mut Broker, _env| {
        if let Err(e) = reconnect_tx_1.send(AppEvent::TouchDisconnect) {
            error!("{:?}", e);
        }
    })
}

fn reconnect_button(reconnect_tx_1: Sender<AppEvent>, locale: Locale) -> impl Widget<Broker> {
    Button::new(locale.reconnect)
        .on_click(move |_ctx, _data: &mut Broker, _env| {
            _ctx.set_focus(ID_BUTTON_RECONNECT);
            if let Err(e) = reconnect_tx_1.send(AppEvent::TouchReConnect) {
                error!("{:?}", e);
            }
        })
        .padding(BUTTON_PADDING)
}

fn connect_button(connect_tx_1: Sender<AppEvent>, locale: Locale) -> impl Widget<Broker> {
    Button::new(locale.connect).on_click(move |_ctx, _data: &mut Broker, _env| {
        _ctx.set_focus(ID_BUTTON_CONNECT);
        if let Err(e) = connect_tx_1.send(AppEvent::TouchConnectByButton) {
            error!("{:?}", e);
        }
    })
}
