use serde::Deserialize;

#[derive(Deserialize)]
pub struct Use {
	pub ns: Option<String>,
	pub db: Option<String>,
}
