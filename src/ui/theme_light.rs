use druid::theme::*;
use druid::{Color, Env};

pub fn update_env(env: &mut Env) {
    env.set(WINDOW_BACKGROUND_COLOR, Color::rgb8(0xFF, 0xFF, 0xFF));
    env.set(TEXT_COLOR, Color::rgb8(0x33, 0x33, 0x33));

    // 影响输入框的背景色：BACKGROUND_LIGHT
    env.set(BACKGROUND_LIGHT, Color::rgb8(0xF0, 0xF0, 0xF0));
    // 影响switch off状态的背景色：从BACKGROUND_LIGHT到BACKGROUND_DARK的渐变色
    env.set(BACKGROUND_DARK, Color::rgb8(0xd0, 0xd0, 0xd0));

    // 影响switch on状态的背景色，整体颜色为light到dark的渐变色
    env.set(PRIMARY_LIGHT, Color::rgb8(0xff, 0xa5, 0x00));
    env.set(PRIMARY_DARK, Color::rgb8(0xff, 0xa5, 0x00));
    // 影响switch 圆圈按钮的背景色    A55CAA
    env.set(FOREGROUND_DARK, Color::rgb8(0xa5, 0x5c, 0xaa));
    env.set(FOREGROUND_LIGHT, Color::rgb8(0xa5, 0x5c, 0xaa));

    // 按钮背景色：橙色 (#FFA500。渐变色
    env.set(BUTTON_DARK, Color::rgb8(0xff, 0xc0, 0x4d));
    env.set(BUTTON_LIGHT, Color::rgb8(0xff, 0xc0, 0x4d));

    // env.set(DISABLED_FOREGROUND_LIGHT, Color::rgb8(0x80, 0x00, 0x80));
    // env.set(DISABLED_FOREGROUND_DARK, Color::rgb8(0x80, 0x00, 0x80));

    // 影响边框的颜色
    // env.set(BORDER_DARK, Color::rgb8(0x80, 0x00, 0x80));
    // env.set(BORDER_LIGHT, Color::rgb8(0x80, 0x00, 0x80));

    // 影响边框的颜色
    env.set(SCROLLBAR_COLOR, Color::rgb8(0x80, 0x00, 0x80));
    env.set(SCROLLBAR_BORDER_COLOR, Color::rgb8(0x80, 0x00, 0x80));

    env.set(DISABLED_TEXT_COLOR, Color::rgb8(0xff, 0xa5, 0x00));
    env.set(PLACEHOLDER_COLOR, Color::rgb8(0xF0, 0xF0, 0xF0));
    //
    // env.set(
    //     SELECTED_TEXT_BACKGROUND_COLOR,
    //     Color::rgb8(0xff, 0xa5, 0x00),
    // );
    // env.set(
    //     SELECTED_TEXT_INACTIVE_BACKGROUND_COLOR,
    //     Color::rgb8(0xff, 0xa5, 0x00),
    // );
    // env.set(SELECTION_TEXT_COLOR, Color::rgb8(0xff, 0xa5, 0x00));

    //

    //
    // env.set(DISABLED_FOREGROUND_LIGHT, Color::rgb8(0xff, 0xa5, 0x00));
    // env.set(DISABLED_FOREGROUND_DARK, Color::rgb8(0xff, 0xa5, 0x00));
    //
    // env.set(SCROLLBAR_COLOR, Color::rgb8(0xff, 0xa5, 0x00));
}
