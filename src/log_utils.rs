use colored::{Colorize, ColoredString};
use env_logger::{Builder, Target};
use log::{LevelFilter, Level};
use std::io::Write;
use chrono::Local;

fn colornize_by_level(content: String,level: log::Level) -> ColoredString{
        match level {
            Level::Trace => content.cyan(),
            Level::Debug => content.blue(),
            Level::Info => content.green(),
            Level::Warn => content.yellow(),
            Level::Error => content.red().bold(),
        }
}

pub fn init_log(){
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.filter_level(LevelFilter::Debug);
    builder.format(|buf, record| {
        let log = format!(
            "{} [{:>5}] - {}", 
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.args()
        );
        writeln!(buf,
            "{}",
            colornize_by_level(log, record.level())
        )
    });
    builder.init();
}