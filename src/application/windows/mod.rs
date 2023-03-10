use self::{mixer::MixerWindow, sampler::SamplerWindow, settings::SettingsWindow};
use crate::{
	data::{Project, SystemState},
	resources::strings,
};
use std::{
	collections::{BTreeMap, BTreeSet},
	sync::{Arc, RwLock},
};
use strum::{AsRefStr, Display, EnumIter};

pub mod mixer;
pub mod sampler;
pub mod settings;

#[derive(AsRefStr, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
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
	fn show(
		&mut self,
		ctx: &egui::Context,
		name: &WindowName,
		open: &mut bool,
		state: &Arc<RwLock<Project>>,
		system: &mut SystemState,
	);

	/// Display GUI inside window
	fn ui(&mut self, ui: &mut egui::Ui, state: &Arc<RwLock<Project>>, system: &mut SystemState);

	fn toggle_shortcut(&self) -> Option<egui::KeyboardShortcut>;
}

type WindowMap = BTreeMap<WindowName, Box<dyn Window>>;

pub struct Windows {
	pub windows: WindowMap,
	pub open: BTreeSet<WindowName>,
}

impl Default for Windows {
	fn default() -> Self {
		let mut windows: WindowMap = BTreeMap::new();
		windows.insert(WindowName::Mixer, Box::new(MixerWindow::default()));
		windows.insert(WindowName::Sampler, Box::new(SamplerWindow::default()));
		windows.insert(WindowName::Settings, Box::new(SettingsWindow::default()));

		Self::new(windows)
	}
}

impl Windows {
	pub fn new(windows: WindowMap) -> Self {
		let mut open = BTreeSet::new();
		open.insert(WindowName::Sampler);
		open.insert(WindowName::Settings);
		open.insert(WindowName::Mixer);

		Self { windows, open }
	}

	pub fn checkboxes(&mut self, ctx: &egui::Context) {
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

	pub fn windows(
		&mut self,
		ctx: &egui::Context,
		state: &Arc<RwLock<Project>>,
		system: &mut SystemState,
	) {
		let Self { windows, open } = self;
		for (name, window) in windows {
			let mut is_open = open.contains(name);

			// Display Window
			window.show(ctx, name, &mut is_open, state, system);
			Windows::set_open(open, name, is_open);

			// Handle toggle shortcuts
			if let Some(shortcut) = window.toggle_shortcut() {
				if ctx.input_mut(|i| i.consume_shortcut(&shortcut)) {
					Windows::set_open(open, name, !is_open);
				}
			}
		}
	}

	fn set_open(open: &mut BTreeSet<WindowName>, key: &WindowName, is_open: bool) {
		if is_open {
			if !open.contains(key) {
				open.insert(key.to_owned());
			}
		} else {
			if open.contains(key) {
				open.remove(key);
			}
		}
	}
}
