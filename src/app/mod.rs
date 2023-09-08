mod log;
mod opt;

use opt::auth::Credentials;
use opt::patch::Patch;
use serde::Serialize;
use serde_json::Value as Json;
use serde_wasm_bindgen::from_value;
use serde_wasm_bindgen::Serializer;
use std::collections::VecDeque;
use surrealdb::engine::any::Any;
use surrealdb::opt::auth::Database;
use surrealdb::opt::auth::Namespace;
use surrealdb::opt::auth::Root;
use surrealdb::opt::auth::Scope;
use surrealdb::opt::Config;
use surrealdb::opt::PatchOp;
use surrealdb::opt::Resource;
use surrealdb::sql::Range;
use surrealdb::sql::Value;
use wasm_bindgen::prelude::*;

pub use crate::err::Error;

// Converts a Rust value into a [`JsValue`].
fn to_value<T: Serialize + ?Sized>(value: &T) -> Result<JsValue, serde_wasm_bindgen::Error> {
	value.serialize(&Serializer::json_compatible())
}

#[wasm_bindgen(start)]
pub fn setup() {
	self::log::init();
}

#[wasm_bindgen]
pub struct Surreal {
	db: surrealdb::Surreal<Any>,
}

#[wasm_bindgen]
impl Surreal {
	/// Construct the database engine
	///
	/// ```js
	/// const db = new Surreal();
	/// ```
	#[wasm_bindgen(constructor)]
	pub fn init() -> Surreal {
		Surreal {
			db: surrealdb::Surreal::init(),
		}
	}

	/// Connect to a database engine
	///
	/// ```js
	/// const db = new Surreal();
	///
	/// // Connect to a WebSocket engine
	/// await db.connect('ws://localhost:8000');
	///
	/// // Connect to an HTTP engine
	/// await db.connect('http://localhost:8000');
	///
	/// // Connect to a memory engine
	/// await db.connect('mem://');
	///
	/// // Connect to an IndxDB engine
	/// await db.connect('indxdb://MyDatabase');
	///
	/// // Limit number of concurrent connections
	/// await db.connect('indxdb://MyDatabase', { capacity: 100000 });
	///
	/// // Enable strict mode on a local engine
	/// await db.connect('indxdb://MyDatabase', { strict: true });
	///
	/// // Enable notifications
	/// await db.connect('indxdb://MyDatabase', { notifications: true });
	///
	/// // Set query timeout time in seconds
	/// await db.connect('indxdb://MyDatabase', { query_timeout: 60 });
	///
	/// // Set transaction timeout time in seconds
	/// await db.connect('indxdb://MyDatabase', { transaction_timeout: 60 });
	///
	/// // Set changefeeds tick interval in seconds
	/// await db.connect('indxdb://MyDatabase', { tick_interval: 60 });
	///
	/// // Configure a system user
	/// await db.connect('indxdb://MyDatabase', { user: { username: "root", password: "root" } });
	///
	/// // Enable all capabilities
	/// await db.connect('indxdb://MyDatabase', { capabilities: true });
	///
	/// // Disable all capabilities
	/// await db.connect('indxdb://MyDatabase', { capabilities: false });
	///
	/// // Allow guest access
	/// await db.connect('indxdb://MyDatabase', { capabilities: { guest_access: true } });
	///
	/// // Allow all SurrealQL functions
	/// await db.connect('indxdb://MyDatabase', { capabilities: { functions: true } });
	///
	/// // Disallow all SurrealQL functions
	/// await db.connect('indxdb://MyDatabase', { capabilities: { functions: false } });
	///
	/// // Allow only certain SurrealQL functions
	/// await db.connect('indxdb://MyDatabase', { capabilities: { functions: ["fn", "string", "array::join"] } });
	///
	/// // Allow and disallow certain SurrealQL functions
	/// await db.connect('indxdb://MyDatabase', {
	///     capabilities: {
	///         functions: {
	///             allow: ["fn", "string", "array::join"], // You can also use `true` or `false` here to allow all or allow none
	///             deny: ["array"],                        // You can also use `true` or `false` here to deny all or deny none
	///         },
	///     },
	/// });
	///
	/// // Allow all network targets
	/// await db.connect('indxdb://MyDatabase', { capabilities: { network_targets: true } });
	///
	/// // Disallow all network targets
	/// await db.connect('indxdb://MyDatabase', { capabilities: { network_targets: false } });
	///
	/// // Allow only certain network targets
	/// await db.connect('indxdb://MyDatabase', { capabilities: { network_targets: ["http"] } });
	///
	/// // Allow and disallow certain network targets
	/// await db.connect('indxdb://MyDatabase', {
	///     capabilities: {
	///         network_targets: {
	///             allow: ["http"],                      // You can also use `true` or `false` here to allow all or allow none
	///             deny: ["ssh"],                        // You can also use `true` or `false` here to deny all or deny none
	///         },
	///     },
	/// });
	/// ```
	pub async fn connect(&self, endpoint: String, opts: JsValue) -> Result<(), Error> {
		let connect = match from_value::<Option<opt::endpoint::Options>>(opts)? {
			Some(opts) => {
				let capacity = opts.capacity;
				let config = Config::try_from(opts)?;
				let connect = self.db.connect((endpoint, config));
				match capacity {
					Some(capacity) => connect.with_capacity(capacity),
					None => connect,
				}
			}
			None => self.db.connect(endpoint),
		};
		connect.await.map_err(Into::into)
	}

