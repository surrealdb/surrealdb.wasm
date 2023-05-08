use crate::app::convert::Convert;
use crate::err::Error;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::RwLock;
use surrealdb::sql::Value;
use surrealdb::Auth;
use surrealdb::Datastore;
use surrealdb::Session;
use wasm_bindgen::JsError;
use wasm_bindgen::JsValue;

pub struct Local {
	url: String,
	kvs: Option<Rc<Datastore>>,
	session: Session,
}

impl Local {
	pub fn new(url: String) -> Local {
		Local {
			url,
			kvs: None,
			session: Session::default(),
		}
	}

	async fn init(this: &Rc<RwLock<Self>>) -> Result<Rc<Datastore>, Error> {
		// Read unlock this instance
		let lk = this.read().unwrap();
		//
		match lk.kvs.as_ref() {
			None => {
				drop(lk);
				let mut lk = this.write().unwrap();
				let kvs = Datastore::new(&lk.url).await?;
				let kvs = Rc::new(kvs);
				lk.kvs = Some(kvs.clone());
				lk.session.au = Arc::new(Auth::Kv);
				Ok(kvs)
			}
			Some(v) => Ok(v.clone()),
		}
	}

	pub async fn sql(
		this: Rc<RwLock<Self>>,
		text: JsValue,
		vars: JsValue,
	) -> Result<JsValue, JsValue> {
		// Fetch the datastore instance
		let kvs = Local::init(&this).await?;
		// Clone the current session data
		let sess = this.read().unwrap().session.clone();
		// Perform the query
		let text: String = text.convert();
		let vars: BTreeMap<String, Value> = vars.convert();
		match kvs.execute(&text, &sess, Some(vars), false).await {
			Ok(v) => Ok(JsValue::from_serde(&v).unwrap()),
			Err(e) => Err(JsError::from(e).into()),
		}
	}

	pub async fn yuse(
		this: Rc<RwLock<Self>>,
		ns: Option<String>,
		db: Option<String>,
	) -> Result<JsValue, JsValue> {
		// Fetch a writeable instance
		let mut this = this.write().unwrap();
		// Set the NS if specified
		if let Some(ns) = ns {
			this.session.ns = Some(ns);
		}
		// Set the DB if specified
		if let Some(db) = db {
			this.session.db = Some(db);
		}
		// Return an empty result
		Ok(JsValue::NULL)
	}

	pub async fn select(
		this: Rc<RwLock<Self>>,
		table: Option<String>,
		thing: Option<String>,
	) -> Result<JsValue, JsValue> {
		// Fetch the datastore instance
		let kvs = Local::init(&this).await?;
		// Clone the current session data
		let sess = this.read().unwrap().session.clone();
		// Perform the query
		match table {
			Some(table) => match thing {
				Some(id) => {
					let text = format!("SELECT * FROM type::thing($tb, $id)");
					let vars = map! {
						String::from("tb") => Value::from(table),
						String::from("id") => Value::from(id),
					};
					match kvs.execute(&text, &sess, Some(vars), false).await {
						Ok(v) => match v.first().unwrap().output() {
							Ok(v) => Ok(JsValue::from_serde(v.single()).unwrap()),
							Err(e) => Err(JsError::new(e.to_string().as_str()).into()),
						},
						Err(e) => Err(JsError::from(e).into()),
					}
				}
				None => {
					let text = format!("SELECT * FROM type::table($tb)");
					let vars = map! {
						String::from("tb") => Value::from(table),
					};
					match kvs.execute(&text, &sess, Some(vars), false).await {
						Ok(v) => match v.first().unwrap().output() {
							Ok(v) => Ok(JsValue::from_serde(v).unwrap()),
							Err(e) => Err(JsError::new(e.to_string().as_str()).into()),
						},
						Err(e) => Err(JsError::from(e).into()),
					}
				}
			},
			None => Err(JsValue::from("Specify a table")),
		}
	}

	pub async fn create(
		this: Rc<RwLock<Self>>,
		table: Option<String>,
		thing: Option<String>,
		data: JsValue,
	) -> Result<JsValue, JsValue> {
		// Fetch the datastore instance
		let kvs = Local::init(&this).await?;
		// Clone the current session data
		let sess = this.read().unwrap().session.clone();
		// Perform the query
		match table {
			Some(table) => match thing {
				Some(id) => {
					let text = format!("CREATE type::thing($tb, $id) CONTENT $data");
					let vars = map! {
						String::from("tb") => Value::from(table),
						String::from("id") => Value::from(id),
						String::from("data") => data.convert(),
					};
					match kvs.execute(&text, &sess, Some(vars), false).await {
						Ok(v) => match v.first().unwrap().output() {
							Ok(v) => Ok(JsValue::from_serde(v.single()).unwrap()),
							Err(e) => Err(JsError::new(e.to_string().as_str()).into()),
						},
						Err(e) => Err(JsError::from(e).into()),
					}
				}
				None => {
					let text = format!("CREATE type::table($tb) CONTENT $data");
					let vars = map! {
						String::from("tb") => Value::from(table),
						String::from("data") => data.convert(),
					};
					match kvs.execute(&text, &sess, Some(vars), false).await {
						Ok(v) => match v.first().unwrap().output() {
							Ok(v) => Ok(JsValue::from_serde(v.single()).unwrap()),
							Err(e) => Err(JsError::new(e.to_string().as_str()).into()),
						},
						Err(e) => Err(JsError::from(e).into()),
					}
				}
			},
			None => Err(JsValue::from("Specify a table")),
		}
	}

