use dmp::Diff;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
#[serde(remote = "Diff")]
struct DiffDef {
	operation: i32,
	text: String,
}

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
		value: Value,
	},
	Change {
		path: String,
		#[serde(with = "DiffDef")]
		diff: Diff,
	},
}
