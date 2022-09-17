#![allow(unused_mut)]
use config::Config;
use std::path::PathBuf;

use druid::im::HashMap;
use druid::{Color, FontFamily};
use serde::{Deserialize, Serialize};

pub const DEFAULT_SETTINGS: &str = include_str!("../../resources/themes/settings.toml");
pub const DEFAULT_LIGHT_THEME: &str = include_str!("../../resources/themes/light-theme.toml");
pub const DEFAULT_DARK_THEME: &str = include_str!("../../resources/themes/dark-theme.toml");

pub struct LapceTheme {}

impl LapceTheme {
    pub const LAPCE_WARN: &'static str = "lapce.warn";
    pub const LAPCE_ERROR: &'static str = "lapce.error";
    pub const LAPCE_ACTIVE_TAB: &'static str = "lapce.active_tab";
    pub const LAPCE_INACTIVE_TAB: &'static str = "lapce.inactive_tab";
    pub const LAPCE_DROPDOWN_SHADOW: &'static str = "lapce.dropdown_shadow";
    pub const LAPCE_BORDER: &'static str = "lapce.border";
    pub const LAPCE_SCROLL_BAR: &'static str = "lapce.scroll_bar";

    pub const EDITOR_BACKGROUND: &'static str = "editor.background";
    pub const EDITOR_FOREGROUND: &'static str = "editor.foreground";
    pub const EDITOR_DIM: &'static str = "editor.dim";
    pub const EDITOR_FOCUS: &'static str = "editor.focus";
    pub const EDITOR_CARET: &'static str = "editor.caret";
    pub const EDITOR_SELECTION: &'static str = "editor.selection";
    pub const EDITOR_CURRENT_LINE: &'static str = "editor.current_line";
    pub const EDITOR_LINK: &'static str = "editor.link";

    pub const INLAY_HINT_FOREGROUND: &'static str = "inlay_hint.foreground";
    pub const INLAY_HINT_BACKGROUND: &'static str = "inlay_hint.background";

    pub const ERROR_LENS_ERROR_FOREGROUND: &'static str = "error_lens.error.foreground";
    pub const ERROR_LENS_ERROR_BACKGROUND: &'static str = "error_lens.error.background";
    pub const ERROR_LENS_WARNING_FOREGROUND: &'static str = "error_lens.warning.foreground";
    pub const ERROR_LENS_WARNING_BACKGROUND: &'static str = "error_lens.warning.background";
    pub const ERROR_LENS_OTHER_FOREGROUND: &'static str = "error_lens.other.foreground";
    pub const ERROR_LENS_OTHER_BACKGROUND: &'static str = "error_lens.other.background";

    pub const SOURCE_CONTROL_ADDED: &'static str = "source_control.added";
    pub const SOURCE_CONTROL_REMOVED: &'static str = "source_control.removed";
    pub const SOURCE_CONTROL_MODIFIED: &'static str = "source_control.modified";

    pub const PALETTE_BACKGROUND: &'static str = "palette.background";
    pub const PALETTE_CURRENT: &'static str = "palette.current";

    pub const COMPLETION_BACKGROUND: &'static str = "completion.background";
    pub const COMPLETION_CURRENT: &'static str = "completion.current";

    pub const HOVER_BACKGROUND: &'static str = "hover.background";

    pub const ACTIVITY_BACKGROUND: &'static str = "activity.background";
    pub const ACTIVITY_CURRENT: &'static str = "activity.current";

    pub const PANEL_BACKGROUND: &'static str = "panel.background";
    pub const PANEL_CURRENT: &'static str = "panel.current";
    pub const PANEL_HOVERED: &'static str = "panel.hovered";

    pub const STATUS_BACKGROUND: &'static str = "status.background";
    pub const STATUS_MODAL_NORMAL: &'static str = "status.modal.normal";
    pub const STATUS_MODAL_INSERT: &'static str = "status.modal.insert";
    pub const STATUS_MODAL_VISUAL: &'static str = "status.modal.visual";
    pub const STATUS_MODAL_TERMINAL: &'static str = "status.modal.terminal";

    pub const PALETTE_INPUT_LINE_HEIGHT: druid::Key<f64> =
        druid::Key::new("lapce.palette_input_line_height");
    pub const PALETTE_INPUT_LINE_PADDING: druid::Key<f64> =
        druid::Key::new("lapce.palette_input_line_padding");
    pub const INPUT_LINE_HEIGHT: druid::Key<f64> = druid::Key::new("lapce.input_line_height");
    pub const INPUT_LINE_PADDING: druid::Key<f64> = druid::Key::new("lapce.input_line_padding");
    pub const INPUT_FONT_SIZE: druid::Key<u64> = druid::Key::new("lapce.input_font_size");

