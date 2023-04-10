#![windows_subsystem = "windows"]

use druid::{
    commands, AppDelegate, AppLauncher, Command, DelegateCtx, Env, Handled, PlatformError, Target,
    WindowDesc,
};
use flexi_logger::{Age, Cleanup, Criterion, FileSpec, Naming};

use for_mqtt::data::hierarchy::AppData;

use for_mqtt::logic::deal_event;

use for_mqtt::ui::ids::{SELF_SIGNED_FILE, TIPS};
use for_mqtt::ui::{init_layout, tips};

use backtrace::Backtrace;
use directories::UserDirs;
use for_mqtt::config::Config;
use for_mqtt::data::localized::{get_locale, Locale};
use for_mqtt::data::AppEvent;
use for_mqtt::util::custom_logger::CustomWriter;
use for_mqtt::util::db::ArcDb;
use log::error;
use log::LevelFilter::{Debug, Info};
use std::process::exit;
use std::sync::Arc;
use std::{panic, thread};

fn main() -> Result<(), PlatformError> {
    let (tx, rx) = crossbeam_channel::bounded(1024);

    let user_dirs = UserDirs::new().unwrap();
    let home_path = user_dirs.home_dir().to_path_buf().join(".for-mqtt");

    let fs_path = home_path.clone();
    let fs = FileSpec::default()
        .directory(fs_path)
        .basename("for-mqtt")
        .suffix("log");
    // 若为true，则会覆盖rotate中的数字、keep^
    let criterion = Criterion::AgeOrSize(Age::Day, 10_000_000);
    let naming = Naming::Numbers;
    let cleanup = Cleanup::KeepLogFiles(2);
    let append = true;

    let _logger = custom_utils::logger::logger_feature_with_path(
        "for-mqtt",
        Debug,
        Info,
        home_path.clone(),
        home_path.clone(),
    )
    .module("sled", Info)
    .config(fs, criterion, naming, cleanup, append)
    .log_to_write(Box::new(CustomWriter(tx.clone())))
    .build();

    panic::set_hook(Box::new(|panic_info| {
        error!("{:?}", Backtrace::new());
        if let Some(location) = panic_info.location() {
            error!(
                "panic occurred in file '{}' at line {}",
                location.file(),
                location.line(),
            );
        }
        exit(1);
    }));

    let mut config = Config::init(home_path.clone());
    if config.display_tips {
        config.display_tips = false;
        config.update(home_path.clone());
        tx.send(AppEvent::OtherDisplayTips).unwrap();
    }

    let locale = get_locale();
    let win = WindowDesc::new(init_layout(tx.clone(), locale.clone())) //.background(B_WINDOW))
        .title("for-mqtt")
        .window_size((1200.0, 710.0)); //.menu(menu);
    let mut db = ArcDb::init_db(tx.clone(), home_path.join("db"))?;
    let mut data = db.read_app_data()?;

    let launcher = AppLauncher::with_window(win)
        .configure_env(|_env: &mut Env, _data: &AppData| {
            // env.set(WINDOW_BACKGROUND_COLOR, WHITE);
        })
        .delegate(Delegate(locale));
    let event_sink = launcher.get_external_handle();
    thread::Builder::new()
        .name("logic-worker".to_string())
        .spawn(move || {
            if let Err(e) = deal_event(event_sink, rx, tx) {
                error!("{:?}", e);
            }
        })
        .unwrap();

    data.init_broker();

    launcher.launch(data)?;
    Ok(())
}

pub struct Delegate(Locale);
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
            let new_win = WindowDesc::new(tips::tips_ui_builder(self.0.clone()))
                .window_size((500.0, 240.0))
                .resizable(false)
                .set_always_on_top(true);
            _ctx.new_window(new_win);
            return Handled::Yes;
        }
        Handled::No
    }
}

// fn make_menu(_: Option<WindowId>, _state: &AppData, _: &Env) -> Menu<AppData> {
//     let custom = Menu::new("abc")
//         .entry(MenuItem::new("Tips").on_activate(|ctx, _data, _env| ctx.submit_command(TIPS)));
//     custom
// }
