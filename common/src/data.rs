use std::{io::ErrorKind, path::Path};

use sqlx::{query_as, Connection};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Sample {
	pub sample_id: String,
	pub dissolved_oxygen: f64,
	pub conductance: f64,
	pub ph: f64,
	pub water_temperature: f64,
	pub dissolved_chloride: f64,
	pub total_alkalinity: f64,
	pub total_hardness: f64,
	pub dissolved_calcium: f64,
	pub dissolved_magnesium: f64,
	pub dissolved_sodium: f64,
	pub area: SampleArea,
}

#[derive(Debug, Clone, sqlx::Decode)]
pub enum SampleArea {
	North,
	South
}

impl From<String> for SampleArea {
	fn from(value: String) -> Self {
		match value.as_str() {
			"North" => return Self::North,
			"South" => return Self::South,
			val => panic!("Could not parse SampleArea from string \"{}\"", val),
		}
	}
}

#[derive(Debug, Clone)]
pub enum SampleLoadError {
	Configuration(String),
	Database(String),
	Protocol(String),
	OtherError(String),
	ColumnMissing(String),
	ColumnDecode { index: String, source: String },
	Decode(String),
	FileNotFound,
	PermissionDenied,
	EmptyTable,
	Unknown,
}

impl Sample {
	pub async fn load_from_db(db_path: &Path) -> Result<Vec<Self>, SampleLoadError> {
		use sqlx::Error::*;
		match sqlx::SqliteConnection::connect(db_path.to_str().unwrap()).await {
			Ok(mut database) => {
				match query_as!(Self, "SELECT * FROM samples").fetch_all(&mut database).await {
					Ok(data) => return Ok(data),
					Err(Database(database_error)) => return Err(SampleLoadError::Database(format!("{:?}", database_error))),
					Err(Protocol(error)) => return Err(SampleLoadError::Protocol(error)),
					Err(RowNotFound) => return Err(SampleLoadError::EmptyTable),
					Err(ColumnNotFound(column)) => return Err(SampleLoadError::ColumnMissing(column)),
					Err(ColumnDecode { index, source }) => return Err(SampleLoadError::ColumnDecode { index, source: format!("{:?}", source) }),
					Err(Decode(error)) => return Err(SampleLoadError::Decode(format!("{:?}", error))),
					_ => return Err(SampleLoadError::Unknown),
				};
			},
			Err(Configuration(error)) => return Err(SampleLoadError::Configuration(format!("{:?}", error))),
			Err(Database(database_error)) => return Err(SampleLoadError::Database(format!("{:?}", database_error))),
			Err(Protocol(error)) => return Err(SampleLoadError::Protocol(error)),
			Err(Io(error)) => {
				match error.kind() {
					ErrorKind::NotFound => return Err(SampleLoadError::FileNotFound),
					ErrorKind::PermissionDenied => return Err(SampleLoadError::PermissionDenied),
					other_error => return Err(SampleLoadError::OtherError(format!("{:?}", other_error))),
				}
			},
			_ => return Err(SampleLoadError::Unknown),
		}
	}
}