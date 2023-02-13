use self::channel::Channel;
use crate::{resources::strings, windows::Window};
use egui::{TopBottomPanel, Ui};

mod channel;

pub struct Mixer {
	pub channels: Vec<Channel>,
	pub remove_queue: Vec<usize>,
}

impl Mixer {
	pub fn add_channel(&mut self) {
		let len = self.channels.len();
		let name = format!("{} {}", strings::CHANNEL_DEFAULT_NAME, len + 1);
		let channel = Channel::new(Some(&name));

		self.channels.push(channel);
	}

	pub fn clean_channels(&mut self) {
		if self.remove_queue.len() == 0 {
			return;
		}

		self.remove_queue.iter().for_each(|idx| {
			self.channels.remove(*idx);
		});

		self.remove_queue.clear();
	}
}

impl Default for Mixer {
	fn default() -> Self {
		Self {
			channels: vec![
				Channel::new(Some("Master")),
				Channel::new(Some("Channel 1")),
				Channel::new(Some("Channel 2")),
			],
			remove_queue: Vec::with_capacity(1),
		}
	}
}

impl Window for Mixer {
	fn show(&mut self, ctx: &egui::Context, name: &'static str, open: &mut bool) {
		egui::Window::new(name)
			.open(open)
			.resizable(true)
			.collapsible(false)
			.default_width(640.0)
			.show(ctx, |ui| self.ui(ui));
	}

	fn ui(&mut self, ui: &mut Ui) {
		TopBottomPanel::top("mixer_menu").show_inside(ui, |ui| {
			if ui.button(strings::MIXER_NEW_CHANNEL).clicked() {
				self.add_channel();
			}
		});

		egui::ScrollArea::horizontal().show(ui, |ui| {
			ui.horizontal(|ui| {
				self.channels.iter_mut().enumerate().for_each(|(idx, c)| {
					c.view(ui, idx, &mut self.remove_queue)
				});
			});
		});

		self.clean_channels();
	}

	fn as_any(&mut self) -> &mut dyn std::any::Any {
		self
	}
}
