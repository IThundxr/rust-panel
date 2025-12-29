use crate::config::Config;
use bollard::Docker;

#[derive(Clone)]
pub struct App {
	pub docker: Docker,
}

impl App {
	pub fn new() -> Self {
		Self {
			docker: Docker::connect_with_socket_defaults().unwrap(), // TODO - Config/Env
		}
	}

	pub fn config() -> Config {
		Config::config()
			.read()
			.unwrap()
			.clone()
			.try_deserialize::<Config>()
			.unwrap()
	}
}
