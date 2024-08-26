use crate::err::Error;
use serde::Deserialize;
use std::collections::HashSet;
#[cfg(not(feature = "surrealdb2"))]
use surrealdb::opt::capabilities;
#[cfg(feature = "surrealdb2")]
use surrealdb2::dbs::capabilities;

#[derive(Deserialize)]
pub struct Options {
	pub strict: Option<bool>,
	pub query_timeout: Option<u8>,
	pub transaction_timeout: Option<u8>,
	pub capabilities: Option<CapabilitiesConfig>,
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

#[cfg(not(feature = "surrealdb2"))]
impl TryFrom<CapabilitiesConfig> for capabilities::Capabilities {
	type Error = Error;

	fn try_from(config: CapabilitiesConfig) -> Result<Self, Self::Error> {
		match config {
			CapabilitiesConfig::Bool(true) => Ok(Self::all()),
			CapabilitiesConfig::Bool(false) => Ok(Self::default().with_deny_all_function()),
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
								capabilities = capabilities.with_allow_all_functions();
							}
							false => {
								capabilities = capabilities.with_deny_all_function();
							}
						},
						Targets::Array(set) => {
							for fnc in set.iter() {
								capabilities = capabilities
									.with_allow_function(fnc)
									.map_err(|e| e.to_string())?;
							}
						}
						Targets::Config {
							allow,
							deny,
						} => {
							if let Some(config) = allow {
								match config {
									TargetsConfig::Bool(functions) => match functions {
										true => {
											capabilities = capabilities.with_allow_all_functions();
										}
										false => {
											capabilities = capabilities.with_deny_all_function();
										}
									},
									TargetsConfig::Array(set) => {
										for fnc in set.iter() {
											capabilities = capabilities
												.with_allow_function(fnc)
												.map_err(|e| e.to_string())?;
										}
									}
								}
							}

							if let Some(config) = deny {
								match config {
									TargetsConfig::Bool(functions) => match functions {
										true => {
											capabilities = capabilities.with_allow_none_functions()
										}
										false => {
											capabilities = capabilities.with_deny_none_function()
										}
									},
									TargetsConfig::Array(set) => {
										for fnc in set.iter() {
											capabilities = capabilities
												.with_deny_function(fnc)
												.map_err(|e| e.to_string())?;
										}
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
								capabilities = capabilities.with_allow_all_net_targets();
							}
							false => capabilities = capabilities.with_deny_all_net_target(),
						},
						Targets::Array(set) => {
							for net_target in set.iter() {
								capabilities = capabilities
									.with_allow_net_target(net_target)
									.map_err(|e| e.to_string())?;
							}
						}
						Targets::Config {
							allow,
							deny,
						} => {
							if let Some(config) = allow {
								match config {
									TargetsConfig::Bool(network_targets) => match network_targets {
										true => {
											capabilities =
												capabilities.with_allow_all_net_targets();
										}
										false => {
											capabilities = capabilities.with_deny_all_net_target();
										}
									},
									TargetsConfig::Array(set) => {
										for net_target in set.iter() {
											capabilities = capabilities
												.with_allow_net_target(net_target)
												.map_err(|e| e.to_string())?;
										}
									}
								}
							}

							if let Some(config) = deny {
								match config {
									TargetsConfig::Bool(network_targets) => match network_targets {
										true => {
											capabilities =
												capabilities.with_allow_none_net_targets();
										}
										false => {
											capabilities = capabilities.with_deny_none_net_target();
										}
									},
									TargetsConfig::Array(set) => {
										for net_target in set.iter() {
											capabilities = capabilities
												.with_deny_net_target(net_target)
												.map_err(|e| e.to_string())?;
										}
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