	/// Switch to a specific namespace or database
	///
	/// ```js
	/// const db = new Surreal();
	///
	/// // Switch to a namespace
	/// await db.use({ ns: 'namespace' });
	///
	/// // Switch to a database
	/// await db.use({ db: 'database' });
	///
	/// // Switch both
	/// await db.use({ ns: 'namespace', db: 'database' });
	/// ```
	#[wasm_bindgen(js_name = use)]
	pub async fn yuse(&self, value: JsValue) -> Result<(), Error> {
		let opts: opt::yuse::Use = from_value(value)?;
		match (opts.ns, opts.db) {
			(Some(ns), Some(db)) => self.db.use_ns(ns).use_db(db).await.map_err(Into::into),
			(Some(ns), None) => self.db.use_ns(ns).await.map_err(Into::into),
			(None, Some(db)) => self.db.use_db(db).await.map_err(Into::into),
			(None, None) => Err("Select either namespace or database to use".into()),
		}
	}

	/// Assign a value as a parameter for this connection
	///
	/// ```js
	/// await db.set('name', { first: 'Tobie', last: 'Morgan Hitchcock' });
	/// ```
	pub async fn set(&self, key: String, value: JsValue) -> Result<(), Error> {
		let value: Json = from_value(value)?;
		self.db.set(key, value).await?;
		Ok(())
	}

	/// Remove a parameter from this connection
	///
	/// ```js
	/// await db.unset('name');
	/// ```
	pub async fn unset(&self, key: String) -> Result<(), Error> {
		self.db.unset(key).await?;
		Ok(())
	}

	/// Sign up a user to a specific authentication scope
	///
	/// ```js
	/// const token = await db.signup({
	///     namespace: 'namespace',
	///     database: 'database',
	///     scope: 'user_scope',
	///     email: 'john.doe@example.com',
	///     password: 'password123'
	/// });
	/// ```
	pub async fn signup(&self, credentials: JsValue) -> Result<JsValue, Error> {
		match from_value::<Credentials>(credentials)? {
			Credentials::Scope {
				namespace,
				database,
				scope,
				params,
			} => {
				let response = self
					.db
					.signup(Scope {
						params,
						namespace: &namespace,
						database: &database,
						scope: &scope,
					})
					.await?;
				Ok(to_value(&response)?)
			}
			Credentials::Database {
				..
			} => Err("Database users cannot signup, only scope users can".into()),
			Credentials::Namespace {
				..
			} => Err("Namespace users cannot signup, only scope users can".into()),
			Credentials::Root {
				..
			} => Err("Root users cannot signup, only scope users can".into()),
		}
	}

	/// Sign this connection in to a specific authentication scope
	///
	/// ```js
	/// const token = await db.signin({
	///     namespace: 'namespace',
	///     database: 'database',
	///     scope: 'user_scope',
	///     email: 'john.doe@example.com',
	///     password: 'password123'
	/// });
	/// ```
	pub async fn signin(&self, credentials: JsValue) -> Result<JsValue, Error> {
		let token = match &from_value::<Credentials>(credentials)? {
			Credentials::Scope {
				namespace,
				database,
				scope,
				params,
			} => self.db.signin(Scope {
				namespace,
				database,
				scope,
				params,
			}),
			Credentials::Database {
				namespace,
				database,
				username,
				password,
			} => self.db.signin(Database {
				namespace,
				database,
				username,
				password,
			}),
			Credentials::Namespace {
				namespace,
				username,
				password,
			} => self.db.signin(Namespace {
				namespace,
				username,
				password,
			}),
			Credentials::Root {
				username,
				password,
			} => self.db.signin(Root {
				username,
				password,
			}),
		}
		.await?;
		Ok(to_value(&token)?)
	}

