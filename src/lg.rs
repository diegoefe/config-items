use serde::{Serialize, Deserialize};
use std::error::Error;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::Config as LogConfig;
use log4rs::config::{Appender, Logger, Root};
// use log::*;
use std::env;

use crate::data::get_log_filename;

fn default_level() -> String {
    "info".into()
}

/// Logging configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Logging {
    pub file_name: String,
    #[serde(default="default_level")]
    pub level: String
}
impl Logging {
    /// Initialize using current configuration
    pub fn init(&self) -> Result<(), Box<dyn Error>> {
        create_log_config_and_init(self)?;
        Ok(())
    }
}
impl Default for Logging {
    fn default() -> Self {
        Logging {
            file_name: get_log_filename().into(),
            level: default_level(),
        }
    }
}

fn match_error_string(es:&str) -> Result<LevelFilter, Box<dyn Error>> {
    let ll = match es {
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        "trace" => LevelFilter::Trace,
        _ => return Err(Box::from(format!("Invalid log level '{}'", es)))
    };
    Ok(ll)
}

/// Create a logging configuration
pub fn create_log_config(lg:&Logging)  -> Result< LogConfig, Box<dyn Error>>  {
    const PATTERN:&str = "[{d(%Y-%m-%d %H:%M:%S)} {l}] {m}{n}";

    let level_filter = match env::var("RUST_LOG") {
        Ok(rl) => match_error_string(&rl)?,
        Err(_) => match_error_string(lg.level.as_str())?
    };

    let stdout = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new(PATTERN)))
        .build();

    let requests = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new(PATTERN)))
        .build(&lg.file_name)?;

    let config = LogConfig::builder()
        .appender(Appender::builder()
            .build("stdout", Box::new(stdout)))
        .appender(Appender::builder()
             .build("requests", Box::new(requests)))
        .logger(Logger::builder()
            .build("app::backend::db", level_filter))
        .logger(Logger::builder()
                .appender("requests")
                .additive(false)
            .build("app::requests", level_filter))
        .build(Root::builder()
                .appender("stdout")
                .appender("requests")
            .build(level_filter))?;
    Ok(config)
}

/// Create a logging configuration and initialize logger
fn create_log_config_and_init(lg:&Logging)  -> Result<(), Box<dyn Error>>  {
    let lcfg = create_log_config(lg)?;
    match log4rs::init_config(lcfg) {
        Ok(_)=> Ok(()),
        Err(_) => todo!(),
    }        
}