use wasm_bindgen::prelude::*;
use surrealdb::sql::{Array, Value};
use crate::err::Error;
use crate::app::opt::to_value::to_value;

#[wasm_bindgen(typescript_custom_section)]
const ITEXT_STYLE: &'static str = r#"

type SuperUserAuth = {
	username: string;
	password: string;
};

type NamespaceUserAuth = {
	namespace: string;
	username: string;
	password: string;
};

type DatabaseUserAuth = {
	namespace: string;
	database: string;
	username: string;
	password: string;
};

type ScopeUserAuth = {
	namespace: string;
	database: string;
	scope: string;
	[k: string]: unknown;
};

type AnyAuth = SuperUserAuth | NamespaceUserAuth | DatabaseUserAuth | ScopeUserAuth;

type CapabilitiesAllowDenyList = {
	allow?: boolean | string[];
	deny?: boolean | string[];
};

type ConnectionOptions = {
    capacity?: number;
	strict?: boolean;
	notifications?: boolean;
	query_timeout?: number;
	transaction_timeout?: number;
	tick_interval?: number;
	user?: AnyAuth;
	capabilities?: boolean | {
		guest_access?: boolean;
		functions?: boolean | string[] | CapabilitiesAllowDenyList;
		network_targets?: boolean | string[] | CapabilitiesAllowDenyList;
	}
}

type UseOptions = {
	namespace?: string;
	database?: string;
};

type BasePatch<T = string> = {
	path: T;
};

export type AddPatch<T = string, U = unknown> = BasePatch<T> & {
	op: "add";
	value: U;
};

export type RemovePatch<T = string> = BasePatch<T> & {
	op: "remove";
};

export type ReplacePatch<T = string, U = unknown> = BasePatch<T> & {
	op: "replace";
	value: U;
};

export type ChangePatch<T = string, U = string> = BasePatch<T> & {
	op: "change";
	value: U;
};

export type Patch =
	| AddPatch
	| RemovePatch
	| ReplacePatch
	| ChangePatch;
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ConnectionOptions")]
    pub type TConnectionOptions;
    #[wasm_bindgen(typescript_type = "AnyAuth")]
    pub type TAnyAuth;
    #[wasm_bindgen(typescript_type = "ScopeUserAuth")]
    pub type TScopeUserAuth;
    #[wasm_bindgen(typescript_type = "UseOptions")]
    pub type TUseOptions;
    #[wasm_bindgen(typescript_type = "unknown")]
    pub type TUnknown;
    #[wasm_bindgen(typescript_type = "Record<string, unknown>")]
    pub type TRecordUnknown;
	#[wasm_bindgen(typescript_type = "unknown[]")]
	pub type TArrayUnknown;
	#[wasm_bindgen(typescript_type = "Record<string, unknown>[]")]
	pub type TArrayRecordUnknown;
	#[wasm_bindgen(typescript_type = "Patch[]")]
	pub type TArrayPatch;
}

impl TArrayUnknown {
	pub fn from_value(value: Value) -> Result<Self, Error> {
		let value = match value {
			Value::Array(_) => value,
			_ => Value::Array(Array::from(value)),
		};

		let value = to_value(&value.into_json())?;
		Ok(value.into())
	}
}

impl TArrayRecordUnknown {
	pub fn from_value(value: Value) -> Result<Self, Error> {
		let value = match value {
			Value::Array(v) => v,
			_ => Array::from(value),
		};

		for v in value.clone() {
			if !matches!(v, Value::Object(_)) {
				return Err(Error::from("Encountered a non-object value in array"))
			}
		}

		let value = Value::Array(value);
		let value = to_value(&value.into_json())?;
		Ok(value.into())
	}
}
