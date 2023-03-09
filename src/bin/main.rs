#![allow(unused_imports)]
#![windows_subsystem = "windows"]

use druid::theme::WINDOW_BACKGROUND_COLOR;
use druid::{AppLauncher, Color, Env, LocalizedString, PlatformError, WindowDesc};
use flexi_logger::{Age, Cleanup, Criterion, FileSpec, Naming};
use for_mqtt::data::hierarchy::AppData;
use for_mqtt::logic::deal_event;
use for_mqtt::ui::common::WHITE;
use for_mqtt::ui::init_layout;
use for_mqtt::util::custom_logger::CustomWriter;
use for_mqtt::util::db::ArcDb;
use log::error;
use log::LevelFilter::{Debug, Info};
use std::path::PathBuf;
use std::str::FromStr;
use std::thread;

fn main() -> Result<(), PlatformError> {
    let (tx, rx) = crossbeam_channel::bounded(1024);

    let fs_path = PathBuf::from_str("./resources/log").unwrap();
    let fs = FileSpec::default()
        .directory(fs_path)
        .basename("for-mqtt")
        .suffix("log");
    // 若为true，则会覆盖rotate中的数字、keep^
    let criterion = Criterion::AgeOrSize(Age::Day, 10_000_000);
    let naming = Naming::Numbers;
    let cleanup = Cleanup::KeepLogFiles(2);
    let append = true;

    let _logger = custom_utils::logger::logger_feature("for-mqtt", Info, Info)
        .module("sled", Info)
        .config(fs, criterion, naming, cleanup, append)
        .log_to_write(Box::new(CustomWriter(tx.clone())))
        .build();

    let win = WindowDesc::new(init_layout(tx.clone()))
        .title(LocalizedString::new("app-names"))
        .window_size((1200.0, 700.0)); //.menu(menu);
    let mut db = ArcDb::init_db(tx.clone())?;
    let data = db.read_app_data()?;

    let launcher =
        AppLauncher::with_window(win).configure_env(|_env: &mut Env, _data: &AppData| {
            // env.set(WINDOW_BACKGROUND_COLOR, WHITE);
        });
    let event_sink = launcher.get_external_handle();
    thread::Builder::new()
        .name("logic-worker".to_string())
        .spawn(move || {
            if let Err(e) = deal_event(event_sink, rx, tx) {
                error!("{:?}", e);
            }
        })
        .unwrap();
    launcher.launch(data)?;
    Ok(())
}
