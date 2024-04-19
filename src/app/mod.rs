use std::collections::BTreeMap;
use std::time::Duration;

mod types;
mod opt;

use opt::to_value::to_value;
use serde_json::json;
use serde_wasm_bindgen::from_value;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::rpc::format::cbor;
use surrealdb::rpc::method::Method;
use surrealdb::rpc::{Data, RpcContext};
use surrealdb::sql::Value;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use wasm_streams::readable::sys;
use wasm_streams::ReadableStream;
use web_sys::js_sys::Uint8Array;
use types::TsConnectionOptions;
use opt::endpoint::Options;
use futures::StreamExt;

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

		let response = stream.map(|notification| {
			let json = json!({
				"id": notification.id,
				"action": notification.action.to_string(),
				"result": notification.result.into_json(),
			});
			to_value(&json).map_err(Into::into)
		});
		Ok(ReadableStream::from_stream(response).into_raw())
	}

	pub async fn connect(endpoint: String, opts: Option<TsConnectionOptions>) -> Result<SurrealWasmEngine, Error> {
		let endpoint = match &endpoint {
			s if s.starts_with("mem:") => "memory",
			s => s
		};
		let kvs = Datastore::new(endpoint).await?.with_notifications();
		let kvs = match from_value::<Option<Options>>(JsValue::from(opts))? {
			None => kvs,
			Some(opts) => kvs
				.with_capabilities(opts.capabilities.map_or(Ok(Default::default()), |a| a.try_into())?)
				.with_transaction_timeout(opts.transaction_timeout.map(|qt| Duration::from_secs(qt as u64)))
				.with_query_timeout(opts.query_timeout.map(|qt| Duration::from_secs(qt as u64)))
				.with_strict_mode(opts.strict.map_or(Default::default(), |s| s))
		};

		let inner = SurrealWasmEngineInner {
			kvs,
			session: Session {
				// rt: true,
				..Default::default()
			},
			vars: Default::default(),
		};

		Ok(SurrealWasmEngine(inner))
	}
}

struct SurrealWasmEngineInner {
	pub kvs: Datastore,
	pub session: Session,
	pub vars: BTreeMap<String, Value>,
}

impl RpcContext for SurrealWasmEngineInner {
	// const LQ_SUPPORT: bool = true;

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
		let val = "todo".to_string();

		val
	}

	// async fn handle_live(&self, lqid: &Uuid) {
	// 	LIVE_QUERIES.write().await.insert(*lqid, self.id);
	// 	trace!("Registered live query {} on websocket {}", lqid, self.id);
	// }

	// async fn handle_kill(&self, lqid: &Uuid) {
	// 	if let Some(id) = LIVE_QUERIES.write().await.remove(lqid) {
	// 		trace!("Unregistered live query {} on websocket {}", lqid, id);
	// 	}
	// }
}
