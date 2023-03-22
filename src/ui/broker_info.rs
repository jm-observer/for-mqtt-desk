use crate::data::common::{Broker, Protocol, SignedTy};
use crate::data::lens::PortLens;
use crate::data::AppEvent;
use crate::ui::common::{
    error_display_widget, label_static, BUTTON_PADDING, B_BOXTEXT, B_CONTENT, TEXTBOX_MULTI_WIDTH,
    TEXTBOX_WIDTH,
};
use crate::ui::formatter::{check_port, MustInput};
use crate::ui::ids::{
    TextBoxErrorDelegate, ID_ADDR, ID_BUTTON_CONNECT, ID_BUTTON_RECONNECT, ID_PORT,
    SELF_SIGNED_FILE,
};

use crossbeam_channel::Sender;
use druid::widget::{Button, Container, Either, Flex, RadioGroup, Switch, TextBox};
use druid::{Env, FileDialogOptions, FileSpec, UnitPoint, Widget};
use druid::{LocalizedString, WidgetExt};
use log::{debug, error};

pub fn display_broker(id: usize, tx: Sender<AppEvent>) -> Container<Broker> {
    let save_tx_0 = tx.clone();
    let _save_tx_1 = tx.clone();
    let connect_tx_1 = tx.clone();
    let disconnect_tx_1 = tx.clone();
    let save_tx_1 = tx.clone();
    let reconnect_tx_1 = tx.clone();

    let connection = Flex::column()
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
        .with_child(display_tls(id))
        .with_child(Either::new(
            move |data: &Broker, _: &Env| data.tab_status.connected,
            Flex::row()
                .with_child(save_button(save_tx_0))
                .with_child(reconnect_button(reconnect_tx_1))
                .with_child(disconnect_button(disconnect_tx_1))
                .align_left(),
            Flex::row()
                .with_child(save_button(save_tx_1))
                .with_child(connect_button(connect_tx_1))
                .align_left(),
        ))
        .with_child(
            Flex::row()
                .with_child(label_static("params", UnitPoint::RIGHT))
                .with_flex_child(
                    TextBox::multiline()
                        // .background(B_BOXTEXT)
                        // .with_placeholder("Multi")
                        .lens(Broker::params)
                        .fix_height(180.)
                        .fix_width(TEXTBOX_MULTI_WIDTH),
                    1.0,
                )
                .align_left(),
        );
    Container::new(connection)
}

pub fn display_tls(id: usize) -> impl Widget<Broker> {
    Either::new(
        move |data: &Broker, _: &Env| data.tls,
        Flex::column()
            .with_child(
                Flex::row()
                    .with_child(label_static("tls", UnitPoint::RIGHT))
                    .with_child(Switch::new().lens(Broker::tls))
                    .align_left(),
            )
            .with_child(display_signed_ty(id))
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

pub fn display_signed_ty(id: usize) -> impl Widget<Broker> {
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
            .with_child(open(id))
            .align_left(),
    )
}

fn open(index: usize) -> impl Widget<Broker> {
    let certifacate = FileSpec::new("Certificate file", &["crt", "pem"]);
    // let default_save_name = String::from("MyFile.txt");
    let open_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![certifacate])
        .default_type(certifacate)
        // .name_label("Target")
        .title("Choose a certifacate")
        .button_text("Open");

    let open = Button::new("Open").on_click(move |ctx, _, _| {
        ctx.submit_command(SELF_SIGNED_FILE.with(index));
        ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options.clone()))
    });
    open
}

fn save_button(save_tx_1: Sender<AppEvent>) -> impl Widget<Broker> {
    Button::new(LocalizedString::new("Save")).on_click(move |_ctx, data: &mut Broker, _env| {
        if let Err(e) = save_tx_1.send(AppEvent::TouchSaveBroker(data.id)) {
            error!("{:?}", e);
        }
    })
}

fn disconnect_button(reconnect_tx_1: Sender<AppEvent>) -> impl Widget<Broker> {
    Button::new(LocalizedString::new("Disconnect")).on_click(
        move |_ctx, data: &mut Broker, _env| {
            if let Err(e) = reconnect_tx_1.send(AppEvent::TouchDisconnect(data.id)) {
                error!("{:?}", e);
            }
        },
    )
}

fn reconnect_button(reconnect_tx_1: Sender<AppEvent>) -> impl Widget<Broker> {
    Button::new(LocalizedString::new("Reconnect"))
        .on_click(move |_ctx, data: &mut Broker, _env| {
            _ctx.set_focus(ID_BUTTON_RECONNECT);
            if let Err(e) = reconnect_tx_1.send(AppEvent::TouchReConnect(data.id)) {
                error!("{:?}", e);
            }
        })
        .padding(BUTTON_PADDING)
}

fn connect_button(connect_tx_1: Sender<AppEvent>) -> impl Widget<Broker> {
    Button::new(LocalizedString::new("Connect")).on_click(move |_ctx, broker: &mut Broker, _env| {
        debug!("{:?}", broker);
        _ctx.set_focus(ID_BUTTON_CONNECT);
        if let Err(e) = connect_tx_1.send(AppEvent::TouchConnectByButton(broker.id)) {
            error!("{:?}", e);
        }
    })
}
