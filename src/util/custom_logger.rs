use crate::data::AppEvent;
use crossbeam_channel::Sender;
use flexi_logger::writers::LogWriter;
use flexi_logger::{DeferredNow, FormatFunction, Logger};
use log::{debug, info, Record};

pub struct CustomWriter(pub Sender<AppEvent>);

impl LogWriter for CustomWriter {
    fn write(&self, now: &mut DeferredNow, record: &Record) -> std::io::Result<()> {
        if record.level() <= self.max_log_level() {
            if let Err(_) = self
                .0
                .send(AppEvent::UpdateStatusBar(record.args().to_string()))
            {}
        }
        Ok(())
    }

    fn flush(&self) -> std::io::Result<()> {
        Ok(())
    }

    fn max_log_level(&self) -> log::LevelFilter {
        log::LevelFilter::Info
    }
}
