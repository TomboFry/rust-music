use super::{main_menu::draw_application_menu, windows::Windows};
use crate::{
	data::{ui_event_handler, AudioEngine, Project, SystemState},
	resources::assets::setup_custom_fonts,
};
use rtrb::RingBuffer;
use std::sync::{Arc, RwLock};

pub struct System {
	pub system: SystemState,
	pub project: Arc<RwLock<Project>>,
	pub windows: Windows,
}

impl System {
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		setup_custom_fonts(&cc.egui_ctx);

		// Audio Engine Updates
		let (event_tx, event_rx) = RingBuffer::new(64);

		let system = SystemState::new(event_tx);

		// Share project state between threads
		let project = Project::default();
		let project_lock_a = Arc::new(RwLock::new(project));
		let project_lock_b = project_lock_a.clone();

		let _engine_thread = AudioEngine::init(project_lock_b, event_rx);

		Self {
			system,
			project: project_lock_a,
			windows: Windows::default(),
		}
	}
}

impl eframe::App for System {
	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
		draw_application_menu(
			ctx,
			frame,
			&mut self.windows,
			&mut self.system,
			&self.project,
		);

		// TODO: Replace with rectangle to display custom colour or image
		egui::panel::CentralPanel::default().show(ctx, |_| {});

		self.windows.checkboxes(ctx);
		self.windows.windows(ctx, &self.project, &mut self.system);

		ui_event_handler(&mut self.project, &mut self.system.ui_events);

		ctx.request_repaint();
	}
}
