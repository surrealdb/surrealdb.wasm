use serde::Serialize;
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::prelude::JsValue;

// Converts a Rust value into a [`JsValue`].
pub fn to_value<T: Serialize + ?Sized>(value: &T) -> Result<JsValue, serde_wasm_bindgen::Error> {
	value.serialize(&Serializer::json_compatible())
}
