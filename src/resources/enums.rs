use std::{error::Error, time::Instant};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PlayState {
	Stopped,
	Paused {
		start_time: Instant,
		pause_time: Instant,
	},
	Playing {
		start_time: Instant,
	},
}

#[derive(Debug)]
pub enum Progress {
	InProgress(f32),
	Finished,
	Cancelled,
	Error(Box<dyn Error>),
}
