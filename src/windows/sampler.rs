use crate::{
	data::audio_file::AudioFile,
	windows::{View, Window},
};
use std::path::PathBuf;

pub struct Sampler {
	files: Vec<AudioFile>,
	files_to_remove: Vec<usize>,
}

impl Sampler {
	pub fn add_samples(&mut self, paths: Vec<PathBuf>) {
		paths
			.iter()
			.for_each(|path| self.files.push(AudioFile::new(path)));
	}
}

impl Default for Sampler {
	fn default() -> Self {
		Self {
			files: vec![],
			files_to_remove: Vec::with_capacity(1),
		}
	}
}

impl Window for Sampler {
	fn show(&mut self, ctx: &egui::Context, name: &'static str, open: &mut bool) {
		egui::Window::new(name)
			.open(open)
			.collapsible(false)
			.min_width(380.0)
			.show(ctx, |ui| self.ui(ui));
	}

	fn ui(&mut self, ui: &mut egui::Ui) {
		if ui.button("Add Audio").clicked() {
			let files = rfd::FileDialog::new()
				.add_filter(
					"Audio Files (*.mp3, *.wav, *.flac, etc)",
					&["mp3", "wav", "flac"],
				)
				.pick_files();

			self.add_samples(files.unwrap_or(vec![]));
		}

		self.files.iter().enumerate().for_each(|(index, file)| {
			let file_name = file.path.file_name().unwrap().to_str().unwrap();
			let full_path = file.path.as_os_str().to_str().unwrap();

			ui.horizontal(|ui| {
				ui.label(file_name).on_hover_text(full_path);
				if ui.button("❌").clicked() {
					self.files_to_remove.push(index);
				}
			});
		});

		self.files_to_remove.iter().for_each(|idx| {
			self.files.remove(*idx);
		});
		self.files_to_remove.clear();
	}

	fn as_any(&mut self) -> &mut dyn std::any::Any {
		self
	}
}
