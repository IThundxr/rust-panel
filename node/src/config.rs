use config::File;
use inotify::{Inotify, WatchMask};
use serde::Deserialize;
use std::sync::{OnceLock, RwLock};
use uuid::Uuid;

const CONFIG_FILE: &str = "config.yml";

#[derive(Deserialize)]
pub struct Config {
	pub uuid: Uuid,
	pub token_id: String,
	pub token: String,
}

impl Config {
	pub fn config() -> &'static RwLock<config::Config> {
		static CONFIG: OnceLock<RwLock<config::Config>> = OnceLock::new();
		CONFIG.get_or_init(|| RwLock::new(Self::load()))
	}

	pub fn load() -> config::Config {
		config::Config::builder()
			.add_source(File::with_name(CONFIG_FILE))
			.build()
			.unwrap()
	}

	pub fn reload() {
		*Self::config().write().unwrap() = Self::load();
	}

	pub fn setup_watcher() -> std::thread::JoinHandle<()> {
		std::thread::spawn(|| {
			let mut inotify = Inotify::init().unwrap();

			inotify
				.watches()
				.add(CONFIG_FILE, WatchMask::MODIFY | WatchMask::CLOSE_WRITE)
				.unwrap();

			let mut buffer = [0u8; 4096];
			loop {
				let events = inotify.read_events_blocking(&mut buffer).unwrap();

				for _ in events {
					Config::reload()
				}
			}
		})
	}
}
