use crate::data::common::QoS;
use crate::data::AString;
use druid::theme::{BORDER_LIGHT, TEXTBOX_BORDER_WIDTH};
use druid::widget::{Label, SizedBox};
use druid::{Color, Env, UnitPoint, Widget, WidgetExt};

pub fn label_dy<T: druid::Data>(f: impl Fn(&T, &Env) -> String + 'static) -> impl Widget<T> {
    Label::dynamic(f)
        .align_vertical(UnitPoint::LEFT)
        .padding(1.0)
        .fix_width(80f64)
        .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH)
}
pub fn label_dy_expand_width<T: druid::Data>(
    f: impl Fn(&T, &Env) -> String + 'static,
) -> impl Widget<T> {
    Label::dynamic(f)
        .align_vertical(UnitPoint::LEFT)
        .padding(1.0)
        .expand_width()
        .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH)
}
pub fn label_static<T: druid::Data>(text: &str) -> impl Widget<T> {
    Label::new(text)
        .align_vertical(UnitPoint::LEFT)
        .padding(1.0)
        .fix_width(80f64)
        .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH)
}

pub const QOS: fn() -> SizedBox<QoS> = || {
    Label::dynamic(|qos: &QoS, _: &Env| format!("{}", *qos as u8))
        .with_text_size(8.)
        .fix_width(20f64)
};

pub const TOPIC: fn() -> SizedBox<AString> = || {
    Label::dynamic(|data: &AString, _: &Env| format!("{}", data)).fix_width(150.)
    // .align_horizontal(UnitPoint::LEFT)
};

pub const MSG: fn() -> SizedBox<AString> =
    || Label::dynamic(|data: &AString, _: &Env| format!("{}", data)).fix_width(170.);

// pub use druid::Color::GREEN;
pub const GREEN: Color = Color::rgb8(0, 128, 0);
/// Opaque yellow.
pub const YELLOW: Color = Color::rgb8(255, 255, 0);
pub const SILVER: Color = Color::grey8(192);
