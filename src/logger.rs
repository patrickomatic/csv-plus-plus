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
    INIT.call_once(|| {
        let mut builder = env_logger::Builder::new();

        builder.filter_level(verbosity);

        // turn off timestamps and log level if we're just showing errors
        if verbosity == log::LevelFilter::Error {
            builder.format_level(false).format_timestamp(None);
        }

        // turn off module_paths unless we're at debug or below
        if verbosity != log::LevelFilter::Debug && verbosity != log::LevelFilter::Trace {
            builder.format_target(false);
        }

        builder.init()
    });
}
