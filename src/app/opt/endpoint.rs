use crate::err::Error;
use serde::Deserialize;
use std::collections::HashSet;
use std::time::Duration;
use surrealdb::opt::auth::Root;
use surrealdb::opt::capabilities;
use surrealdb::opt::Config;

#[derive(Deserialize)]
pub struct Options {
	pub capacity: Option<usize>,
	pub strict: Option<bool>,
	pub notifications: Option<bool>,
	pub query_timeout: Option<u8>,
	pub transaction_timeout: Option<u8>,
	pub user: Option<User>,
	pub tick_interval: Option<u8>,
	pub capabilities: Option<CapabilitiesConfig>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum User {
	Root {
		username: String,
		password: String,
	},
	/* Not supported yet
	Namespace { namespace: String, username: String, password: String },
	Database { namespace: String, database: String, username: String, password: String },
	*/
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum CapabilitiesConfig {
	Bool(bool),
	Capabilities {
		scripting: Option<bool>,
		guest_access: Option<bool>,
		live_query_notifications: Option<bool>,
		functions: Option<Targets>,
		network_targets: Option<Targets>,
	},
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Targets {
	Bool(bool),
	Array(HashSet<String>),
	Config {
		allow: Option<TargetsConfig>,
		deny: Option<TargetsConfig>,
	},
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum TargetsConfig {
	Bool(bool),
	Array(HashSet<String>),
}

impl TryFrom<Options> for Config {
	type Error = Error;

	fn try_from(opts: Options) -> Result<Self, Self::Error> {
		let mut config = Self::new();

		if let Some(strict) = opts.strict {
			config = config.set_strict(strict);
		}

		if let Some(query_timeout) = opts.query_timeout {
			config = config.query_timeout(Duration::from_secs(query_timeout as u64));
		}

		if let Some(transaction_timeout) = opts.transaction_timeout {
			config = config.transaction_timeout(Duration::from_secs(transaction_timeout as u64));
		}

		if let Some(user) = opts.user {
			match &user {
				User::Root {
					username,
					password,
				} => {
					config = config.user(Root {
						username,
						password,
					});
				}
			}
		}

		if let Some(tick_interval) = opts.tick_interval {
			config = config.tick_interval(Duration::from_secs(tick_interval as u64));
		}

		if let Some(capabilities) = opts.capabilities {
			config = config.capabilities(capabilities.try_into()?);
		}

		Ok(config)
	}
}

macro_rules! process_targets {
	($set:ident) => {{
		let mut functions = HashSet::with_capacity($set.len());
		for function in $set {
			functions.insert(function.parse()?);
		}
		capabilities::Targets::Some(functions)
	}};
}

impl TryFrom<CapabilitiesConfig> for capabilities::Capabilities {
	type Error = Error;

	fn try_from(config: CapabilitiesConfig) -> Result<Self, Self::Error> {
		match config {
			CapabilitiesConfig::Bool(true) => Ok(Self::all()),
			CapabilitiesConfig::Bool(false) => {
				Ok(Self::default().with_functions(capabilities::Targets::None))
			}
			CapabilitiesConfig::Capabilities {
				scripting,
				guest_access,
				live_query_notifications,
				functions,
				network_targets,
			} => {
				let mut capabilities = Self::default();

				if let Some(scripting) = scripting {
					capabilities = capabilities.with_scripting(scripting);
				}

				if let Some(guest_access) = guest_access {
					capabilities = capabilities.with_guest_access(guest_access);
				}

				if let Some(live_query_notifications) = live_query_notifications {
					capabilities =
						capabilities.with_live_query_notifications(live_query_notifications);
				}

				if let Some(functions) = functions {
					match functions {
						Targets::Bool(functions) => match functions {
							true => {
								capabilities =
									capabilities.with_functions(capabilities::Targets::All);
							}
							false => {
								capabilities =
									capabilities.with_functions(capabilities::Targets::None);
							}
						},
						Targets::Array(set) => {
							capabilities = capabilities.with_functions(process_targets!(set));
						}
						Targets::Config {
							allow,
							deny,
						} => {
							if let Some(config) = allow {
								match config {
									TargetsConfig::Bool(functions) => match functions {
										true => {
											capabilities = capabilities
												.with_functions(capabilities::Targets::All);
										}
										false => {
											capabilities = capabilities
												.with_functions(capabilities::Targets::None);
										}
									},
									TargetsConfig::Array(set) => {
										capabilities =
											capabilities.with_functions(process_targets!(set));
									}
								}
							}

							if let Some(config) = deny {
								match config {
									TargetsConfig::Bool(functions) => match functions {
										true => {
											capabilities = capabilities
												.without_functions(capabilities::Targets::All);
										}
										false => {
											capabilities = capabilities
												.without_functions(capabilities::Targets::None);
										}
									},
									TargetsConfig::Array(set) => {
										capabilities =
											capabilities.without_functions(process_targets!(set));
									}
								}
							}
						}
					}
				}

				if let Some(network_targets) = network_targets {
					match network_targets {
						Targets::Bool(network_targets) => match network_targets {
							true => {
								capabilities =
									capabilities.with_network_targets(capabilities::Targets::All);
							}
							false => {
								capabilities =
									capabilities.with_network_targets(capabilities::Targets::None);
							}
						},
						Targets::Array(set) => {
							capabilities = capabilities.with_network_targets(process_targets!(set));
						}
						Targets::Config {
							allow,
							deny,
						} => {
							if let Some(config) = allow {
								match config {
									TargetsConfig::Bool(network_targets) => match network_targets {
										true => {
											capabilities = capabilities
												.with_network_targets(capabilities::Targets::All);
										}
										false => {
											capabilities = capabilities
												.with_network_targets(capabilities::Targets::None);
										}
									},
									TargetsConfig::Array(set) => {
										capabilities = capabilities
											.with_network_targets(process_targets!(set));
									}
								}
							}

							if let Some(config) = deny {
								match config {
									TargetsConfig::Bool(network_targets) => match network_targets {
										true => {
											capabilities = capabilities.without_network_targets(
												capabilities::Targets::All,
											);
										}
										false => {
											capabilities = capabilities.without_network_targets(
												capabilities::Targets::None,
											);
										}
									},
									TargetsConfig::Array(set) => {
										capabilities = capabilities
											.without_network_targets(process_targets!(set));
									}
								}
							}
						}
					}
				}

				Ok(capabilities)
			}
		}
	}
}
