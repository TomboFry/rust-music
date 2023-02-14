use super::WindowName;
use crate::{data::audio_file::AudioFile, resources::strings, windows::Window};

pub struct Sampler {
	files: Vec<AudioFile>,
	remove_queue: Vec<usize>,
}

impl Sampler {
	pub fn add_samples(&mut self) {
		let files = rfd::FileDialog::new()
			.add_filter(
				strings::FILE_PICKER_AUDIO_NAME,
				strings::FILE_PICKER_AUDIO_EXTENSIONS,
			)
			.pick_files();

		files.unwrap_or(vec![])
			.iter()
			.for_each(|path| self.files.push(AudioFile::new(path)));
	}

	pub fn clean_samples(&mut self) {
		if self.remove_queue.len() == 0 {
			return;
		}

		self.remove_queue.iter().for_each(|idx| {
			self.files.remove(*idx);
		});

		self.remove_queue.clear();
	}
}

impl Default for Sampler {
	fn default() -> Self {
		Self {
			files: vec![],
			remove_queue: Vec::with_capacity(1),
		}
	}
}

impl Window for Sampler {
	fn show(&mut self, ctx: &egui::Context, name: &WindowName, open: &mut bool) {
		egui::Window::new(name.as_ref())
			.open(open)
			.collapsible(false)
			.min_width(380.0)
			.show(ctx, |ui| self.ui(ui));
	}

	fn ui(&mut self, ui: &mut egui::Ui) {
		if ui.button(strings::SAMPLER_ADD_LABEL).clicked() {
			self.add_samples();
		}

		self.files.iter().enumerate().for_each(|(index, file)| {
			let file_name = file.path.file_name().unwrap().to_str().unwrap();
			let full_path = file.path.as_os_str().to_str().unwrap();

			ui.horizontal(|ui| {
				ui.label(file_name).on_hover_text(full_path);
				if ui.button("❌").clicked() {
					self.remove_queue.push(index);
				}
			});
		});

		self.clean_samples();
	}

	fn as_any(&mut self) -> &mut dyn std::any::Any {
		self
	}
}
