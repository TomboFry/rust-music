use crate::{
	data::channel::Channel,
	resources::strings::CHANNEL_DEFAULT_NAME,
	windows::{View, Window},
};
use egui::{Context, TopBottomPanel, Ui};

pub struct Mixer {
	pub channels: Vec<Channel>,
}

impl Default for Mixer {
	fn default() -> Self {
		Self {
			channels: vec![
				Channel::new(Some("Master")),
				Channel::new(Some("Channel 1")),
				Channel::new(Some("Channel 2")),
			],
		}
	}
}

impl Window for Mixer {
	fn name(&self) -> &'static str {
		"Mixer"
	}

	fn show(&mut self, ctx: &Context, open: &mut bool) {
		egui::Window::new(self.name())
			.open(open)
			.resizable(true)
			.collapsible(false)
			.default_width(640.0)
			.hscroll(true)
			.show(ctx, |ui| self.ui(ui));
	}
}

impl View for Mixer {
	fn ui(&mut self, ui: &mut Ui) {
		TopBottomPanel::top("mixer_menu").show_inside(ui, |ui| {
			if ui.button("New Channel").clicked() {
				let len = self.channels.len();
				let name = format!("{} {}", CHANNEL_DEFAULT_NAME, len);
				let channel = Channel::new(Some(&name));

				self.channels.push(channel);
			}
		});

		ui.horizontal(|ui| {
			self.channels.iter_mut().for_each(|c| c.view(ui));
		});
	}
}
