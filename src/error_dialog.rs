use common::{data::SampleLoadError, variables::{FileLoadError, FileSaveError}};
use rfd::MessageButtons;

pub trait ToErrorMessage {
    fn to_description(&self) -> String;
}

impl ToErrorMessage for FileLoadError {
    fn to_description(&self) -> String {
        use FileLoadError::*;
        match self {
            FileNotFound => return String::from("The requested file could not be found."),
            InvalidJSON => return String::from("The requested file did not contain valid JSON."),
            PermissionDenied => return String::from("Permission denied when trying to access file."),
            OtherError(error_data) => return format!("Unknown error:\n {:#?}", error_data),
        }
    }
}

impl ToErrorMessage for FileSaveError {
    fn to_description(&self) -> String {
        use FileSaveError::*;
        match self {
            PermissionDenied => return String::from("Permission denied when trying to write to file."),
            ReadOnlyFilesystem => return String::from("The selected file is on a read-only filesystem."),
            StorageFull => return String::from("Storage is full."),
            QuotaExceeded => return String::from("Quota exceeded."),
            FileTooLarge => return String::from("The file is too large."),
            OtherError(error_data) => return format!("Unknown error:\n {:#?}", error_data),
        }
    }
}

impl ToErrorMessage for SampleLoadError {
    fn to_description(&self) -> String {
        use SampleLoadError::*;
        match self {
            Configuration(err) => return format!("Configuration string error:\n{}", err),
            Database(err) => return format!("Database error:\n{}", err),
            Protocol(err) => return format!("Protocol error:\n{}", err),
            OtherError(err) => return format!("Unknown error:\n {}", err),
            ColumnMissing(err) => return format!("Column is missing:\n {}", err),
            ColumnDecode { index, source } => return format!("Could not decode column {}:\n {}", index, source),
            Decode(err) => return format!("Decoding error:\n {}", err),
            FileNotFound => return String::from("The requested file could not be found."),
            PermissionDenied => return String::from("Permission denied when trying to access file."),
            EmptyTable => return String::from("The database contained a valid but empty table."),
            Unknown => return String::from("Unknown error reading database."),
        }
    }
}

pub async fn show_error_dialog(title: impl Into<String>, error: impl ToErrorMessage) {
    rfd::AsyncMessageDialog::new()
        .set_title(title)
        .set_description(error.to_description())
        .set_buttons(MessageButtons::Ok)
        .show()
        .await;
}