	pub async fn update(
		this: Rc<RwLock<Self>>,
		table: Option<String>,
		thing: Option<String>,
		data: JsValue,
	) -> Result<JsValue, JsValue> {
		// Fetch the datastore instance
		let kvs = Local::init(&this).await?;
		// Clone the current session data
		let sess = this.read().unwrap().session.clone();
		// Perform the query
		match table {
			Some(table) => match thing {
				Some(id) => {
					let text = format!("UPDATE type::thing($tb, $id) CONTENT $data");
					let vars = map! {
						String::from("tb") => Value::from(table),
						String::from("id") => Value::from(id),
						String::from("data") => data.convert(),
					};
					match kvs.execute(&text, &sess, Some(vars), false).await {
						Ok(v) => match v.first().unwrap().output() {
							Ok(v) => Ok(JsValue::from_serde(v.single()).unwrap()),
							Err(e) => Err(JsError::new(e.to_string().as_str()).into()),
						},
						Err(e) => Err(JsError::from(e).into()),
					}
				}
				None => {
					let text = format!("UPDATE type::table($tb) CONTENT $data");
					let vars = map! {
						String::from("tb") => Value::from(table),
						String::from("data") => data.convert(),
					};
					match kvs.execute(&text, &sess, Some(vars), false).await {
						Ok(v) => match v.first().unwrap().output() {
							Ok(v) => Ok(JsValue::from_serde(v).unwrap()),
							Err(e) => Err(JsError::new(e.to_string().as_str()).into()),
						},
						Err(e) => Err(JsError::from(e).into()),
					}
				}
			},
			None => Err(JsValue::from("Specify a table")),
		}
	}

	pub async fn modify(
		this: Rc<RwLock<Self>>,
		table: Option<String>,
		thing: Option<String>,
		data: JsValue,
	) -> Result<JsValue, JsValue> {
		// Fetch the datastore instance
		let kvs = Local::init(&this).await?;
		// Clone the current session data
		let sess = this.read().unwrap().session.clone();
		// Perform the query
		match table {
			Some(table) => match thing {
				Some(id) => {
					let text = format!("UPDATE type::thing($tb, $id) MERGE $data");
					let vars = map! {
						String::from("tb") => Value::from(table),
						String::from("id") => Value::from(id),
						String::from("data") => data.convert(),
					};
					match kvs.execute(&text, &sess, Some(vars), false).await {
						Ok(v) => match v.first().unwrap().output() {
							Ok(v) => Ok(JsValue::from_serde(v.single()).unwrap()),
							Err(e) => Err(JsError::new(e.to_string().as_str()).into()),
						},
						Err(e) => Err(JsError::from(e).into()),
					}
				}
				None => {
					let text = format!("UPDATE type::table($tb) MERGE $data");
					let vars = map! {
						String::from("tb") => Value::from(table),
						String::from("data") => data.convert(),
					};
					match kvs.execute(&text, &sess, Some(vars), false).await {
						Ok(v) => match v.first().unwrap().output() {
							Ok(v) => Ok(JsValue::from_serde(v).unwrap()),
							Err(e) => Err(JsError::new(e.to_string().as_str()).into()),
						},
						Err(e) => Err(JsError::from(e).into()),
					}
				}
			},
			None => Err(JsValue::from("Specify a table")),
		}
	}

	pub async fn delete(
		this: Rc<RwLock<Self>>,
		table: Option<String>,
		thing: Option<String>,
	) -> Result<JsValue, JsValue> {
		// Fetch the datastore instance
		let kvs = Local::init(&this).await?;
		// Clone the current session data
		let sess = this.read().unwrap().session.clone();
		// Perform the query
		match table {
			Some(table) => match thing {
				Some(id) => {
					let text = format!("DELETE type::thing($tb, $id)");
					let vars = map! {
						String::from("tb") => Value::from(table),
						String::from("id") => Value::from(id),
					};
					match kvs.execute(&text, &sess, Some(vars), false).await {
						Ok(v) => match v.first().unwrap().output() {
							Ok(v) => Ok(JsValue::from_serde(v.single()).unwrap()),
							Err(e) => Err(JsError::new(e.to_string().as_str()).into()),
						},
						Err(e) => Err(JsError::from(e).into()),
					}
				}
				None => {
					let text = format!("DELETE type::table($tb)");
					let vars = map! {
						String::from("tb") => Value::from(table),
					};
					match kvs.execute(&text, &sess, Some(vars), false).await {
						Ok(v) => match v.first().unwrap().output() {
							Ok(v) => Ok(JsValue::from_serde(v).unwrap()),
							Err(e) => Err(JsError::new(e.to_string().as_str()).into()),
						},
						Err(e) => Err(JsError::from(e).into()),
					}
				}
			},
			None => Err(JsValue::from("Specify a table")),
		}
	}
}
