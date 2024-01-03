use std::fs::{self, DirEntry};
use std::io;
use std::path::PathBuf;
use std::time::SystemTime;
use std::ffi::OsString;
use chrono::{Utc, TimeZone, LocalResult};

pub fn get_desktop_path() -> PathBuf {
    PathBuf::from("/home/mike/projects/rust/file_reader/mock_desktop")
}


pub fn create_dir(path: &PathBuf) -> io::Result<()> {
    fs::create_dir(path)?;
    Ok(())
}

pub fn read_dir_contents(dir: &PathBuf) -> io::Result<Vec<DirEntry>> {
    let mut entries = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        entries.push(entry);
        
    }
    
    Ok(entries)
}

pub fn read_file_created_time(file: &DirEntry) -> io::Result<String> {
    let metadata = file.metadata()?;
    let created = metadata.created()?;
    
    match created.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => {
            let formatted_date = convert_timestamp_to_date(duration.as_secs() as i64);
            Ok(formatted_date)
        },
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
    
    
    
}

pub fn list_all_extensions(entries: &[DirEntry]) -> io::Result<Vec<OsString>> {
    let mut extensions = Vec::new();
    for entry in entries {
        if let Some(extension) = entry.path().extension() {
            let extension = extension.to_os_string();
            if !extensions.contains(&extension) {
                extensions.push(extension);
            }
        }
    }
    
    Ok(extensions)
}

fn convert_timestamp_to_date(timestamp: i64) -> String {
    match Utc.timestamp_opt(timestamp, 0) {
        LocalResult::Single(date_time) => {
            let formatted_date = date_time.format("%Y-%m-%d %H:%M:%S").to_string();
            formatted_date
        },
        _ => {
            // Handle the case where the timestamp is out of range or invalid
            String::from("Invalid date")
        },
    }
}