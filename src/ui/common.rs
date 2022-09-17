use druid::theme::{BORDER_LIGHT, TEXTBOX_BORDER_WIDTH};
use druid::widget::Label;
use druid::{Env, UnitPoint, Widget, WidgetExt};

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
