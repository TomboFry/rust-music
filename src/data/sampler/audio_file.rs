use std::{fs::File, io::Read, path::PathBuf};

pub struct AudioFile {
	pub path: PathBuf,
	pub data: Vec<u8>,
}

impl AudioFile {
	pub fn load_from_disk(path: &PathBuf) -> Self {
		let mut f = File::open(path).unwrap();
		let mut buffer = Vec::new();
		f.read_to_end(&mut buffer).unwrap();

		AudioFile {
			path: path.to_owned(),
			data: buffer,
		}
	}
}
