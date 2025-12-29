use bollard::Docker;

#[derive(Clone)]
pub(crate) struct App {
	pub(crate) docker: Docker,
}

impl App {
	pub(crate) fn new() -> Self {
		Self {
			docker: Docker::connect_with_socket_defaults().unwrap(), // TODO - Config/Env
		}
	}
}
