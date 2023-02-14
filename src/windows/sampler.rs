use super::WindowName;
use crate::{data::SystemState, resources::strings, windows::Window};

pub struct SamplerWindow {}

impl Default for SamplerWindow {
	fn default() -> Self {
		Self {}
	}
}

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

		state.sampler
			.files
			.iter()
			.enumerate()
			.for_each(|(index, file)| {
				let file_name = file.path.file_name().unwrap().to_str().unwrap();
				let full_path = file.path.as_os_str().to_str().unwrap();

				ui.horizontal(|ui| {
					ui.label(file_name).on_hover_text(full_path);
					if ui.button("❌").clicked() {
						state.sampler.remove_queue.push(index);
					}
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
