use log::Level;
use log::Record;
use web_sys::console;

pub fn init() {
	// Display stack traces if enabled
	#[cfg(feature = "stack-traces")]
	console_error_panic_hook::set_once();
	// Ensure logs are output accordingly
	let mut logger = fern::Dispatch::new();
	logger = logger.level_for("surrealdb", log::LevelFilter::Trace);
	logger = logger.level(log::LevelFilter::Trace);
	logger = logger.chain(fern::Output::call(log));
	logger.apply().unwrap();
}

fn log(record: &Record) {
	// Use respective console output level
	let console_log = match record.level() {
		Level::Error => console::error_1,
		Level::Warn => console::warn_1,
		Level::Info => console::info_1,
		Level::Debug => console::log_1,
		Level::Trace => console::debug_1,
	};
	// Output the log message to the console
	console_log(&format!("{}", record.args()).into());
}
