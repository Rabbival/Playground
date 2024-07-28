use std::fmt::*;

#[derive(Debug)]
pub enum OsAccessLog {
    FolderCreated(String),
    FolderExists(String),
    WroteToFile(String),
    FileDeleted(String),
    FileCreated(String),
    AppendedToFile(String),
}

impl Display for OsAccessLog {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            OsAccessLog::FolderCreated(folder) => write!(f, "Folder created: {}", folder),
            OsAccessLog::FolderExists(folder) => write!(f, "Folder exists: {}", folder),
            OsAccessLog::WroteToFile(file) => write!(f, "Wrote to file: {}", file),
            OsAccessLog::FileDeleted(file) => write!(f, "File deleted: {}", file),
            OsAccessLog::FileCreated(file) => write!(f, "File created: {}", file),
            OsAccessLog::AppendedToFile(file) => write!(f, "Appended to file: {}", file),
        }
    }
}
