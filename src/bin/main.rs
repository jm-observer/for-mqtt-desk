#![windows_subsystem = "windows"]



use druid::{
    commands, AppDelegate, AppLauncher, Command, DelegateCtx, Env, Handled,
    LocalizedString, Menu, MenuItem, PlatformError, Target, WindowDesc,
    WindowId,
};
use flexi_logger::{Age, Cleanup, Criterion, FileSpec, Naming};

use for_mqtt::data::hierarchy::AppData;

use for_mqtt::logic::deal_event;

use for_mqtt::ui::ids::{SELF_SIGNED_FILE, TIPS};
use for_mqtt::ui::{init_layout, tips};

use for_mqtt::util::custom_logger::CustomWriter;
use for_mqtt::util::db::ArcDb;
use log::LevelFilter::{Debug, Info};
use log::{error};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::{panic, thread};

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

    let _logger = custom_utils::logger::logger_feature("for-mqtt", Debug, Info)
        .module("sled", Info)
        .config(fs, criterion, naming, cleanup, append)
        .log_to_write(Box::new(CustomWriter(tx.clone())))
        .build();

    panic::set_hook(Box::new(|panic_info| {
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            error!("panic occurred: {s:?}");
        } else {
            error!("panic occurred");
        }
        if let Some(location) = panic_info.location() {
            error!(
                "panic occurred in file '{}' at line {}",
                location.file(),
                location.line(),
            );
        } else {
            error!("panic occurred but can't get location information...");
        }
    }));

    let win = WindowDesc::new(init_layout(tx.clone()))
        .title(LocalizedString::new("app-names"))
        .menu(make_menu)
        .window_size((1200.0, 700.0)); //.menu(menu);
    let mut db = ArcDb::init_db(tx.clone())?;
    let mut data = db.read_app_data()?;

    let launcher = AppLauncher::with_window(win)
        .configure_env(|_env: &mut Env, _data: &AppData| {
            // env.set(WINDOW_BACKGROUND_COLOR, WHITE);
        })
        .delegate(Delegate);
    let event_sink = launcher.get_external_handle();
    thread::Builder::new()
        .name("logic-worker".to_string())
        .spawn(move || {
            if let Err(e) = deal_event(event_sink, rx, tx) {
                error!("{:?}", e);
            }
        })
        .unwrap();

    // open a tag
    if data.brokers.len() == 0 {
        data.add_broker();
    }
    launcher.launch(data)?;
    Ok(())
}

pub struct Delegate;
impl AppDelegate<AppData> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppData,
        _env: &Env,
    ) -> Handled {
        // debug!("{:?}", data.get_self_signed_file());
        if let Some(index) = cmd.get(SELF_SIGNED_FILE) {
            data.set_self_signed_file(*index);
            return Handled::Yes;
        } else if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            // debug!("{} {:?}", data.brokers.len(), file_info,);
            if let Some(index) = data.get_self_signed_file() {
                if let Some(broker) = data.brokers.get_mut(index) {
                    broker.self_signed_ca = Arc::new(file_info.path.to_string_lossy().to_string());
                    return Handled::Yes;
                }
            }
        } else if let Some(_) = cmd.get(TIPS) {
            let new_win = WindowDesc::new(tips::tips_ui_builder())
                .window_size((500.0, 210.0))
                .resizable(false)
                .set_always_on_top(true);
            _ctx.new_window(new_win);
            return Handled::Yes;
        }
        Handled::No
    }
}

fn make_menu(_: Option<WindowId>, _state: &AppData, _: &Env) -> Menu<AppData> {
    let custom = Menu::empty().entry(
        MenuItem::new(LocalizedString::new("Tips"))
            .on_activate(|ctx, _data, _env| ctx.submit_command(TIPS)),
    );
    custom
}
