use crate::data::common::PayloadTy;
use crate::ui::common::{GRAY};
use druid::widget::{Label};
use druid::{Data, Env, Lens, Widget, WidgetExt};
use druid_widget_nursery::DropdownSelect;
use std::sync::Arc;

pub fn down_select_payload_ty() -> impl Widget<PayloadTy> {
    DropdownSelect::new(vec![
        ("Text", PayloadTy::Text),
        ("Json", PayloadTy::Json),
        ("Hex", PayloadTy::Hex),
    ])
}

pub fn payload_ty_init<T: Data>(data: impl Lens<T, Arc<String>> + 'static) -> impl Widget<T> {
    // TextBox::<Arc<String>>::new()
    //     .fix_width(15.0)
    //     .padding(1.0)
    //     .disabled_if(|_, _| true)
    //     .lens(data)

    Label::dynamic(|qos: &Arc<String>, _: &Env| format!("{}", qos))
        .lens(data)
        .fix_width(15.0)
        .padding(1.0)
        .background(GRAY)
        .rounded(1.0)
}
