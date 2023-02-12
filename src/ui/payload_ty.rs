use crate::data::common::PayloadTy;
use druid::widget::TextBox;
use druid::{Data, Lens, Widget, WidgetExt};
use druid_widget_nursery::DropdownSelect;
use std::sync::Arc;

pub fn down_select_payload_ty() -> impl Widget<PayloadTy> {
    DropdownSelect::new(vec![
        ("Text", PayloadTy::Text),
        ("Json", PayloadTy::Json),
        ("Hex", PayloadTy::Hex),
    ])
}

pub fn payload_ty_init<T: Data>(data: impl Lens<T, Arc<String>>) -> impl Widget<T> {
    TextBox::<Arc<String>>::new()
        .fix_width(15.0)
        .padding(1.0)
        .lens(data)
}
