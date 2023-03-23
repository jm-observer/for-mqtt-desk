use crate::data::AString;
use crate::ui::ids::{ErrorController, ERROR_TEXT_COLOR};

use druid::text::ValidationError;

use druid::widget::{Controller, Either, Label, LabelText, SizedBox, Svg, SvgData, TextBox};
use druid::{
    Application, Color, Data, Env, Event, EventCtx, UnitPoint, Widget, WidgetExt, WidgetId,
};

use log::info;
use std::sync::Arc;

pub const LABLE_WIDTH: f64 = 80.;
pub const ERROR_LABLE_WIDTH: f64 = 180.;
pub const TEXTBOX_WIDTH: f64 = 180.;
pub const TEXTBOX_MULTI_WIDTH: f64 = 300.;
pub const LABLE_PADDING: f64 = 5.0;
pub const BUTTON_PADDING: f64 = 5.0;

// pub use piet::Color::GREEN;
pub const GREEN: Color = Color::rgb8(0, 128, 0);
/// Opaque yellow.
pub const YELLOW: Color = Color::rgb8(255, 255, 0);
pub const SILVER: Color = Color::grey8(192);
pub const RED: Color = Color::rgb8(255, 0, 0);
pub const WHITE: Color = Color::grey8(255);

pub const B_WINDOW: Color = Color::rgb8(242, 242, 242);
pub const B_CONTENT: Color = Color::rgb8(41, 41, 41);

pub fn svg<T: druid::Data>(data: SvgData) -> impl Widget<T> {
    Svg::new(data).fix_size(18.0, 18.0).padding(1.0)
}

pub fn label_dy<T: druid::Data>(f: impl Fn(&T, &Env) -> String + 'static) -> impl Widget<T> {
    Label::dynamic(f)
        .align_vertical(UnitPoint::RIGHT)
        .padding(LABLE_PADDING)
        .fix_width(LABLE_WIDTH)
    // .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH)
}
pub fn label_dy_expand_width<T: druid::Data>(
    f: impl Fn(&T, &Env) -> String + 'static,
) -> impl Widget<T> {
    Label::dynamic(f)
        .align_vertical(UnitPoint::RIGHT)
        .padding(LABLE_PADDING)
        .expand_width()
    // .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH)
}
pub fn label_static<T: druid::Data>(
    text: impl Into<LabelText<T>>,
    unit: UnitPoint,
) -> impl Widget<T> {
    Label::new(text)
        .align_vertical(unit)
        .padding(LABLE_PADDING)
        .fix_width(LABLE_WIDTH)
    // .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH)
}
pub fn title<T: druid::Data>(text: impl Into<LabelText<T>>, unit: UnitPoint) -> impl Widget<T> {
    Label::new(text)
        .with_text_size(11.0)
        .align_vertical(unit)
        .padding(LABLE_PADDING)
        .fix_width(180.0)
    // .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH)
}
pub fn label_static_expand_width<T: druid::Data>(
    text: impl Into<LabelText<T>>,
    unit: UnitPoint,
) -> impl Widget<T> {
    Label::new(text)
        .align_vertical(unit)
        .padding(LABLE_PADDING)
        .expand_width()
    // .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH)
}

pub const QOS: fn() -> SizedBox<Arc<String>> = || {
    Label::dynamic(|qos: &Arc<String>, _: &Env| format!("{}", qos))
        .with_text_size(8.)
        .fix_width(20f64)
};

pub const TOPIC: fn() -> SizedBox<AString> = || {
    Label::dynamic(|data: &AString, _: &Env| format!("{}", data))
        .controller(RightClickToCopy)
        .fix_width(150.)
};

pub const MSG: fn() -> SizedBox<AString> =
    || Label::dynamic(|data: &AString, _: &Env| format!("{}", data)).fix_width(170.);

pub fn error_display_widget<T: Data>(id: WidgetId) -> impl Widget<T> {
    ErrorController::new(
        Either::new(
            |d: &Option<ValidationError>, _| d.is_some(),
            Label::dynamic(|d: &Option<ValidationError>, _| {
                d.as_ref().map(|d| d.to_string()).unwrap_or_default()
            })
            .with_text_color(ERROR_TEXT_COLOR)
            .with_text_size(12.0)
            .align_vertical(UnitPoint::LEFT)
            .padding(LABLE_PADDING)
            .fix_width(ERROR_LABLE_WIDTH), // .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH)
            SizedBox::empty(),
        )
        .with_id(id),
    )
}

pub struct RightClickToCopy;

impl<T: druid::Data + druid::text::TextStorage + druid::text::EditableText + ToString>
    Controller<T, TextBox<T>> for RightClickToCopy
{
    fn event(
        &mut self,
        child: &mut TextBox<T>,
        _ctx: &mut EventCtx,
        event: &Event,
        data: &mut T,
        _env: &Env,
    ) {
        match event {
            Event::MouseUp(cmd) => {
                if cmd.button.is_right() {
                    if let Some(text) = child.text().borrow().layout.text() {
                        let text = text.to_string();
                        if !text.is_empty() {
                            Application::global()
                                .clipboard()
                                .put_string(text.to_string());
                            info!("copy success!");
                            _ctx.set_handled();
                            return;
                        }
                    }
                }
            }
            _ => {}
        }
        child.event(_ctx, event, data, _env)
    }
}

impl<T: druid::Data + druid::text::TextStorage + druid::text::EditableText + ToString>
    Controller<T, Label<T>> for RightClickToCopy
{
    fn event(
        &mut self,
        child: &mut Label<T>,
        _ctx: &mut EventCtx,
        event: &Event,
        data: &mut T,
        _env: &Env,
    ) {
        match event {
            Event::MouseUp(cmd) => {
                if cmd.button.is_right() {
                    let text = child.text().to_string();
                    if !text.is_empty() {
                        Application::global()
                            .clipboard()
                            .put_string(text.to_string());
                        info!("copy success!");
                        _ctx.set_handled();
                        return;
                    }
                }
            }
            _ => {}
        }
        child.event(_ctx, event, data, _env)
    }
}
