use chrono::Local;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::fs::{self, File};
use std::io::{Error, Write};
use std::path::{Path, PathBuf};
use tar::Builder;

fn create_archive(log_directory: &Path) -> Result<(), Error> {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let archive_name = format!("logs_archive_{}.tar.gz", timestamp);

    let archive_dir = log_directory.parent().unwrap().join("log_archives");
    fs::create_dir_all(&archive_dir)?;

    let archive_path = archive_dir.join(&archive_name);

    let tar_gz = File::create(&archive_path)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = Builder::new(enc);

    tar.append_dir_all("logs", log_directory)?;
    tar.finish()?;

    let log_file = archive_dir.join("archive_log.txt");
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(log_file)?;

    writeln!(file, "{}: Created archive {}", timestamp, archive_name)?;

    println!("Archive created: {}", archive_path.display());

    Ok(())
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <log-directory>", args[0]);
        std::process::exit(1);
    }

    let log_directory = PathBuf::from(&args[1]);

    if !log_directory.is_dir() {
        eprintln!(
            "Error: {} is not a valid directory",
            log_directory.display()
        );
        std::process::exit(1);
    }

    create_archive(&log_directory)?;

    Ok(())
}
