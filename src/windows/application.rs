use crate::{
	resources::{assets::setup_custom_fonts, strings::PROJECT_DEFAULT_NAME},
	windows::{
		menu::draw_application_menu,
		mixer::{Channel, Mixer},
		Window,
	},
};
use egui::{Context, Ui};
use std::collections::BTreeSet;

pub struct Project {
	pub name: String,
}

impl Project {
	pub fn new(name: &str) -> Self {
		Self {
			name: name.to_owned(),
		}
	}
}

pub struct Windows {
	windows: Vec<Box<dyn Window>>,

	open: BTreeSet<String>,
}

impl Default for Windows {
	fn default() -> Self {
		Self::new(vec![
			Box::new(Mixer {
				channels: vec![
					Channel::new(Some("Master")),
					Channel::new(Some("Channel 1")),
					Channel::new(Some("Channel 2")),
				],
			}),
			// Box::new(super::code_editor::CodeEditor::default()),
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

pub struct SystemState {
	pub project: Project,
	pub windows: Windows,
}

impl SystemState {
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		Self {
			project: Project::new(PROJECT_DEFAULT_NAME),
			windows: Windows::default(),
		}
	}
}

impl eframe::App for SystemState {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		draw_application_menu(ctx);
		egui::TopBottomPanel::top("windows").show(ctx, |ui| self.windows.checkboxes(ui));

		self.windows.windows(ctx);
	}
}
