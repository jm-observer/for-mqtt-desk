use crate::data::hierarchy::AppData;
use crate::util::consts::{GITHUB_ADDR, TIPS_CONTENT};
use druid::theme::{BORDER_LIGHT, TEXTBOX_BORDER_WIDTH};
use druid::widget::{Button, Flex, Label};
use druid::{Application, Env, Widget, WidgetExt};
use log::info;

pub fn tips_ui_builder() -> impl Widget<AppData> {
    // TextBox::multiline()
    Flex::column()
        .with_child(Label::dynamic(|_: &AppData, _: &Env| TIPS_CONTENT.to_string()).padding(8.0))
        .with_child(
            Button::new("复制github")
                .padding(8.0)
                .on_click(move |_ctx, _data: &mut AppData, _env| {
                    Application::global()
                        .clipboard()
                        .put_string(GITHUB_ADDR.to_string());
                    info!("copy github addr success!");
                })
                .padding(8.0),
        )
        .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH)
        .rounded(5.0)
}
