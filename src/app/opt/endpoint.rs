use serde::Deserialize;

#[derive(Deserialize)]
pub struct Options {
	pub capacity: Option<usize>,
	pub strict: Option<bool>,
}
