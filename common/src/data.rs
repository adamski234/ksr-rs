use std::path::Path;

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

impl Sample {
	pub async fn load_from_db(db_path: &Path) -> Vec<Self> {
		let mut database = sqlx::SqliteConnection::connect(db_path.to_str().unwrap()).await.unwrap();
		let data = query_as!(Self, "SELECT * FROM samples").fetch_all(&mut database).await.unwrap();
		return data;
	}
}