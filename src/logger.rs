use std::sync;

// TODO: maybe validate by throwing an error if it's more than -vvvv
pub(crate) fn u8_into_level_filter(v: u8) -> log::LevelFilter {
    match v {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3 => log::LevelFilter::Debug,
        4..=std::u8::MAX => log::LevelFilter::Trace,
    }
}

static INIT: sync::Once = sync::Once::new();

pub(crate) fn init(verbosity: log::LevelFilter) {
    INIT.call_once(|| env_logger::Builder::new().filter(None, verbosity).init());
}
