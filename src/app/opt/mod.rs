use surrealdb::Action;

pub mod auth;
pub mod endpoint;
pub mod patch;
pub mod to_value;
pub mod yuse;

pub(crate) trait AsStr {
	fn as_str(&self) -> &'static str;
}

impl AsStr for Action {
	fn as_str(&self) -> &'static str {
		match self {
			Action::Create => "CREATE",
			Action::Update => "UPDATE",
			Action::Delete => "DELETE",
			_ => "UNKNOWN",
		}
	}
}
