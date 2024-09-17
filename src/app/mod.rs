use std::collections::BTreeMap;
use std::time::Duration;

mod opt;
mod types;

use cbor::Cbor;
use futures::StreamExt;
use once_cell::sync::Lazy;
use opt::endpoint::Options;
use serde_wasm_bindgen::from_value;
use surrealdb::dbs::Notification;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::rpc::format::cbor;
use surrealdb::rpc::method::Method;
use surrealdb::rpc::{Data, RpcContext};
use surrealdb::sql::{Object, Value};
use types::TsConnectionOptions;
use uuid::Uuid;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use wasm_streams::readable::sys;
use wasm_streams::ReadableStream;
use web_sys::js_sys::Uint8Array;

pub use crate::err::Error;

#[wasm_bindgen]
pub struct SurrealWasmEngine(SurrealWasmEngineInner);

#[wasm_bindgen]
impl SurrealWasmEngine {
	pub async fn execute(&mut self, data: Uint8Array) -> Result<Uint8Array, Error> {
		let in_data = cbor::req(data.to_vec()).map_err(|e| e.to_string())?;
		let res = self
			.0
			.execute(Method::parse(in_data.method), in_data.params)
			.await
			.map_err(|e| e.to_string())?;
		println!("{:?}", res);
		let out = cbor::res(res).map_err(|e| e.to_string())?;
		Ok(out.as_slice().into())
	}

	pub fn notifications(&self) -> Result<sys::ReadableStream, Error> {
		let stream = self.0.kvs.notifications().ok_or("Notifications not enabled")?;

		fn process_notification(notification: Notification) -> Result<JsValue, JsValue> {
			// Construct live message
			let mut message = Object::default();
			message.insert("id".to_string(), notification.id.into());
			message.insert("action".to_string(), notification.action.to_string().into());
			message.insert("result".to_string(), notification.result);

			// Into CBOR value
			let cbor: Cbor = Value::Object(message)
				.try_into()
				.map_err(|_| JsValue::from_str("Failed to convert notification to CBOR"))?;
			let mut res = Vec::new();
			ciborium::into_writer(&cbor.0, &mut res).unwrap();
			let out_arr: Uint8Array = res.as_slice().into();
			Ok(out_arr.into())
		}

		let response = stream.map(process_notification);
		Ok(ReadableStream::from_stream(response).into_raw())
	}

	pub async fn connect(
		endpoint: String,
		opts: Option<TsConnectionOptions>,
	) -> Result<SurrealWasmEngine, Error> {
		let endpoint = match &endpoint {
			s if s.starts_with("mem:") => "memory",
			s => s,
		};
		let kvs = Datastore::new(endpoint).await?.with_notifications();
		let kvs = match from_value::<Option<Options>>(JsValue::from(opts))? {
			None => kvs,
			Some(opts) => kvs
				.with_capabilities(
					opts.capabilities.map_or(Ok(Default::default()), |a| a.try_into())?,
				)
				.with_transaction_timeout(
					opts.transaction_timeout.map(|qt| Duration::from_secs(qt as u64)),
				)
				.with_query_timeout(opts.query_timeout.map(|qt| Duration::from_secs(qt as u64)))
				.with_strict_mode(opts.strict.map_or(Default::default(), |s| s)),
		};

		let inner = SurrealWasmEngineInner {
			kvs,
			session: Session::default().with_rt(true),
			vars: Default::default(),
		};

		Ok(SurrealWasmEngine(inner))
	}

	pub fn version() -> Result<String, Error> {
		Ok(SURREALDB_VERSION.clone())
	}
}

struct SurrealWasmEngineInner {
	pub kvs: Datastore,
	pub session: Session,
	pub vars: BTreeMap<String, Value>,
}

impl RpcContext for SurrealWasmEngineInner {
	fn kvs(&self) -> &Datastore {
		&self.kvs
	}

	fn session(&self) -> &Session {
		&self.session
	}

	fn session_mut(&mut self) -> &mut Session {
		&mut self.session
	}

	fn vars(&self) -> &BTreeMap<String, Value> {
		&self.vars
	}

	fn vars_mut(&mut self) -> &mut BTreeMap<String, Value> {
		&mut self.vars
	}

	fn version_data(&self) -> impl Into<Data> {
		Value::Strand(format!("surrealdb-{}", *SURREALDB_VERSION).into())
	}

	const LQ_SUPPORT: bool = true;

	async fn handle_live(&self, _lqid: &Uuid) {}

	async fn handle_kill(&self, _lqid: &Uuid) {}
}

static LOCK_FILE: &str = include_str!("../../Cargo.lock");

pub static SURREALDB_VERSION: Lazy<String> = Lazy::new(|| {
	let lock: cargo_lock::Lockfile = LOCK_FILE.parse().expect("Failed to parse Cargo.lock");
	let package = lock
		.packages
		.iter()
		.find(|p| p.name.as_str() == "surrealdb")
		.expect("Failed to find surrealdb in Cargo.lock");

	format!("{}.{}.{}", package.version.major, package.version.minor, package.version.patch)
});
