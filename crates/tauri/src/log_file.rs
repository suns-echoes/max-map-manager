use chrono;
use fern;
use log::LevelFilter;
use std::fs;
use std::path::PathBuf;

pub fn setup_logger() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("logs")?;

    let log_file_path = PathBuf::from("logs/dmesg.log");

    if log_file_path.exists() {
        fs::write(&log_file_path, b"")?;
    }

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(LevelFilter::Info)
        .chain(fern::log_file(log_file_path)?)
        .apply()?;

    Ok(())
}
