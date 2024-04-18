use std::collections::BTreeMap;

use crate::app::opt::to_value::to_value;
use futures::StreamExt;
use serde_json::json;
use surrealdb::dbs::{Action, Session};
use surrealdb::kvs::Datastore;
use surrealdb::opt::Resource;
use surrealdb::rpc::format::cbor;
use surrealdb::rpc::method::Method;
use surrealdb::rpc::{Data, RpcContext};
use surrealdb::sql::{Range, Uuid, Value};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_streams::readable::sys;
use wasm_streams::ReadableStream;
use web_sys::js_sys::Uint8Array;

pub use crate::err::Error;

#[wasm_bindgen]
pub struct WasmRpc(WasmRpcInner);

#[wasm_bindgen]
impl WasmRpc {
	pub async fn execute(&mut self, data: Uint8Array) -> Result<Uint8Array, Error> {
		let in_data = cbor::req(data.to_vec()).map_err(|e| e.to_string())?;
		let res = self
			.0
			.execute(Method::parse(in_data.method), in_data.params)
			.await
			.map_err(|e| e.to_string())?;
		let out = cbor::res(res).map_err(|e| e.to_string())?;
		Ok(out.as_slice().into())
	}

	pub async fn memory() -> Result<WasmRpc, Error> {
		let kvs = Datastore::new("memory").await?.with_notifications();

		let inner = WasmRpcInner {
			kvs,
			session: Default::default(),
			vars: Default::default(),
		};

		Ok(WasmRpc(inner))
	}

	pub async fn indxdb(name: String) -> Result<WasmRpc, Error> {
		let kvs = Datastore::new(&format!("indxdb://{name}")).await?.with_notifications();

		let inner = WasmRpcInner {
			kvs,
			session: Default::default(),
			vars: Default::default(),
		};

		Ok(WasmRpc(inner))
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
}

struct WasmRpcInner {
	pub kvs: Datastore,
	pub session: Session,
	pub vars: BTreeMap<String, Value>,
}

impl RpcContext for WasmRpcInner {
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
}
