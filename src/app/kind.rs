use crate::app::local::Local;
use crate::app::remote::Remote;
use std::rc::Rc;
use std::sync::RwLock;

pub enum Kind {
	None,
	Local(Rc<RwLock<Local>>),
	Remote(Rc<RwLock<Remote>>),
}
