use self::{mixer::Mixer, sampler::Sampler, settings::Settings};
use crate::resources::strings;
use egui::Context;
use std::{
	any::Any,
	collections::{BTreeMap, BTreeSet},
};

pub mod application;
pub mod menu;
pub mod mixer;
pub mod sampler;
pub mod settings;

pub trait Window {
	/// Show windows, etc
	fn show(&mut self, ctx: &egui::Context, name: &'static str, open: &mut bool);

	/// Display GUI inside window
	fn ui(&mut self, ui: &mut egui::Ui);

	fn as_any(&mut self) -> &mut dyn Any;
}

pub struct Windows {
	windows: BTreeMap<&'static str, Box<dyn Window>>,
	open: BTreeSet<String>,
}

impl Default for Windows {
	fn default() -> Self {
		let mut windows: BTreeMap<&'static str, Box<dyn Window>> = BTreeMap::new();
		windows.insert("Mixer", Box::new(Mixer::default()));
		windows.insert("Sampler", Box::new(Sampler::default()));
		windows.insert("Settings", Box::new(Settings::default()));

		Self::new(windows)
	}
}

impl Windows {
	pub fn new(windows: BTreeMap<&'static str, Box<dyn Window>>) -> Self {
		let mut open = BTreeSet::new();
		open.insert("Settings".to_owned());

		Self { windows, open }
	}

	pub fn checkboxes(&mut self, ctx: &Context) {
		let Self { open, windows } = self;
		egui::TopBottomPanel::bottom("application-windows").show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.label(strings::WINDOWS_LABEL);
				for (name, _) in windows {
					let mut is_open = open.contains(*name);
					ui.toggle_value(&mut is_open, *name);
					Windows::set_open(open, name, is_open);
				}
			})
		});
	}

	pub fn windows(&mut self, ctx: &Context) {
		let Self { windows, open } = self;
		for (name, window) in windows {
			let mut is_open = open.contains(*name);
			window.show(ctx, name, &mut is_open);
			Windows::set_open(open, name, is_open);
		}
	}

	fn set_open(open: &mut BTreeSet<String>, key: &str, is_open: bool) {
		if is_open {
			if !open.contains(key) {
				open.insert(key.to_owned());
			}
		} else {
			open.remove(key);
		}
	}
}