	/// Invalidates the authentication for the current connection
	///
	/// ```js
	/// await db.invalidate();
	/// ```
	pub async fn invalidate(&self) -> Result<(), Error> {
		self.db.invalidate().await?;
		Ok(())
	}

	/// Authenticates the current connection with a JWT token
	///
	/// ```js
	/// await db.authenticate('<secret token>');
	/// ```
	pub async fn authenticate(&self, token: String) -> Result<(), Error> {
		self.db.authenticate(token).await?;
		Ok(())
	}

	/// Run a SurrealQL query against the database
	///
	/// ```js
	/// // Run a query without bindings
	/// const people = await db.query('SELECT * FROM person');
	///
	/// // Run a query with bindings
	/// const people = await db.query('SELECT * FROM type::table($table)', { table: 'person' });
	/// ```
	pub async fn query(&self, sql: String, bindings: JsValue) -> Result<JsValue, Error> {
		let bindings: Json = from_value(bindings)?;
		let mut response = match bindings.is_null() {
			true => self.db.query(sql).await?,
			false => self.db.query(sql).bind(bindings).await?,
		};
		let num_statements = response.num_statements();
		let response = if num_statements > 1 {
			let mut output = Vec::<Value>::with_capacity(num_statements);
			for index in 0..num_statements {
				output.push(response.take(index)?);
			}
			Value::from(output)
		} else {
			response.take(0)?
		};
		Ok(to_value(&response.into_json())?)
	}

	/// Select all records in a table, or a specific record
	///
	/// ```js
	/// // Select all records from a table
	/// const people = await db.select('person');
	///
	/// // Select a range records from a table
	/// const people = await db.select('person:jane..john');
	///
	/// // Select a specific record from a table
	/// const person = await db.select('person:h5wxrf2ewk8xjxosxtyc');
	/// ```
	pub async fn select(&self, resource: String) -> Result<JsValue, Error> {
		let response = match resource.parse::<Range>() {
			Ok(range) => {
				self.db.select(Resource::from(range.tb)).range((range.beg, range.end)).await?
			}
			Err(_) => self.db.select(Resource::from(resource)).await?,
		};
		Ok(to_value(&response.into_json())?)
	}

	/// Create a record in the database
	///
	/// ```js
	/// // Create a record with no fields set
	/// const person = await db.create('person');
	///
	/// Create a record with fields set
	/// const person = await db.create('person', {
	///     name: 'Tobie',
	///     settings: {
	///         active: true,
	///         marketing: true
	///     }
	/// });
	/// ```
	pub async fn create(&self, resource: String, data: JsValue) -> Result<JsValue, Error> {
		let resource = Resource::from(resource);
		let response = match from_value::<Option<Json>>(data)? {
			Some(data) => self.db.create(resource).content(data).await?,
			None => self.db.create(resource).await?,
		};
		Ok(to_value(&response.into_json())?)
	}

	/// Update all records in a table, or a specific record
	///
	/// ```js
	/// // Replace all records in a table with the specified data.
	/// const people = await db.update('person', {
	///     name: 'Tobie',
	///     settings: {
	///         active: true,
	///         marketing: true
	///     }
	/// });
	///
	/// // Replace a range of records with the specified data.
	/// const person = await db.update('person:jane..john', {
	///     name: 'Tobie',
	///     settings: {
	///         active: true,
	///         marketing: true
	///     }
	/// });
	///
	/// // Replace the current document / record data with the specified data.
	/// const person = await db.update('person:tobie', {
	///     name: 'Tobie',
	///     settings: {
	///         active: true,
	///         marketing: true
	///     }
	/// });
	/// ```
	pub async fn update(&self, resource: String, data: JsValue) -> Result<JsValue, Error> {
		let update = match resource.parse::<Range>() {
			Ok(range) => self.db.update(Resource::from(range.tb)).range((range.beg, range.end)),
			Err(_) => self.db.update(Resource::from(resource)),
		};
		let response = match from_value::<Option<Json>>(data)? {
			Some(data) => update.content(data).await?,
			None => update.await?,
		};
		Ok(to_value(&response.into_json())?)
	}

