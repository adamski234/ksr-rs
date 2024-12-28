use std::{fs::{self, read_to_string}, io::ErrorKind, path::Path};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Label {
	pub name: String,
	#[serde(rename = "supportPoints")]
	pub support_points: Vec<(f64, f64)>
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct LinguisticVariable {
	pub name: String,
	pub labels: Vec<Label>,
	#[serde(rename = "uodBounds")]
	pub uod_bounds: (f64, f64),
	pub parameter: Option<Parameter>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum Parameter {
	DissolvedOxygen,
	Conductance,
	#[allow(non_camel_case_types)]
	pH,
	WaterTemperature,
	DissolvedChloride,
	TotalAlkalinity,
	TotalHardness,
	DissolvedCalcium,
	DissolvedMagnesium,
	DissolvedSodium,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct VariableFile {
	#[serde(rename = "linguisticVariables")]
	pub linguistic_variables: Vec<LinguisticVariable>,
	pub quantifiers: Vec<LinguisticVariable>,
}

impl VariableFile {
	pub fn parse_file(file: &Path) -> Result<Self, FileLoadError> {
		match read_to_string(file) {
			Ok(json_string) => {
				match serde_json::from_str(&json_string) {
					Ok(data) => return Ok(data),
					Err(data) => {
						match data.classify() {
							serde_json::error::Category::Io => panic!("serde IO error from in-memory parsing: {:?}", data),
							_ => return Err(FileLoadError::InvalidJSON),
						}
					}
				}
			}
			Err(data) => {
				match data.kind() {
					std::io::ErrorKind::NotFound => return Err(FileLoadError::FileNotFound),
					std::io::ErrorKind::PermissionDenied => return Err(FileLoadError::PermissionDenied),
					other_error => return Err(FileLoadError::OtherError(other_error))
				}
			}
		}
	}
	
	pub fn save_file(&self, file: &Path) -> Result<(), FileSaveError> {
		match fs::write(file, serde_json::to_string_pretty(self).unwrap()) {
			Ok(_) => {
				return Ok(());
			}
			Err(data) => {
				match data.kind() {
					ErrorKind::PermissionDenied => return Err(FileSaveError::PermissionDenied),
					ErrorKind::ReadOnlyFilesystem => return Err(FileSaveError::ReadOnlyFilesystem),
					ErrorKind::StorageFull => return Err(FileSaveError::StorageFull),
					ErrorKind::QuotaExceeded => return Err(FileSaveError::QuotaExceeded),
					ErrorKind::FileTooLarge => return Err(FileSaveError::FileTooLarge),
					other => return Err(FileSaveError::OtherError(other)),
				}
			}
		}
	}
}

#[derive(Debug, Clone)]
pub enum FileLoadError {
	FileNotFound,
	PermissionDenied,
	InvalidJSON,
	OtherError(ErrorKind)
}

#[derive(Debug, Clone)]
pub enum FileSaveError {
	PermissionDenied,
	ReadOnlyFilesystem,
	StorageFull,
	QuotaExceeded,
	FileTooLarge,
	OtherError(ErrorKind)
}