use self::{mixer::Mixer, sampler::Sampler};
use egui::{Context, Ui};
use std::collections::BTreeSet;

pub mod application;
pub mod menu;
pub mod mixer;
pub mod sampler;

pub trait Window {
	/// `&'static` so we can also use it as a key to store open/close state.
	fn name(&self) -> &'static str;

	/// Show windows, etc
	fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}

pub trait View {
	fn ui(&mut self, ui: &mut egui::Ui);
}

pub struct Windows {
	windows: Vec<Box<dyn Window>>,
	open: BTreeSet<String>,
}

impl Default for Windows {
	fn default() -> Self {
		Self::new(vec![
			Box::new(Mixer::default()),
			Box::new(Sampler::default()),
		])
	}
}

impl Windows {
	pub fn new(windows: Vec<Box<dyn Window>>) -> Self {
		let open = BTreeSet::new();

		Self { windows, open }
	}

	pub fn checkboxes(&mut self, ui: &mut Ui) {
		let Self { windows, open } = self;
		for window in windows {
			let mut is_open = open.contains(window.name());
			ui.toggle_value(&mut is_open, window.name());
			Windows::set_open(open, window.name(), is_open);
		}
	}

	pub fn windows(&mut self, ctx: &Context) {
		let Self { windows, open } = self;
		for window in windows {
			let mut is_open = open.contains(window.name());
			window.show(ctx, &mut is_open);
			Windows::set_open(open, window.name(), is_open);
		}
	}

	fn set_open(open: &mut BTreeSet<String>, key: &'static str, is_open: bool) {
		if is_open {
			if !open.contains(key) {
				open.insert(key.to_owned());
			}
		} else {
			open.remove(key);
		}
	}
}
