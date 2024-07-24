use std::fmt::Display;

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Clone)]
pub enum SystemAccessError {
    CouldntFindFile(SystemFileName),
    BadFolderPath(FolderToAccess),
    CouldntParseFile(SystemFileName),
    MismatchingPostfix(MismatchError),
}

impl Display for SystemAccessError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SystemAccessError::CouldntFindFile(system_file_name) => {
                write!(
                    f,
                    "couldn't find file {}",
                    system_file_name.name_with_postfix
                )
            }
            SystemAccessError::BadFolderPath(folder_to_access) => {
                write!(f, "bad folder file for {}", folder_to_access)
            }
            SystemAccessError::CouldntParseFile(system_file_name) => {
                write!(f, "couldn't parse {}", system_file_name.name_with_postfix)
            }
            SystemAccessError::MismatchingPostfix(mismatch_error) => mismatch_error.fmt(f),
        }
    }
}
