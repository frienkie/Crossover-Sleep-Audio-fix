//! Write control-plane diagnostics to the native unified log without caches.
use std::ffi::CString;

struct Logger;
static LOGGER: Logger = Logger;

unsafe extern "C" {
    fn crosspuck_audio_log(level: u8, message: *const std::ffi::c_char);
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &log::Record<'_>) {
        if self.enabled(record.metadata()) {
            if let Ok(message) = CString::new(record.args().to_string()) {
                // SAFETY: The native function uses this string only during the call.
                unsafe { crosspuck_audio_log(record.level() as u8, message.as_ptr()) };
            }
        }
    }

    fn flush(&self) {}
}

pub fn init() {
    let level = std::env::var("CROSSPUCK_LOG_LEVEL")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(log::LevelFilter::Info);
    if log::set_logger(&LOGGER).is_ok() {
        log::set_max_level(level);
    }
}
