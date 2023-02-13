use crate::resources::strings;
use egui::{Slider, TextEdit, TextStyle, Ui, Vec2};
use egui_extras_xt::knobs::AudioKnob;

pub struct Channel {
	pub volume: f32,
	pub panning: f32,
	pub muted: bool,
	pub name: String,
}

impl Channel {
	pub fn new(name: Option<&str>) -> Self {
		Self {
			volume: 0.0,
			panning: 0.0,
			muted: false,
			name: name.unwrap_or(strings::CHANNEL_DEFAULT_NAME).to_owned(),
		}
	}

	fn view_contents(&mut self, ui: &mut Ui, index: usize, remove_queue: &mut Vec<usize>) {
		ui.add(TextEdit::singleline(&mut self.name)
			.desired_width(64.0)
			.font(TextStyle::Small)
			.min_size(Vec2::new(64.0, 12.0)));

		// Panning
		ui.add(AudioKnob::new(&mut self.panning)
			.range(-1.0..=1.0)
			.spread(0.75)
			.drag_length(4.0)
			.animated(false)
			.shape(egui_extras_xt::common::WidgetShape::Circle));

		let label = if self.panning == 0.0 {
			""
		} else if self.panning > 0.0 {
			"R"
		} else {
			"L"
		};

		ui.label(format!("{:.0}% {}", (self.panning * 100.0).abs(), label));

		// Volume
		ui.add_enabled_ui(!self.muted, |ui| {
			ui.horizontal(|ui| {
				ui.allocate_space(Vec2::splat(16.0));
				ui.add(Slider::new(&mut self.volume, -30.0..=6.0)
					.vertical()
					.show_value(false))
					.on_hover_text_at_pointer(format!("{:.1} dB", self.volume));
			});
		});

		ui.toggle_value(&mut self.muted, "M");

		// First index is the master channel - let's not remove that!
		if index > 0 {
			if ui.button("❌").clicked() {
				remove_queue.push(index);
			}
		}
	}

	pub fn view(&mut self, ui: &mut Ui, index: usize, remove_queue: &mut Vec<usize>) {
		ui.group(|ui| {
			ui.allocate_ui_with_layout(
				Vec2::new(64.0, 256.0),
				egui::Layout::top_down_justified(egui::Align::Center),
				|ui| self.view_contents(ui, index, remove_queue),
			);
		});
	}
}
