use crate::data::common::QoS;
use crate::data::AString;
use crate::ui::ids::{ErrorController, ERROR_TEXT_COLOR};
use druid::text::ValidationError;
use druid::theme::{BORDER_LIGHT, TEXTBOX_BORDER_WIDTH};
use druid::widget::{Either, Label, LabelText, SizedBox, Svg, SvgData};
use druid::{Color, Data, Env, UnitPoint, Widget, WidgetExt, WidgetId};
use log::debug;

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

pub const QOS: fn() -> SizedBox<QoS> = || {
    Label::dynamic(|qos: &QoS, _: &Env| format!("{}", qos.to_u8()))
        .with_text_size(8.)
        .fix_width(20f64)
};

pub const TOPIC: fn() -> SizedBox<AString> =
    || Label::dynamic(|data: &AString, _: &Env| format!("{}", data)).fix_width(150.);

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
