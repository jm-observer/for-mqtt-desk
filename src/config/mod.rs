use crate::config::theme::{
    LapceConfig, ThemeColor, ThemeConfig, UIConfig, DEFAULT_DARK_THEME, DEFAULT_LIGHT_THEME,
    DEFAULT_SETTINGS,
};
use anyhow::Result;
use config as configs;
use druid::im::HashMap;
use druid::{Color, ExtEventSink};
use serde::Deserialize;
use std::path::Path;

pub mod theme;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Config {
    #[serde(skip)]
    pub id: u64,
    pub lapce: LapceConfig,
    pub ui: UIConfig,
    pub theme: ThemeConfig,
    #[serde(skip)]
    pub default_theme: ThemeConfig,
    #[serde(skip)]
    pub color: ThemeColor,
    #[serde(skip)]
    pub available_themes: HashMap<String, (String, configs::Config)>,
}

pub struct ConfigWatcher {
    event_sink: ExtEventSink,
}

impl ConfigWatcher {
    pub fn new(event_sink: ExtEventSink) -> Self {
        Self { event_sink }
    }
}

impl notify::EventHandler for ConfigWatcher {
    fn handle_event(&mut self, event: notify::Result<notify::Event>) {
        if let Ok(event) = event {
            match event.kind {
                notify::EventKind::Create(_)
                | notify::EventKind::Modify(_)
                | notify::EventKind::Remove(_) => {
                    // todo
                    // let _ = self.event_sink.submit_command(
                    //     LAPCE_UI_COMMAND,
                    //     LapceUICommand::ReloadConfig,
                    //     Target::Auto,
                    // );
                }
                _ => (),
            }
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let default_settings = Self::default_settings();
        let mut default_config: Config = default_settings.clone().try_deserialize().unwrap();

        let settings = default_settings.clone();
        let mut config: Config = settings.try_deserialize()?;
        let available_themes = Self::load_themes();
        if let Some((_, theme)) = available_themes.get(&config.lapce.color_theme.to_lowercase()) {}
        config.available_themes = available_themes;
        config.default_theme = default_config.theme.clone();

        Ok(config)
    }

    fn load_themes() -> HashMap<String, (String, configs::Config)> {
        let mut themes = HashMap::default();
        let (name, theme) = Self::load_theme_from_str(DEFAULT_LIGHT_THEME).unwrap();
        themes.insert(name.to_lowercase(), (name, theme));
        let (name, theme) = Self::load_theme_from_str(DEFAULT_DARK_THEME).unwrap();
        themes.insert(name.to_lowercase(), (name, theme));

        themes
    }

    fn load_theme_from_str(s: &str) -> Option<(String, configs::Config)> {
        let settings = configs::Config::builder()
            .add_source(configs::File::from_str(s, configs::FileFormat::Toml))
            .build()
            .unwrap();
        let table = settings.get_table("theme").ok()?;
        let name = table.get("name")?.to_string();
        Some((name, settings))
    }

    fn load_theme(path: &Path) -> Option<(String, (String, configs::Config))> {
        if !path.is_file() {
            return None;
        }
        let settings = configs::Config::builder()
            .add_source(configs::File::from(path))
            .build()
            .unwrap();
        let table = settings.get_table("theme").ok()?;
        let name = table.get("name")?.to_string();
        Some((name.to_lowercase(), (name, settings)))
    }

    fn default_settings() -> configs::Config {
        configs::Config::builder()
            .add_source(configs::File::from_str(
                DEFAULT_SETTINGS,
                configs::FileFormat::Toml,
            ))
            .build()
            .unwrap()
    }
}
