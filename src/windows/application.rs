use super::{main_menu::draw_application_menu, Windows};
use crate::{
	data::{ui_event_handler, AudioEngine, Project, SystemState},
	resources::{assets::setup_custom_fonts, UiEvent},
};
use rtrb::RingBuffer;
use std::{
	collections::VecDeque,
	sync::{Arc, RwLock},
};

pub struct System {
	pub system: SystemState,
	pub project: Arc<RwLock<Project>>,
	pub ui_events: VecDeque<UiEvent>,
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
			ui_events: VecDeque::with_capacity(10),
			windows: Windows::default(),
		}
	}
}

impl eframe::App for System {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		draw_application_menu(ctx, &mut self.windows, &self.project, &mut self.ui_events);

		// TODO: Replace with rectangle to display custom colour or image
		egui::panel::CentralPanel::default().show(ctx, |_| {});

		self.windows.checkboxes(ctx);
		self.windows
			.windows(ctx, &self.project, &mut self.system, &mut self.ui_events);

		ui_event_handler(&mut self.project, &mut self.ui_events);
	}
}
