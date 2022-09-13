use chrono::{DateTime, Utc};
use js_sys::Array;
use js_sys::Date;
use js_sys::Map;
use js_sys::Object;
use js_sys::Set;
use std::collections::BTreeMap;
use surrealdb::sql::Value;
use wasm_bindgen::{JsCast, JsValue};

pub trait Convert<T> {
	fn convert(self) -> T;
}

impl Convert<Value> for JsValue {
	fn convert(self) -> Value {
		if self.is_null() {
			Value::Null
		} else if self.is_undefined() {
			Value::None
		} else if let Some(v) = self.as_bool() {
			Value::from(v)
		} else if let Some(v) = self.as_string() {
			Value::from(v)
		} else if let Some(v) = self.as_f64() {
			Value::from(v)
		} else if self.has_type::<Date>() {
			let v: Date = self.unchecked_into();
			v.convert()
		} else if self.has_type::<Set>() {
			let v: Set = self.unchecked_into();
			v.convert()
		} else if self.has_type::<Map>() {
			let v: Map = self.unchecked_into();
			v.convert()
		} else if self.has_type::<Array>() {
			let v: Array = self.unchecked_into();
			v.convert()
		} else if self.has_type::<Object>() {
			let v: Object = self.unchecked_into();
			v.convert()
		} else {
			Value::None
		}
	}
}

impl Convert<Value> for Date {
	fn convert(self) -> Value {
		let val: DateTime<Utc> = DateTime::<Utc>::from(self);
		Value::from(val)
	}
}

impl Convert<Value> for Set {
	fn convert(self) -> Value {
		let val: Vec<Value> = self.convert();
		Value::from(val)
	}
}

impl Convert<Value> for Array {
	fn convert(self) -> Value {
		let val: Vec<Value> = self.convert();
		Value::from(val)
	}
}

impl Convert<Value> for Map {
	fn convert(self) -> Value {
		let val: BTreeMap<String, Value> = self.convert();
		Value::from(val)
	}
}

impl Convert<Value> for Object {
	fn convert(self) -> Value {
		let val: BTreeMap<String, Value> = self.convert();
		Value::from(val)
	}
}

impl Convert<String> for JsValue {
	fn convert(self) -> String {
		match self.as_string() {
			Some(v) => v,
			None => String::default(),
		}
	}
}

impl Convert<Vec<Value>> for Set {
	fn convert(self) -> Vec<Value> {
		let mut val: Vec<Value> = vec![];
		if self.has_type::<Set>() {
			for i in self.entries() {
				if let Ok(i) = i {
					let v: Value = i.convert();
					val.push(v);
				}
			}
		}
		val
	}
}

impl Convert<Vec<Value>> for Array {
	fn convert(self) -> Vec<Value> {
		let mut val: Vec<Value> = vec![];
		if self.has_type::<Array>() {
			for i in Array::values(&self).into_iter() {
				if let Ok(i) = i {
					let v: Value = i.convert();
					val.push(v);
				}
			}
		}
		val
	}
}

impl Convert<BTreeMap<String, Value>> for Map {
	fn convert(self) -> BTreeMap<String, Value> {
		let mut val: BTreeMap<String, Value> = BTreeMap::new();
		if self.has_type::<Map>() {
			for i in self.entries() {
				if let Ok(i) = i {
					let i: Array = i.unchecked_into();
					let k: String = i.get(0).as_string().unwrap();
					let v: Value = i.get(1).convert();
					val.insert(k, v);
				}
			}
		}
		val
	}
}

impl Convert<BTreeMap<String, Value>> for Object {
	fn convert(self) -> BTreeMap<String, Value> {
		let mut val: BTreeMap<String, Value> = BTreeMap::new();
		if self.has_type::<Object>() {
			for i in Object::entries(&self).iter() {
				let i: Array = i.unchecked_into();
				let k: String = i.get(0).as_string().unwrap();
				let v: Value = i.get(1).convert();
				val.insert(k, v);
			}
		}
		val
	}
}

impl Convert<BTreeMap<String, Value>> for JsValue {
	fn convert(self) -> BTreeMap<String, Value> {
		if self.has_type::<Map>() {
			let val: Map = self.unchecked_into();
			let val: BTreeMap<String, Value> = val.convert();
			return val;
		}
		if self.has_type::<Object>() {
			let val: Object = self.unchecked_into();
			let val: BTreeMap<String, Value> = val.convert();
			return val;
		}
		BTreeMap::new()
	}
}
