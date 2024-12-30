use common::variables::{FileLoadError, FileSaveError};
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
            PermissionDenied => {return String::from("Permission denied when trying to access file.");}
            OtherError(error_data) => return format!("Unknown error:\n {:#?}", error_data),
        }
    }
}

impl ToErrorMessage for FileSaveError {
    fn to_description(&self) -> String {
        use FileSaveError::*;
        match self {
            PermissionDenied => {return String::from("Permission denied when trying to write to file.");}
            ReadOnlyFilesystem => {return String::from("The selected file is on a read-only filesystem.");}
            StorageFull => return String::from("Storage is full."),
            QuotaExceeded => return String::from("Quota exceeded."),
            FileTooLarge => return String::from("The file is too large."),
            OtherError(error_data) => return format!("Unknown error:\n {:#?}", error_data),
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