    pub const MARKDOWN_BLOCKQUOTE: &'static str = "markdown.blockquote";
}

pub trait GetConfig {
    fn get_config(&self) -> &Config;
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct LapceConfig {
    pub modal: bool,
    pub color_theme: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct UIConfig {
    font_family: String,

    font_size: usize,

    header_height: usize,

    status_height: usize,

    tab_min_width: usize,

    scroll_width: usize,

    drop_shadow_width: usize,

    hover_font_family: String,
    hover_font_size: usize,
}

impl UIConfig {
    pub fn font_family(&self) -> FontFamily {
        if self.font_family.is_empty() {
            FontFamily::SYSTEM_UI
        } else {
            FontFamily::new_unchecked(self.font_family.clone())
        }
    }

    pub fn font_size(&self) -> usize {
        self.font_size.max(6).min(32)
    }

    pub fn header_height(&self) -> usize {
        let font_size = self.font_size();
        self.header_height.max(font_size)
    }

    pub fn status_height(&self) -> usize {
        let font_size = self.font_size();
        self.status_height.max(font_size)
    }

    pub fn tab_min_width(&self) -> usize {
        self.tab_min_width
    }

    pub fn scroll_width(&self) -> usize {
        self.scroll_width
    }

    pub fn drop_shadow_width(&self) -> usize {
        self.drop_shadow_width
    }

    pub fn hover_font_family(&self) -> FontFamily {
        if self.hover_font_family.is_empty() {
            self.font_family()
        } else {
            FontFamily::new_unchecked(self.hover_font_family.clone())
        }
    }

    pub fn hover_font_size(&self) -> usize {
        if self.hover_font_size == 0 {
            self.font_size()
        } else {
            self.hover_font_size
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ThemeConfig {
    #[serde(skip)]
    pub path: PathBuf,
    pub name: String,
    pub base: ThemeBaseConfig,
    pub syntax: HashMap<String, String>,
    pub ui: HashMap<String, String>,
}

impl ThemeConfig {
    fn resolve_color(
        colors: &HashMap<String, String>,
        base: &ThemeBaseColor,
        default: Option<&HashMap<String, Color>>,
    ) -> HashMap<String, Color> {
        colors
            .iter()
            .map(|(name, hex)| {
                if let Some(stripped) = hex.strip_prefix('$') {
                    if let Some(c) = base.get(stripped) {
                        return (name.to_string(), c.clone());
                    }
                    if let Some(default) = default {
                        if let Some(c) = default.get(name) {
                            return (name.to_string(), c.clone());
                        }
                    }
                    return (name.to_string(), Color::rgb8(0, 0, 0));
                }

                if let Ok(c) = Color::from_hex_str(hex) {
                    return (name.to_string(), c);
                }
                if let Some(default) = default {
                    if let Some(c) = default.get(name) {
                        return (name.to_string(), c.clone());
                    }
                }
                (name.to_string(), Color::rgb8(0, 0, 0))
            })
            .collect()
    }

    pub(crate) fn resolve_ui_color(
        &self,
        base: &ThemeBaseColor,
        default: Option<&HashMap<String, Color>>,
    ) -> HashMap<String, Color> {
        Self::resolve_color(&self.ui, base, default)
    }

    pub(crate) fn resolve_syntax_color(
        &self,
        base: &ThemeBaseColor,
        default: Option<&HashMap<String, Color>>,
    ) -> HashMap<String, Color> {
        Self::resolve_color(&self.syntax, base, default)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ThemeBaseConfig {
    pub white: String,
    pub black: String,
    pub grey: String,
    pub blue: String,
    pub red: String,
    pub yellow: String,
    pub orange: String,
    pub green: String,
    pub purple: String,
    pub cyan: String,
    pub magenta: String,
}

impl ThemeBaseConfig {
    pub fn resolve(&self, default: Option<&ThemeBaseColor>) -> ThemeBaseColor {
        ThemeBaseColor {
            white: Color::from_hex_str(&self.white).unwrap_or_else(|_| {
                default
                    .map(|d| d.white.clone())
                    .unwrap_or_else(|| Color::rgb8(0, 0, 0))
            }),
            black: Color::from_hex_str(&self.black).unwrap_or_else(|_| {
                default
                    .map(|d| d.black.clone())
                    .unwrap_or_else(|| Color::rgb8(0, 0, 0))
            }),
            grey: Color::from_hex_str(&self.grey).unwrap_or_else(|_| {
                default
                    .map(|d| d.grey.clone())
                    .unwrap_or_else(|| Color::rgb8(0, 0, 0))
            }),
            blue: Color::from_hex_str(&self.blue).unwrap_or_else(|_| {
                default
                    .map(|d| d.blue.clone())
                    .unwrap_or_else(|| Color::rgb8(0, 0, 0))
            }),
            red: Color::from_hex_str(&self.red).unwrap_or_else(|_| {
                default
                    .map(|d| d.red.clone())
                    .unwrap_or_else(|| Color::rgb8(0, 0, 0))
            }),
            yellow: Color::from_hex_str(&self.yellow).unwrap_or_else(|_| {
                default
                    .map(|d| d.yellow.clone())
                    .unwrap_or_else(|| Color::rgb8(0, 0, 0))
            }),
            orange: Color::from_hex_str(&self.orange).unwrap_or_else(|_| {
                default
                    .map(|d| d.orange.clone())
                    .unwrap_or_else(|| Color::rgb8(0, 0, 0))
            }),
            green: Color::from_hex_str(&self.green).unwrap_or_else(|_| {
                default
                    .map(|d| d.green.clone())
                    .unwrap_or_else(|| Color::rgb8(0, 0, 0))
            }),
            purple: Color::from_hex_str(&self.purple).unwrap_or_else(|_| {
                default
                    .map(|d| d.purple.clone())
                    .unwrap_or_else(|| Color::rgb8(0, 0, 0))
            }),
            cyan: Color::from_hex_str(&self.cyan).unwrap_or_else(|_| {
                default
                    .map(|d| d.cyan.clone())
                    .unwrap_or_else(|| Color::rgb8(0, 0, 0))
            }),
            magenta: Color::from_hex_str(&self.magenta).unwrap_or_else(|_| {
                default
                    .map(|d| d.magenta.clone())
                    .unwrap_or_else(|| Color::rgb8(0, 0, 0))
            }),
        }
    }

    pub fn get(&self, name: &str) -> Option<&String> {
        Some(match name {
            "white" => &self.white,
            "black" => &self.black,
            "grey" => &self.grey,
            "blue" => &self.blue,
            "red" => &self.red,
            "yellow" => &self.yellow,
            "orange" => &self.orange,
            "green" => &self.green,
            "purple" => &self.purple,
            "cyan" => &self.cyan,
            "magenta" => &self.magenta,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct ThemeColor {
    pub base: ThemeBaseColor,
    pub syntax: HashMap<String, Color>,
    pub ui: HashMap<String, Color>,
}

#[derive(Debug, Clone)]
pub struct ThemeBaseColor {
    pub white: Color,
    pub black: Color,
    pub grey: Color,
    pub blue: Color,
    pub red: Color,
    pub yellow: Color,
    pub orange: Color,
    pub green: Color,
    pub purple: Color,
    pub cyan: Color,
    pub magenta: Color,
}

impl Default for ThemeBaseColor {
    fn default() -> Self {
        Self {
            white: Color::rgb8(0, 0, 0),
            black: Color::rgb8(0, 0, 0),
            grey: Color::rgb8(0, 0, 0),
            blue: Color::rgb8(0, 0, 0),
            red: Color::rgb8(0, 0, 0),
            yellow: Color::rgb8(0, 0, 0),
            orange: Color::rgb8(0, 0, 0),
            green: Color::rgb8(0, 0, 0),
            purple: Color::rgb8(0, 0, 0),
            cyan: Color::rgb8(0, 0, 0),
            magenta: Color::rgb8(0, 0, 0),
        }
    }
}

impl ThemeBaseColor {
    fn get(&self, name: &str) -> Option<&Color> {
        Some(match name {
            "white" => &self.white,
            "black" => &self.black,
            "grey" => &self.grey,
            "blue" => &self.blue,
            "red" => &self.red,
            "yellow" => &self.yellow,
            "orange" => &self.orange,
            "green" => &self.green,
            "purple" => &self.purple,
            "cyan" => &self.cyan,
            "magenta" => &self.magenta,
            _ => return None,
        })
    }

    pub fn keys(&self) -> [&'static str; 11] {
        [
            "white", "black", "grey", "blue", "red", "yellow", "orange", "green", "purple", "cyan",
            "magenta",
        ]
    }
}
