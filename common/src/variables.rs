use std::{fs::read_to_string, io::ErrorKind, path::Path};

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

#[derive(Debug, Clone)]
pub enum FileLoadError {
	FileNotFound,
	PermissionDenied,
	InvalidJSON,
	OtherError(ErrorKind)
}

pub fn parse_file(file: &Path) -> Result<VariableFile, FileLoadError> {
	match read_to_string(file) {
		Ok(json_string) => {
			match serde_json::from_str(&json_string) {
				Ok(data) => {
					return Ok(data);
				}
				Err(data) => {
					match data.classify() {
						serde_json::error::Category::Io => panic!("serde IO error from in-memory parsing: {:?}", data),
						_ => {
							return Err(FileLoadError::InvalidJSON);
						}
					}
				}
			}
		}
		Err(data) => {
			match data.kind() {
				std::io::ErrorKind::NotFound => {
					return Err(FileLoadError::FileNotFound);
				},
				std::io::ErrorKind::PermissionDenied => {
					return Err(FileLoadError::PermissionDenied);
				},
				other_error => {
					return Err(FileLoadError::OtherError(other_error));
				},
			}
		}
	}
}