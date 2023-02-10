use crate::{
	data::project::Project,
	resources::assets::setup_custom_fonts,
	windows::{menu::draw_application_menu, Windows},
};

pub struct SystemState {
	pub project: Project,
	pub windows: Windows,
}

impl SystemState {
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		setup_custom_fonts(&cc.egui_ctx);

		Self {
			project: Project::default(),
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
