use crate::app::convert::Convert;
use crate::err::Error;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::RwLock;
use surrealdb::dbs::Auth;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::sql::Value;
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

	pub async fn select(this: Rc<RwLock<Self>>, what: JsValue) -> Result<JsValue, JsValue> {
		// Fetch the datastore instance
		let kvs = Local::init(&this).await?;
		// Convert the input to SurrealQL
		let what: Value = what.convert();
		// Clone the current session data
		let sess = this.read().unwrap().session.clone();
		// Specify the SQL query string
		let text = format!("SELECT * FROM $what");
		// Specify the query parameters
		let vars = Some(map! {
			String::from("what") => what.could_be_table(),
		});
		// Execute the query on the database
		match kvs.execute(&text, &sess, vars, false).await {
			Ok(mut v) => match v.swap_remove(1).output() {
				Ok(v) => Ok(JsValue::from_serde(&v.first()).unwrap()),
				Err(e) => Err(JsError::new(e.to_string().as_str()).into()),
			},
			Err(e) => Err(JsError::from(e).into()),
		}
	}

	pub async fn create(
		this: Rc<RwLock<Self>>,
		what: JsValue,
		data: JsValue,
	) -> Result<JsValue, JsValue> {
		// Fetch the datastore instance
		let kvs = Local::init(&this).await?;
		// Convert the input to SurrealQL
		let what: Value = what.convert();
		// Clone the current session data
		let sess = this.read().unwrap().session.clone();
		// Specify the SQL query string
		let text = format!("CREATE $what CONTENT $data");
		// Specify the query parameters
		let vars = Some(map! {
			String::from("what") => what.could_be_table(),
			String::from("data") => data.convert(),
		});
		// Execute the query on the database
		match kvs.execute(&text, &sess, vars, false).await {
			Ok(mut v) => match v.swap_remove(1).output() {
				Ok(v) => Ok(JsValue::from_serde(&v.first()).unwrap()),
				Err(e) => Err(JsError::new(e.to_string().as_str()).into()),
			},
			Err(e) => Err(JsError::from(e).into()),
		}
	}

	pub async fn update(
		this: Rc<RwLock<Self>>,
		what: JsValue,
		data: JsValue,
	) -> Result<JsValue, JsValue> {
		// Fetch the datastore instance
		let kvs = Local::init(&this).await?;
		// Convert the input to SurrealQL
		let what: Value = what.convert();
		// Clone the current session data
		let sess = this.read().unwrap().session.clone();
		// Specify the SQL query string
		let text = format!("UPDATE $what CONTENT $data");
		// Specify the query parameters
		let vars = Some(map! {
			String::from("what") => what.could_be_table(),
			String::from("data") => data.convert(),
		});
		// Execute the query on the database
		match kvs.execute(&text, &sess, vars, false).await {
			Ok(mut v) => match v.swap_remove(1).output() {
				Ok(v) => Ok(JsValue::from_serde(&v.first()).unwrap()),
				Err(e) => Err(JsError::new(e.to_string().as_str()).into()),
			},
			Err(e) => Err(JsError::from(e).into()),
		}
	}

	pub async fn merge(
		this: Rc<RwLock<Self>>,
		what: JsValue,
		data: JsValue,
	) -> Result<JsValue, JsValue> {
		// Fetch the datastore instance
		let kvs = Local::init(&this).await?;
		// Convert the input to SurrealQL
		let what: Value = what.convert();
		// Clone the current session data
		let sess = this.read().unwrap().session.clone();
		// Specify the SQL query string
		let text = format!("UPDATE $what MERGE $data");
		// Specify the query parameters
		let vars = Some(map! {
			String::from("what") => what.could_be_table(),
			String::from("data") => data.convert(),
		});
		// Execute the query on the database
		match kvs.execute(&text, &sess, vars, false).await {
			Ok(mut v) => match v.swap_remove(1).output() {
				Ok(v) => Ok(JsValue::from_serde(&v.first()).unwrap()),
				Err(e) => Err(JsError::new(e.to_string().as_str()).into()),
			},
			Err(e) => Err(JsError::from(e).into()),
		}
	}

	pub async fn patch(
		this: Rc<RwLock<Self>>,
		what: JsValue,
		data: JsValue,
	) -> Result<JsValue, JsValue> {
		// Fetch the datastore instance
		let kvs = Local::init(&this).await?;
		// Convert the input to SurrealQL
		let what: Value = what.convert();
		// Clone the current session data
		let sess = this.read().unwrap().session.clone();
		// Specify the SQL query string
		let text = format!("UPDATE $what PATCH $data");
		// Specify the query parameters
		let vars = Some(map! {
			String::from("what") => what.could_be_table(),
			String::from("data") => data.convert(),
		});
		// Execute the query on the database
		match kvs.execute(&text, &sess, vars, false).await {
			Ok(mut v) => match v.swap_remove(1).output() {
				Ok(v) => Ok(JsValue::from_serde(&v.first()).unwrap()),
				Err(e) => Err(JsError::new(e.to_string().as_str()).into()),
			},
			Err(e) => Err(JsError::from(e).into()),
		}
	}

	pub async fn delete(this: Rc<RwLock<Self>>, what: JsValue) -> Result<JsValue, JsValue> {
		// Fetch the datastore instance
		let kvs = Local::init(&this).await?;
		// Convert the input to SurrealQL
		let what: Value = what.convert();
		// Clone the current session data
		let sess = this.read().unwrap().session.clone();
		// Specify the SQL query string
		let text = format!("DELETE $what");
		// Specify the query parameters
		let vars = Some(map! {
			String::from("what") => what.could_be_table(),
		});
		// Execute the query on the database
		match kvs.execute(&text, &sess, vars, false).await {
			Ok(mut v) => match v.swap_remove(1).output() {
				Ok(v) => Ok(JsValue::from_serde(&v.first()).unwrap()),
				Err(e) => Err(JsError::new(e.to_string().as_str()).into()),
			},
			Err(e) => Err(JsError::from(e).into()),
		}
	}
}
