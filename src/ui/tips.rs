use crate::data::hierarchy::AppData;
use crate::data::localized::Locale;
use crate::util::consts::GITHUB_ADDR;
use druid::theme::{BORDER_LIGHT, TEXTBOX_BORDER_WIDTH};
use druid::widget::{Button, Flex, Label};
use druid::{commands, Application, Env, Widget, WidgetExt};
use log::info;

pub fn tips_ui_builder(locale: Locale) -> impl Widget<AppData> {
    // TextBox::multiline()
    Flex::column()
        .with_child(
            Label::dynamic(|_: &AppData, _: &Env| {
                let commit = env!("GIT_COMMIT", "error");
                let branch = env!("GIT_BRANCH", "error");
                let build_date_time = env!("BUILD_DATE_TIME", "error");
                format!(
                    r#"1. github: https://github.com/jm-observer/for-mqtt
2. 左键双击broker记录，即可进行连接
3. 左键双击历史订阅记录，即可进行订阅
4. 左键双击订阅记录，即可取消订阅
5. 右键双击订阅topic、发布topic、发布payload，即复制对应内容
6. 当前git编译版本: {}-{}，编译时间：{}"#,
                    branch, commit, build_date_time
                )
            })
            .padding(8.0),
        )
        .with_child(
            Flex::row()
                .with_child(
                    Button::new(locale.copy_github)
                        .padding(8.0)
                        .on_click(move |_ctx, _data: &mut AppData, _env| {
                            Application::global()
                                .clipboard()
                                .put_string(GITHUB_ADDR.to_string());
                            info!("copy github addr success!");
                            _ctx.submit_command(commands::CLOSE_WINDOW);
                        })
                        .padding(8.0),
                )
                .with_child(
                    Button::new(locale.close)
                        .padding(8.0)
                        .on_click(move |_ctx, _data: &mut AppData, _env| {
                            _ctx.submit_command(commands::CLOSE_WINDOW);
                        })
                        .padding(8.0),
                ),
        )
        .border(BORDER_LIGHT, TEXTBOX_BORDER_WIDTH)
        .rounded(5.0)
}
