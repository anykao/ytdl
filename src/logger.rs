use fern;
use fern::colors::{Color, ColoredLevelConfig};
use log;

pub fn init() {
    let colors = ColoredLevelConfig::new()
        .debug(Color::Magenta)
        .info(Color::Green);
    // let colors = ColoredLevelConfig::default();

    // Configure logger at runtime
    fern::Dispatch::new()
    // Perform allocation-free log formatting
    .format(move |out, message, record| {
        out.finish(format_args!(
                "[{}][{}:{}][{}] {}",
                // chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.file().unwrap_or("_"),
                record.line().unwrap_or(0),
                colors.color(record.level()),
                message
        ))
    })
    // Add blanket level filter -
    .level(log::LevelFilter::Debug)
    // - and per-module overrides
    // .level_for("hyper", log::LevelFilter::Info)
    // Output to stdout, files, and other Dispatch configurations
    .chain(::std::io::stdout())
    // .chain(fern::log_file("output.log")?)
    // Apply globally
    .apply().unwrap();
}
