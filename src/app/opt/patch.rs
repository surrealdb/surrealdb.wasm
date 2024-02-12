use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
#[serde(tag = "op")]
#[serde(rename_all = "lowercase")]
pub enum Patch {
	Add {
		path: String,
		value: Value,
	},
	Remove {
		path: String,
	},
	Replace {
		path: String,
		value: String,
	},
	Change {
		path: String,
		value: String,
	},
}
