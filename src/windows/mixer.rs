use crate::{
	resources::strings::CHANNEL_DEFAULT_NAME,
	windows::{View, Window},
};
use egui::{Slider, TextEdit, Ui, Vec2};

pub struct Channel {
	pub volume: f64,
	pub panning: f64,
	pub muted: bool,
	pub name: String,
}

impl Channel {
	pub fn new(name: Option<&str>) -> Self {
		Self {
			volume: 0.0,
			panning: 0.0,
			muted: false,
			name: name.unwrap_or(CHANNEL_DEFAULT_NAME).to_owned(),
		}
	}

	pub fn view(&mut self, ui: &mut Ui) {
		ui.vertical(|ui| {
			ui.add(
				TextEdit::singleline(&mut self.name)
					.desired_width(96.0)
					.min_size(Vec2::new(96.0, 16.0)),
			);

			ui.add(Slider::new(&mut self.panning, -1.0..=1.0).show_value(false))
				.on_hover_text_at_pointer(format!("{:.0}%", self.panning * 100.0));

			ui.add_enabled_ui(!self.muted, |ui| {
				ui.add(
					Slider::new(&mut self.volume, -30.0..=6.0)
						.vertical()
						.show_value(false),
				)
				.on_hover_text_at_pointer(format!("{:.1} dB", self.volume))
			});

			ui.toggle_value(&mut self.muted, "Muted");
		});
	}
}

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

	fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
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
	fn ui(&mut self, ui: &mut egui::Ui) {
		egui::TopBottomPanel::top("mixer_menu").show_inside(ui, |ui| {
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
