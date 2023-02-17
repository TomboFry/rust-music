use std::{fs::File, io::Read, path::PathBuf};

use crate::resources::PlayState;

pub struct AudioFile {
	pub path: PathBuf,
	pub data: Vec<u8>,
	pub channel: usize,
	pub play_state: PlayState,
}

impl AudioFile {
	pub fn load_from_disk(path: &PathBuf) -> Self {
		// TODO: Load asynchronously
		let mut f = File::open(path).unwrap();
		let mut buffer = Vec::new();
		f.read_to_end(&mut buffer).unwrap();

		AudioFile {
			path: path.to_owned(),
			data: buffer,
			channel: 0,
			play_state: PlayState::Stopped,
		}
	}
}
