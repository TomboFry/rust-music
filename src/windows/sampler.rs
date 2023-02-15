use super::WindowName;
use crate::{
	data::{AudioFile, SystemState},
	resources::strings,
	windows::Window,
};

#[derive(Default)]
pub struct SamplerWindow {}

impl Window for SamplerWindow {
	fn show(
		&mut self,
		ctx: &egui::Context,
		name: &WindowName,
		open: &mut bool,
		state: &mut SystemState,
	) {
		egui::Window::new(name.as_ref())
			.open(open)
			.collapsible(false)
			.min_width(380.0)
			.show(ctx, |ui| self.ui(ui, state));
	}

	fn ui(&mut self, ui: &mut egui::Ui, state: &mut SystemState) {
		if ui.button(strings::SAMPLER_ADD_LABEL).clicked() {
			state.sampler.add_samples();
		}

		let mut ui_sample = |index, file: &mut AudioFile, ui: &mut egui::Ui| {
			let file_name = file.path.file_name().unwrap().to_str().unwrap();
			let full_path = file.path.as_os_str().to_str().unwrap();

			ui.add(egui::DragValue::new(&mut file.channel)
				.clamp_range(0..=state.mixer.channels.len() - 1));

			if ui.button("❌").clicked() {
				state.sampler.remove_queue.push(index);
			}

			ui.label(file_name).on_hover_text(full_path);
		};

		egui::ScrollArea::vertical().show(ui, |ui| {
			state.sampler
				.files
				.iter_mut()
				.enumerate()
				.for_each(|(index, file)| {
					ui.horizontal(|ui| ui_sample(index, file, ui));
				});
		});

		state.sampler.clean_samples();
	}

	fn as_any(&mut self) -> &mut dyn std::any::Any {
		self
	}

	fn toggle_shortcut(&self) -> Option<egui::KeyboardShortcut> {
		Some(egui::KeyboardShortcut::new(
			egui::Modifiers::CTRL | egui::Modifiers::SHIFT,
			egui::Key::A,
		))
	}
}
