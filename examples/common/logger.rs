use chrono::Local;
use fern::colors::{Color, ColoredLevelConfig};

pub fn setup_logger() -> Result<(), log::SetLoggerError> {
    // Create dispatch
    let dispatch = fern::Dispatch::new();

    // Set colors
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red);

    // Set dispatch formatting
    dispatch
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .apply()
}
