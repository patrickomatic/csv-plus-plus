use crate::{Error, Result};
use std::sync;

pub(crate) fn u8_into_level_filter(v: u8) -> Result<log::LevelFilter> {
    Ok(match v {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3 => log::LevelFilter::Debug,
        4 => log::LevelFilter::Trace,
        _ => {
            return Err(Error::InitError(format!(
                "{v} is not a valid verbosity level - the most you can do is -vvvv"
            )))
        }
    })
}

static INIT: sync::Once = sync::Once::new();

pub(crate) fn init(verbosity: log::LevelFilter) {
    INIT.call_once(|| env_logger::Builder::new().filter(None, verbosity).init());
}
