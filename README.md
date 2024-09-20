# Log Archive Tool
A tool to archive logs on a set schedule by compressing them and storing them in a new directory.

# Requirements
- [x] Provide the log directory as an argument
- [x] Compress the logs to a tar.gz file and store in a new directory
- [x] Log date and time of the archive to the file

# Tool Requirements
- Rust Language

# Installation
1. Clone the repositry
``` bash
  git clone https://github.com/Cyber-Freak999/log-archive
  cd log-archive
```
2. Build the project
``` bash
  cargo build --release
```
3. (Optional) Install the binary system-wide
``` bash
  sudo cp target/release/log_archive /usr/local/bin/log-archive
```

# Usage
``` bash
  log-archive /path/to/logs
```

## Example
``` bash
  sudo log-archive /var/log
```

This will create a compressed archive of the logs in a new directory called `log_archives`, located in the parent directory of the specified log directory.\
The archive will be named in the format: `logs_archive_YYYYMMDD_HHMMSS.tar.gz`\
A log of the archive creation will be added to `log_archives/archive_log.txt`.

[Link](https://roadmap.sh/projects/log-archive-tool)
