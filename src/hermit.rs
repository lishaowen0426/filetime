use crate::FileTime;
use std::fs::{self, File};
use std::io;
use std::os::hermit::fs::MetadataExt;
use std::path::Path;

pub fn set_file_times(p: &Path, atime: FileTime, mtime: FileTime) -> io::Result<()> {
    Ok(())
}

pub fn set_file_mtime(p: &Path, mtime: FileTime) -> io::Result<()> {
    Ok(())
}

pub fn set_file_atime(p: &Path, atime: FileTime) -> io::Result<()> {
    Ok(())
}

pub fn set_symlink_file_times(p: &Path, atime: FileTime, mtime: FileTime) -> io::Result<()> {
    Ok(())
}

pub fn set_file_handle_times(
    f: &File,
    atime: Option<FileTime>,
    mtime: Option<FileTime>,
) -> io::Result<()> {
    Ok(())
}

pub fn from_last_modification_time(meta: &fs::Metadata) -> FileTime {
    FileTime {
        seconds: meta.mtime(),
        nanos: meta.mtime_nsec() as u32,
    }
}

pub fn from_last_access_time(meta: &fs::Metadata) -> FileTime {
    FileTime {
        seconds: meta.atime(),
        nanos: meta.atime_nsec() as u32,
    }
}

pub fn from_creation_time(_meta: &fs::Metadata) -> Option<FileTime> {
    None
}
