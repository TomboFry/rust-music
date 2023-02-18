use crate::resources::strings;

pub struct Sampler {
	pub files: Vec<AudioFile>,
}

mod audio_file;
pub use audio_file::AudioFile;

impl Sampler {
	pub fn add_samples(&mut self) {
		let files = rfd::FileDialog::new()
			.add_filter(
				strings::FILE_PICKER_AUDIO_NAME,
				strings::FILE_PICKER_AUDIO_EXTENSIONS,
			)
			.pick_files();

		files.unwrap_or(vec![]).iter().for_each(|path| {
			self.files.push(AudioFile::load_from_disk(path));
		});
	}
}

impl Default for Sampler {
	fn default() -> Self {
		Self { files: vec![] }
	}
}
