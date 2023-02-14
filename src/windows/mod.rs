use self::{mixer::Mixer, sampler::Sampler, settings::Settings};
use crate::resources::strings;
use egui::Context;
use std::{
	any::Any,
	collections::{BTreeMap, BTreeSet},
};
use strum::{AsRefStr, Display};

pub mod application;
pub mod menu;
pub mod mixer;
pub mod sampler;
pub mod settings;

#[derive(Clone, Copy, Display, PartialEq, Eq, PartialOrd, Ord, AsRefStr)]
pub enum WindowName {
	#[strum(serialize = "Mixer")]
	Mixer,

	#[strum(serialize = "Sampler")]
	Sampler,

	#[strum(serialize = "Settings")]
	Settings,
}

pub trait Window {
	/// Show windows, etc
	fn show(&mut self, ctx: &egui::Context, name: &WindowName, open: &mut bool);

	/// Display GUI inside window
	fn ui(&mut self, ui: &mut egui::Ui);

	fn as_any(&mut self) -> &mut dyn Any;
}

pub struct Windows {
	windows: BTreeMap<WindowName, Box<dyn Window>>,
	open: BTreeSet<WindowName>,
}

impl Default for Windows {
	fn default() -> Self {
		let mut windows: BTreeMap<WindowName, Box<dyn Window>> = BTreeMap::new();
		windows.insert(WindowName::Mixer, Box::new(Mixer::default()));
		windows.insert(WindowName::Sampler, Box::new(Sampler::default()));
		windows.insert(WindowName::Settings, Box::new(Settings::default()));

		Self::new(windows)
	}
}

impl Windows {
	pub fn new(windows: BTreeMap<WindowName, Box<dyn Window>>) -> Self {
		let mut open = BTreeSet::new();
		open.insert(WindowName::Settings);

		Self { windows, open }
	}

	pub fn checkboxes(&mut self, ctx: &Context) {
		let Self { open, windows } = self;
		egui::TopBottomPanel::bottom("application-windows").show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.label(strings::WINDOWS_LABEL);
				for (name, _) in windows {
					let mut is_open = open.contains(name);
					ui.toggle_value(&mut is_open, name.as_ref());
					Windows::set_open(open, name, is_open);
				}
			})
		});
	}

	pub fn windows(&mut self, ctx: &Context) {
		let Self { windows, open } = self;
		for (name, window) in windows {
			let mut is_open = open.contains(name);
			window.show(ctx, name, &mut is_open);
			Windows::set_open(open, name, is_open);
		}
	}

	fn set_open(open: &mut BTreeSet<WindowName>, key: &WindowName, is_open: bool) {
		if is_open {
			if !open.contains(key) {
				open.insert(key.to_owned());
			}
		} else {
			open.remove(key);
		}
	}
}
