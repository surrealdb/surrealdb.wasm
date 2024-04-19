use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const ITEXT_STYLE: &'static str = r#"
	type CapabilitiesAllowDenyList = {
		allow?: boolean | string[];
		deny?: boolean | string[];
	};

	type ConnectionOptions = {
		strict?: boolean;
		query_timeout?: number;
		transaction_timeout?: number;
		capabilities?: boolean | {
			guest_access?: boolean;
			functions?: boolean | string[] | CapabilitiesAllowDenyList;
			network_targets?: boolean | string[] | CapabilitiesAllowDenyList;
		}
	}
"#;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(typescript_type = "ConnectionOptions")]
	pub type TsConnectionOptions;
}
