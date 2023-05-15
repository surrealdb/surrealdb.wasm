use serde::Deserialize;
// TODO switch to `sql::Value` once we implement a deserialiser for it
use serde_json::Value;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Credentials {
	Scope {
		namespace: String,
		database: String,
		scope: String,
		#[serde(flatten)]
		params: Value,
	},
	Database {
		namespace: String,
		database: String,
		username: String,
		password: String,
	},
	Namespace {
		namespace: String,
		username: String,
		password: String,
	},
	Root {
		username: String,
		password: String,
	},
}
