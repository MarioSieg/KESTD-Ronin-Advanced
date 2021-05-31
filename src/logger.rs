use log::warn;
use std::fs;
use std::path::Path;

pub fn level_filter() -> log::LevelFilter {
    #[cfg(debug_assertions)]
    {
        log::LevelFilter::Trace
    }
    #[cfg(not(debug_assertions))]
    {
        log::LevelFilter::Info
    }
}

pub const DIR: &str = "proto";

pub fn create() -> Result<(), log::SetLoggerError> {
    use colors::*;
    use fern::*;
    let mut colors = ColoredLevelConfig::new().info(Color::Green);
    colors.warn = Color::Magenta;
    colors.info = Color::BrightBlue;

    let log_dir = Path::new(DIR);
    if !log_dir.exists() && fs::create_dir(log_dir).is_err() {
        warn!(
            "Failed to create log directory: {:?}! Log file creation might fail too!",
            log_dir
        );
    }

    let log_file_path = String::from(log_dir.to_str().unwrap_or_default())
        + &chrono::Local::now()
            .format("/engine_session_%Y_%m_%d_%H_%M_%S.log")
            .to_string();
    let log_file = fern::log_file(&log_file_path);

    #[allow(unused_mut)]
    let mut dispatch = Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(level_filter())
        .chain(std::io::stdout());

    if let Ok(file) = log_file {
        dispatch = dispatch.chain(file);
    } else {
        warn!("Failed to create log file: {:?}", log_file_path);
    }

    #[cfg(not(debug_assertions))]
    {
        dispatch = dispatch.level_for("gfx_backend_vulkan", LevelFilter::Off);
        dispatch = dispatch.level_for("gfx_backend_dx11", LevelFilter::Off);
        dispatch = dispatch.level_for("gfx_backend_dx12", LevelFilter::Off);
        dispatch = dispatch.level_for("naga", LevelFilter::Off);
    }

    dispatch.apply()
}
