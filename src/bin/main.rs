use druid::{AppLauncher, PlatformError, WindowDesc};
use for_mqtt::logic::deal_event;
use for_mqtt::ui::init_layout;
use for_mqtt::util::db::ArcDb;
use std::thread;

#[tokio::main]
async fn main() -> Result<(), PlatformError> {
    custom_utils::logger::logger_stdout_debug();
    let win = WindowDesc::new(init_layout()); //.menu(menu);

    let (tx, rx) = std::sync::mpsc::channel();
    let mut db = ArcDb::init_db(tx)?;
    let data = db.read_app_data()?;

    let launcher = AppLauncher::with_window(win);
    let event_sink = launcher.get_external_handle();
    tokio::spawn(deal_event(event_sink, rx));

    launcher.launch(data)?;
    Ok(())
}
