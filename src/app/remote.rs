use surrealdb::Session;

pub struct Remote {
	url: String,
	session: Session,
}

impl Remote {
	pub fn new(url: String) -> Remote {
		Remote {
			url,
			session: Session::default(),
		}
	}
}
