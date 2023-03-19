use crate::data::common::{Broker, Protocol, SignedTy};
use crate::data::lens::PortLens;
use crate::data::{AString, AppEvent};
use crate::ui::common::{
    error_display_widget, label_static, BUTTON_PADDING, TEXTBOX_MULTI_WIDTH, TEXTBOX_WIDTH,
};
use crate::ui::formatter::{check_addr, check_no_empty, check_port, MustInput};
use crate::ui::ids::{
    TextBoxErrorDelegate, ID_ADDR, ID_BUTTON_CONNECT, ID_BUTTON_RECONNECT, ID_CLIENT_ID, ID_PORT,
    SELF_SIGNED_FILE,
};
use crate::util::general_id;
use druid::widget::{Button, Container, Either, Flex, Label, Painter, RadioGroup, Switch, TextBox};
use druid::{
    Color, Data, Env, FileDialogOptions, FileSpec, LensExt, RenderContext, UnitPoint, Widget,
};
use druid::{LocalizedString, WidgetExt};
use log::{debug, error};

pub fn display_broker(id: usize) -> Container<Broker> {
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
                .with_child(
                    Button::new(LocalizedString::new("Save"))
                        .on_click(move |_ctx, data: &mut Broker, _env| {
                            todo!()
                            // if let Err(e) = data.db.tx.send(AppEvent::SaveBroker(id)) {
                            //     error!("{:?}", e);
                            // }
                        })
                        .padding(BUTTON_PADDING),
                )
                .with_child(
                    Button::new(LocalizedString::new("Reconnect"))
                        .on_click(move |_ctx, data: &mut Broker, _env| {
                            todo!()
                            // _ctx.set_focus(ID_BUTTON_RECONNECT);
                            // if let Err(e) = data.db.tx.send(AppEvent::ReConnect(id)) {
                            //     error!("{:?}", e);
                            // }
                        })
                        .padding(BUTTON_PADDING),
                )
                .with_child(Button::new(LocalizedString::new("Disconnect")).on_click(
                    move |_ctx, data: &mut Broker, _env| {
                        todo!()
                        // if let Err(e) = data.db.tx.send(AppEvent::Disconnect(id)) {
                        //     error!("{:?}", e);
                        // }
                    },
                ))
                .align_left(),
            Flex::row()
                .with_child(Button::new(LocalizedString::new("Save")).on_click(
                    move |_ctx, data: &mut Broker, _env| {
                        todo!()
                        // if let Err(e) = data.db.tx.send(AppEvent::SaveBroker(id)) {
                        //     error!("{:?}", e);
                        // }
                    },
                ))
                .with_child(Button::new(LocalizedString::new("Connect")).on_click(
                    move |_ctx, data: &mut Broker, _env| {
                        todo!()
                        // if let Some(broker) = data.brokers.iter_mut().find(|x| x.id == id) {
                        //     debug!("{:?}", broker);
                        //     _ctx.set_focus(ID_BUTTON_CONNECT);
                        //     if broker.client_id.as_str().is_empty() {
                        //         broker.client_id = general_id().into();
                        //     }
                        //     if let Err(e) = data.db.tx.send(AppEvent::Connect(broker.clone())) {
                        //         error!("{:?}", e);
                        //     }
                        // } else {
                        //     error!("can't get the broker");
                        // }
                    },
                ))
                .align_left(),
        ))
        .with_child(
            Flex::row()
                .with_child(label_static("params", UnitPoint::RIGHT))
                .with_flex_child(
                    TextBox::multiline()
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

fn label_widget<T: Data>(widget: impl Widget<T> + 'static, label: &str) -> impl Widget<T> {
    Flex::column()
        .must_fill_main_axis(true)
        .with_flex_child(widget.center(), 1.0)
        .with_child(
            Painter::new(|ctx, _: &_, _: &_| {
                let size = ctx.size().to_rect();
                ctx.fill(size, &Color::WHITE)
            })
            .fix_height(1.0),
        )
        .with_child(Label::new(label).center().fix_height(40.0))
        .border(Color::WHITE, 1.0)
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

pub fn display_credential(id: usize) -> impl Widget<Broker> {
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
