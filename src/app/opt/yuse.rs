use serde::Deserialize;

#[derive(Deserialize)]
pub struct Use {
	pub namespace: Option<String>,
	pub database: Option<String>,
}
