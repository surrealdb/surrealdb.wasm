use crate::app::opt::patch::Patch;
use serde::Deserialize;
use serde_json::Value;
use std::collections::VecDeque;

#[derive(Deserialize)]
pub struct Create {
	pub content: Value,
}

#[derive(Deserialize)]
pub struct Update {
	#[serde(flatten)]
	pub clause: Option<Clause>,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Clause {
	Content(Value),
	Merge(Value),
	Patch(VecDeque<Patch>),
}
