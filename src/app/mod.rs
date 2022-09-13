mod convert;
mod kind;
mod local;
mod log;
mod remote;

use crate::app::convert::Convert;
use crate::app::kind::Kind;
use crate::app::local::Local;
use crate::app::remote::Remote;
use crate::err::Error;
use js_sys::Promise;
use std::rc::Rc;
use std::sync::RwLock;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen]
pub fn setup() {
	self::log::init();
}

#[wasm_bindgen]
pub struct Surreal {
	kind: Kind,
}

#[wasm_bindgen]
impl Surreal {
	#[wasm_bindgen(constructor)]
	pub fn new(url: JsValue) -> Surreal {
		// Attempt to convert the URL string
		let url: String = url.convert();
		// Return a new instance of Surreal
		Surreal {
			kind: match &url[..] {
				s if s.starts_with("ws:") => {
					let v = Rc::new(RwLock::new(Remote::new(url)));
					Kind::Remote(v)
				}
				s if s.starts_with("wss:") => {
					let v = Rc::new(RwLock::new(Remote::new(url)));
					Kind::Remote(v)
				}
				s if s.starts_with("ixdb:") => {
					let v = Rc::new(RwLock::new(Local::new(url)));
					Kind::Local(v)
				}
				"memory" => {
					let v = Rc::new(RwLock::new(Local::new(url)));
					Kind::Local(v)
				}
				_ => Kind::None,
			},
		}
	}

	#[wasm_bindgen]
	pub fn init(&mut self, url: JsValue) -> Promise {
		// Attempt to convert the URL string
		let url: String = url.convert();
		// Attempt to instantiate the instance
		match &url[..] {
			s if s.starts_with("ws:") => {
				let v = Rc::new(RwLock::new(Remote::new(url)));
				self.kind = Kind::Remote(v);
				Promise::resolve(&JsValue::NULL)
			}
			s if s.starts_with("wss:") => {
				let v = Rc::new(RwLock::new(Remote::new(url)));
				self.kind = Kind::Remote(v);
				Promise::resolve(&JsValue::NULL)
			}
			s if s.starts_with("ixdb:") => {
				let v = Rc::new(RwLock::new(Local::new(url)));
				self.kind = Kind::Local(v);
				Promise::resolve(&JsValue::NULL)
			}
			"memory" => {
				let v = Rc::new(RwLock::new(Local::new(url)));
				self.kind = Kind::Local(v);
				Promise::resolve(&JsValue::NULL)
			}
			_ => {
				self.kind = Kind::None;
				Promise::reject(&JsValue::from(Error::InvalidUrl))
			}
		}
	}

	#[wasm_bindgen(js_name = use)]
	pub fn yuse(&self, ns: Option<String>, db: Option<String>) -> Promise {
		match &self.kind {
			Kind::None => Promise::reject(&JsValue::from(Error::MustInstantiate)),
			Kind::Local(v) => future_to_promise(Local::yuse(v.clone(), ns, db)),
			Kind::Remote(_) => todo!(),
		}
	}

	#[wasm_bindgen]
	pub fn sql(&self, text: JsValue, vars: JsValue) -> Promise {
		match &self.kind {
			Kind::None => Promise::reject(&JsValue::from(Error::MustInstantiate)),
			Kind::Local(v) => future_to_promise(Local::sql(v.clone(), text, vars)),
			Kind::Remote(_) => todo!(),
		}
	}

	#[wasm_bindgen]
	pub fn info(&self) -> Promise {
		match &self.kind {
			Kind::None => Promise::reject(&JsValue::from(Error::MustInstantiate)),
			Kind::Local(_) => Promise::reject(&JsValue::NULL),
			Kind::Remote(_) => todo!(),
		}
	}

	#[wasm_bindgen]
	pub fn signup(&self) -> Promise {
		match &self.kind {
			Kind::None => Promise::reject(&JsValue::from(Error::MustInstantiate)),
			Kind::Local(_) => Promise::reject(&JsValue::NULL),
			Kind::Remote(_) => todo!(),
		}
	}

	#[wasm_bindgen]
	pub fn signin(&self) -> Promise {
		match &self.kind {
			Kind::None => Promise::reject(&JsValue::from(Error::MustInstantiate)),
			Kind::Local(_) => Promise::reject(&JsValue::NULL),
			Kind::Remote(_) => todo!(),
		}
	}

	#[wasm_bindgen]
	pub fn invalidate(&self) -> Promise {
		match &self.kind {
			Kind::None => Promise::reject(&JsValue::from(Error::MustInstantiate)),
			Kind::Local(_) => Promise::reject(&JsValue::NULL),
			Kind::Remote(_) => todo!(),
		}
	}

	#[wasm_bindgen]
	pub fn authenticate(&self) -> Promise {
		match &self.kind {
			Kind::None => Promise::reject(&JsValue::from(Error::MustInstantiate)),
			Kind::Local(_) => Promise::reject(&JsValue::NULL),
			Kind::Remote(_) => todo!(),
		}
	}

	#[wasm_bindgen]
	pub fn live(&self) -> Promise {
		match &self.kind {
			Kind::None => Promise::reject(&JsValue::from(Error::MustInstantiate)),
			Kind::Local(_) => todo!(),
			Kind::Remote(_) => todo!(),
		}
	}

	#[wasm_bindgen]
	pub fn kill(&self) -> Promise {
		match &self.kind {
			Kind::None => Promise::reject(&JsValue::from(Error::MustInstantiate)),
			Kind::Local(_) => todo!(),
			Kind::Remote(_) => todo!(),
		}
	}

	#[wasm_bindgen]
	pub fn select(&self, table: Option<String>, thing: Option<String>) -> Promise {
		match &self.kind {
			Kind::None => Promise::reject(&JsValue::from(Error::MustInstantiate)),
			Kind::Local(v) => future_to_promise(Local::select(v.clone(), table, thing)),
			Kind::Remote(_) => todo!(),
		}
	}

	#[wasm_bindgen]
	pub fn create(&self, table: Option<String>, thing: Option<String>, data: JsValue) -> Promise {
		match &self.kind {
			Kind::None => Promise::reject(&JsValue::from(Error::MustInstantiate)),
			Kind::Local(v) => future_to_promise(Local::create(v.clone(), table, thing, data)),
			Kind::Remote(_) => todo!(),
		}
	}

	#[wasm_bindgen]
	pub fn update(&self, table: Option<String>, thing: Option<String>, data: JsValue) -> Promise {
		match &self.kind {
			Kind::None => Promise::reject(&JsValue::from(Error::MustInstantiate)),
			Kind::Local(v) => future_to_promise(Local::update(v.clone(), table, thing, data)),
			Kind::Remote(_) => todo!(),
		}
	}

	#[wasm_bindgen]
	pub fn modify(&self, table: Option<String>, thing: Option<String>, data: JsValue) -> Promise {
		match &self.kind {
			Kind::None => Promise::reject(&JsValue::from(Error::MustInstantiate)),
			Kind::Local(v) => future_to_promise(Local::modify(v.clone(), table, thing, data)),
			Kind::Remote(_) => todo!(),
		}
	}

	#[wasm_bindgen]
	pub fn delete(&self, table: Option<String>, thing: Option<String>) -> Promise {
		match &self.kind {
			Kind::None => Promise::reject(&JsValue::from(Error::MustInstantiate)),
			Kind::Local(v) => future_to_promise(Local::delete(v.clone(), table, thing)),
			Kind::Remote(_) => todo!(),
		}
	}
}
