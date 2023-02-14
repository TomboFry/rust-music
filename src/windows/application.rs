use crate::{
	data::project::Project,
	resources::assets::setup_custom_fonts,
	windows::{menu::draw_application_menu, mixer::mixer::Mixer, Windows},
};

pub struct SystemState {
	pub project: Project,
	pub mixer: Mixer,
}

impl Default for SystemState {
	fn default() -> Self {
		Self {
			project: Project::default(),
			mixer: Mixer::default(),
		}
	}
}

pub struct System {
	pub state: SystemState,
	pub windows: Windows,
}

impl System {
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		setup_custom_fonts(&cc.egui_ctx);

		Self {
			state: SystemState::default(),
			windows: Windows::default(),
		}
	}
}

impl eframe::App for System {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		draw_application_menu(ctx, &mut self.windows, &mut self.state);

		egui::panel::CentralPanel::default().show(ctx, |_| {});

		self.windows.checkboxes(ctx);
		self.windows.windows(ctx, &mut self.state);
	}
}
