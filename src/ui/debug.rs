use crate::data::hierarchy::AppData;
use crate::ui::common::label_static;
use druid::Widget;

#[allow(dead_code)]
pub fn debug_label_appdata() -> impl Widget<AppData> {
    label_static("debug_label_appdata")
}
