use surrealdb::Error as DBError;
use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Error, Debug)]
pub enum Error {
	#[error("{0}")]
	DBError(#[from] DBError),

	#[error("Invalid URL")]
	InvalidUrl,

	#[error("You need to instantiate this instance using the init(url) method")]
	MustInstantiate,
}

impl From<Error> for JsValue {
	fn from(v: Error) -> Self {
		JsValue::from(v.to_string())
	}
}
