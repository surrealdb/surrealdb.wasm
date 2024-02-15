mod log;
mod opt;
mod types;

use dmp::Diff;
use futures::StreamExt;
use opt::auth::Credentials;
use opt::patch::Patch;
use opt::to_value::to_value;
use opt::AsStr as _;
use serde_json::json;
use serde_json::Value as Json;
use serde_wasm_bindgen::from_value;
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
use surrealdb::sql::{json, Array, Value};
use types::*;
use wasm_bindgen::prelude::*;
use wasm_streams::readable::sys;
use wasm_streams::readable::ReadableStream;

pub use crate::err::Error;

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
	/// // Enable live query notifications
	/// await db.connect('indxdb://MyDatabase', { capabilities: { live_query_notifications: true } });
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
	pub async fn connect(
		&self,
		endpoint: String,
		opts: Option<TsConnectionOptions>,
	) -> Result<(), Error> {
		let opts = JsValue::from(opts);
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
	/// await db.use({ namespace: 'namespace' });
	///
	/// // Switch to a database
	/// await db.use({ database: 'database' });
	///
	/// // Switch both
	/// await db.use({ namespace: 'namespace', database: 'database' });
	/// ```
	#[wasm_bindgen(js_name = use)]
	pub async fn yuse(&self, opts: Option<TsUseOptions>) -> Result<(), Error> {
		let opts = JsValue::from(opts);
		let opts: opt::yuse::Use = from_value(opts)?;
		match (opts.namespace, opts.database) {
			(Some(namespace), Some(database)) => {
				self.db.use_ns(namespace).use_db(database).await.map_err(Into::into)
			}
			(Some(namespace), None) => self.db.use_ns(namespace).await.map_err(Into::into),
			(None, Some(database)) => self.db.use_db(database).await.map_err(Into::into),
			(None, None) => Err("Select either namespace or database to use".into()),
		}
	}

	/// Assign a value as a parameter for this connection
	///
	/// ```js
	/// await db.set('name', { first: 'Tobie', last: 'Morgan Hitchcock' });
	/// ```
	pub async fn set(&self, key: String, value: TsUnknown) -> Result<(), Error> {
		let value = JsValue::from(value);
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
	pub async fn signup(&self, credentials: TsScopeUserAuth) -> Result<String, Error> {
		let credentials = JsValue::from(credentials);
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
				Ok(response.into_insecure_token())
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
	pub async fn signin(&self, credentials: TsAnyAuth) -> Result<String, Error> {
		let credentials = JsValue::from(credentials);
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
		Ok(token.into_insecure_token())
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
	pub async fn authenticate(&self, token: String) -> Result<bool, Error> {
		self.db.authenticate(token).await?;
		Ok(true)
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
	pub async fn query(
		&self,
		sql: String,
		bindings: Option<TsRecordUnknown>,
	) -> Result<TsArrayUnknown, Error> {
		let bindings = JsValue::from(bindings);
		let mut response = match bindings.is_undefined() {
			true => self.db.query(sql).await?,
			false => {
				let bindings = json(&from_value::<Json>(bindings)?.to_string())?;
				self.db.query(sql).bind(bindings).await?
			}
		};
		let num_statements = response.num_statements();
		let response = {
			let mut output = Vec::<Value>::with_capacity(num_statements);
			for index in 0..num_statements {
				output.push(response.take(index)?);
			}
			Value::from(Array::from(output))
		};
		TsArrayUnknown::from_value(response)
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
	pub async fn select(&self, resource: String) -> Result<TsArrayRecordUnknown, Error> {
		let response = match resource.parse::<Range>() {
			Ok(range) => {
				self.db.select(Resource::from(range.tb)).range((range.beg, range.end)).await?
			}
			Err(_) => self.db.select(Resource::from(resource)).await?,
		};
		TsArrayRecordUnknown::from_value(response)
	}

	/// Live select all records in a table, or a specific record
	///
	/// ```js
	/// // Live select all records from a table
	/// const stream = await db.live('person');
	///
	/// // Live select a range records from a table
	/// const stream = await db.live('person:jane..john');
	///
	/// // Live select a specific record from a table
	/// const stream = await db.live('person:jane');
	///
	/// // Get a reader
	/// const reader = stream.getReader();
	///
	/// // Listen for changes
	/// while (true) {
	///   // Read from the stream
	///   const {done, notification} = await reader.read();
	///
	///   // Do something with each notification
	///   console.log(notification);
	///
	///   // Exit the loop if done
	///   if (done) break;
	/// }
	/// ```
	pub async fn live(&self, resource: String) -> Result<sys::ReadableStream, Error> {
		let stream = match resource.parse::<Range>() {
			Ok(range) => {
				self.db
					.select(Resource::from(range.tb))
					.range((range.beg, range.end))
					.live()
					.into_owned()
					.await?
			}
			Err(_) => self.db.select(Resource::from(resource)).live().into_owned().await?,
		};
		let response = stream.map(|notification| {
			let json = json!({
				"id": notification.query_id,
				"action": notification.action.as_str(),
				"result": notification.data.into_json(),
			});
			to_value(&json).map_err(Into::into)
		});
		Ok(ReadableStream::from_stream(response).into_raw())
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
	pub async fn create(
		&self,
		resource: String,
		data: Option<TsRecordUnknown>,
	) -> Result<TsArrayRecordUnknown, Error> {
		let data = JsValue::from(data);
		let resource = Resource::from(resource);
		let response = match data.is_undefined() {
			true => self.db.create(resource).await?,
			false => {
				let data = json(&from_value::<Json>(data)?.to_string())?;
				self.db.create(resource).content(data).await?
			}
		};
		TsArrayRecordUnknown::from_value(response)
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
	pub async fn update(
		&self,
		resource: String,
		data: Option<TsRecordUnknown>,
	) -> Result<TsArrayRecordUnknown, Error> {
		let data = JsValue::from(data);
		let update = match resource.parse::<Range>() {
			Ok(range) => self.db.update(Resource::from(range.tb)).range((range.beg, range.end)),
			Err(_) => self.db.update(Resource::from(resource)),
		};
		let response = match data.is_undefined() {
			true => update.await?,
			false => {
				let data = json(&from_value::<Json>(data)?.to_string())?;
				update.content(data).await?
			}
		};
		TsArrayRecordUnknown::from_value(response)
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
	pub async fn merge(
		&self,
		resource: String,
		data: TsRecordUnknown,
	) -> Result<TsArrayRecordUnknown, Error> {
		let data = JsValue::from(data);
		let update = match resource.parse::<Range>() {
			Ok(range) => self.db.update(Resource::from(range.tb)).range((range.beg, range.end)),
			Err(_) => self.db.update(Resource::from(resource)),
		};
		let data = json(&from_value::<Json>(data)?.to_string())?;
		let response = update.merge(data).await?;
		TsArrayRecordUnknown::from_value(response)
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
	pub async fn patch(
		&self,
		resource: String,
		data: TsArrayPatch,
	) -> Result<TsArrayRecordUnknown, Error> {
		let data = JsValue::from(data);
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
					value,
				} => PatchOp::change(
					&path,
					Diff {
						operation: 0,
						text: value,
					},
				),
			}),
			None => {
				return TsArrayRecordUnknown::from_value(update.await?);
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
					value,
				} => PatchOp::change(
					&path,
					Diff {
						operation: 0,
						text: value,
					},
				),
			});
		}
		// Execute the update statement
		let response = patch.await?;
		TsArrayRecordUnknown::from_value(response)
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
	pub async fn delete(&self, resource: String) -> Result<TsArrayRecordUnknown, Error> {
		let response = match resource.parse::<Range>() {
			Ok(range) => {
				self.db.delete(Resource::from(range.tb)).range((range.beg, range.end)).await?
			}
			Err(_) => self.db.delete(Resource::from(resource)).await?,
		};
		TsArrayRecordUnknown::from_value(response)
	}

	/// Return the version of the server
	///
	/// ```js
	/// const version = await db.version();
	/// ```
	pub async fn version(&self) -> Result<String, Error> {
		let response = self.db.version().await?;
		Ok(response.to_string())
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
