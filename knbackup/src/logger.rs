use std::env;

use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
    Config,
};

pub fn init() {
    let log_level = if let Ok(rust_log) = env::var("RUST_LOG"){
        match rust_log.to_lowercase().as_str() {
            "off" => log::LevelFilter::Off,
            "error" => log::LevelFilter::Error,
            "warn" => log::LevelFilter::Warn,
            "info" => log::LevelFilter::Info,
            "debug" => log::LevelFilter::Debug,
            "trace" => log::LevelFilter::Trace,
            _ => log::LevelFilter::Info,
        }
    } else {
        log::LevelFilter::Info
    };

    let root = Root::builder().appender("stdout").build(log_level);

    let console_appender = ConsoleAppender::builder()
        .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new(
            "[{l}][{d(%H:%M:%S)}][{T}][{t}] {m}{n}",
        )))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(console_appender)))
        .build(root)
        .unwrap();

    log4rs::init_config(config).unwrap();
}