	/// Merge records in a table with specified data
	///
	/// ```js
	/// // Merge all records in a table with specified data.
	/// const person = await db.merge('person', {
	///     marketing: true
	/// });
	///
	/// // Merge a range of records with the specified data.
	/// const person = await db.merge('person:jane..john', {
	///     marketing: true
	/// });
	///
	/// // Merge the current document / record data with the specified data.
	/// const person = await db.merge('person:tobie', {
	///     marketing: true
	/// });
	/// ```
	pub async fn merge(&self, resource: String, data: JsValue) -> Result<JsValue, Error> {
		let update = match resource.parse::<Range>() {
			Ok(range) => self.db.update(Resource::from(range.tb)).range((range.beg, range.end)),
			Err(_) => self.db.update(Resource::from(resource)),
		};
		let data: Json = from_value(data)?;
		let response = update.merge(data).await?;
		Ok(to_value(&response.into_json())?)
	}

	/// Patch all records in a table or a specific record
	///
	/// ```js
	/// // Apply JSON Patch changes to all records in the database.
	/// const person = await db.patch('person', [{
	///     op: 'replace',
	///     path: '/settings/active',
	///     value: false
	/// }]);
	///
	/// // Apply JSON Patch to a range of records.
	/// const person = await db.patch('person:jane..john', [{
	///     op: 'replace',
	///     path: '/settings/active',
	///     value: false
	/// }]);
	///
	/// // Apply JSON Patch to a specific record.
	/// const person = await db.patch('person:tobie', [{
	///     op: 'replace',
	///     path: '/settings/active',
	///     value: false
	/// }]);
	/// ```
	pub async fn patch(&self, resource: String, data: JsValue) -> Result<JsValue, Error> {
		// Prepare the update request
		let update = match resource.parse::<Range>() {
			Ok(range) => self.db.update(Resource::from(range.tb)).range((range.beg, range.end)),
			Err(_) => self.db.update(Resource::from(resource)),
		};
		let mut patches: VecDeque<Patch> = from_value(data)?;
		// Extract the first patch
		let mut patch = match patches.pop_front() {
			// Setup the correct update type using the first patch
			Some(p) => update.patch(match p {
				Patch::Add {
					path,
					value,
				} => PatchOp::add(&path, value),
				Patch::Remove {
					path,
				} => PatchOp::remove(&path),
				Patch::Replace {
					path,
					value,
				} => PatchOp::replace(&path, value),
				Patch::Change {
					path,
					diff,
				} => PatchOp::change(&path, diff),
			}),
			None => {
				return Ok(to_value(&update.await?.into_json())?);
			}
		};
		// Loop through the rest of the patches and append them
		for p in patches {
			patch = patch.patch(match p {
				Patch::Add {
					path,
					value,
				} => PatchOp::add(&path, value),
				Patch::Remove {
					path,
				} => PatchOp::remove(&path),
				Patch::Replace {
					path,
					value,
				} => PatchOp::replace(&path, value),
				Patch::Change {
					path,
					diff,
				} => PatchOp::change(&path, diff),
			});
		}
		// Execute the update statement
		let response = patch.await?;
		Ok(to_value(&response.into_json())?)
	}

	/// Delete all records, or a specific record
	///
	/// ```js
	/// // Delete all records from a table
	/// const records = await db.delete('person');
	///
	/// // Delete a range records from a table
	/// const people = await db.delete('person:jane..john');
	///
	/// // Delete a specific record from a table
	/// const record = await db.delete('person:h5wxrf2ewk8xjxosxtyc');
	/// ```
	pub async fn delete(&self, resource: String) -> Result<JsValue, Error> {
		let response = match resource.parse::<Range>() {
			Ok(range) => {
				self.db.delete(Resource::from(range.tb)).range((range.beg, range.end)).await?
			}
			Err(_) => self.db.delete(Resource::from(resource)).await?,
		};
		Ok(to_value(&response.into_json())?)
	}

	/// Return the version of the server
	///
	/// ```js
	/// const version = await db.version();
	/// ```
	pub async fn version(&self) -> Result<JsValue, Error> {
		let response = self.db.version().await?;
		Ok(to_value(&response)?)
	}

	/// Check whether the server is healthy or not
	///
	/// ```js
	/// await db.health();
	/// ```
	pub async fn health(&self) -> Result<(), Error> {
		self.db.health().await?;
		Ok(())
	}
}
