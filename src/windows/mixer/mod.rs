use super::application::SystemState;
use crate::{
	resources::strings,
	windows::{Window, WindowName},
};
use egui::{TopBottomPanel, Ui};

pub mod channel;
pub mod mixer;

pub struct MixerWindow {}

impl Window for MixerWindow {
	fn show(
		&mut self,
		ctx: &egui::Context,
		name: &WindowName,
		open: &mut bool,
		state: &mut SystemState,
	) {
		egui::Window::new(name.as_ref())
			.open(open)
			.resizable(true)
			.collapsible(false)
			.default_width(640.0)
			.show(ctx, |ui| self.ui(ui, state));
	}

	fn ui(&mut self, ui: &mut Ui, state: &mut SystemState) {
		TopBottomPanel::top("mixer_menu").show_inside(ui, |ui| {
			if ui.button(strings::MIXER_NEW_CHANNEL).clicked() {
				state.mixer.add_channel();
			}
		});

		egui::ScrollArea::horizontal().show(ui, |ui| {
			ui.horizontal(|ui| {
				state.mixer
					.channels
					.iter_mut()
					.enumerate()
					.for_each(|(idx, c)| {
						c.view(ui, idx, &mut state.mixer.remove_queue)
					});
			});
		});

		state.mixer.clean_channels();
	}

	fn as_any(&mut self) -> &mut dyn std::any::Any {
		self
	}
}
