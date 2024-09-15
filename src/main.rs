use chrono::Local;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::fs::{self, File};
use std::io::{Error, Write};
use std::path::{Path, PathBuf};
use tar::Builder;

/// Create an archive of the specified log directory
fn create_archive(log_directory: &Path) -> Result<(), Error> {
    // Generate a timestamp for the archive name
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let archive_name = format!("logs_archive_{}.tar.gz", timestamp);

    // Create the archive directory  if  it doesn't exist
    let archive_dir = log_directory.parent().unwrap().join("log_archives");
    fs::create_dir_all(&archive_dir)?;

    // Set the full path for the archive file
    let archive_path = archive_dir.join(&archive_name);

    // Create a GzEncoder to compress the archive
    let tar_gz = File::create(&archive_path)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = Builder::new(enc);

    // Add the entire log directory to the tar archive
    tar.append_dir_all("logs", log_directory)?;
    // Finish writing the tar archive
    tar.finish()?;

    // Open (or create) the log file
    let log_file = archive_dir.join("archive_log.txt");
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(log_file)?;

    // Write a log entry for this archive creation
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
