use super::AudioEngineEvent;
use crate::data::Project;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rtrb::Consumer;
use std::{
	sync::{Arc, RwLock},
	thread::{self, JoinHandle},
	time::Duration,
};

#[derive(Copy, Clone)]
pub struct EngineConfig {
	pub sample_rate: f64,
	pub channels: usize,
	pub timer: f64,
	pub real_buffer_size: Option<usize>,
	pub buffer_start: Option<cpal::StreamInstant>,
}

pub struct AudioEngine {
	stream: Option<cpal::Stream>,
}

impl AudioEngine {
	pub fn init(
		project: Arc<RwLock<Project>>,
		mut event_rx: Consumer<AudioEngineEvent>,
	) -> JoinHandle<()> {
		thread::spawn(move || {
			println!("AudioEngine Thread Created!");
			let mut engine = AudioEngine { stream: None };

			loop {
				if let Ok(upd) = event_rx.pop() {
					match upd {
						AudioEngineEvent::Disable => {
							println!("Disabling engine");
							engine.stream = None;
						}
						AudioEngineEvent::Enable {
							buffer_size,
							config,
							device_index,
						} => {
							println!("Enabling engine");
							println!(
								"{} channels at {} Hz (buffer size: {})",
								config.channels(),
								config.sample_rate().0,
								buffer_size,
							);

							engine.stream = AudioEngine::build_stream(
								config,
								device_index,
								buffer_size,
								project.clone(),
							);

							if let Some(stream) = &engine.stream {
								stream.play().unwrap();
								println!("Playing!");
							} else {
								panic!("Could not create stream");
							}
						}
					}
				}

				// TODO: Automatically wait for new events, rather than sleeping?
				std::thread::sleep(Duration::from_millis(100));
			}
		})
	}

	/// Create a new CPAL audio stream to handle project data
	fn build_stream(
		config: cpal::SupportedStreamConfig,
		device_index: usize,
		buffer_size: u32,
		project: Arc<RwLock<Project>>,
	) -> Option<cpal::Stream> {
		let device = &cpal::default_host()
			.output_devices()
			.unwrap()
			.collect::<Vec<cpal::Device>>()[device_index];

		println!("{} ({})", device.name().unwrap(), config.sample_format());

		let build_config = cpal::StreamConfig {
			buffer_size: cpal::BufferSize::Fixed(buffer_size),
			channels: 2, // TODO: Support less or more than 2 channels? How would the mixer work?
			sample_rate: config.sample_rate(),
		};

		let mut stream_config = EngineConfig {
			sample_rate: config.sample_rate().0 as f64,
			channels: config.channels() as usize,
			timer: 0.0,
			real_buffer_size: None,
			buffer_start: None,
		};

		let err_fn = move |err| eprintln!("{err}");

		match config.sample_format() {
			cpal::SampleFormat::F32 => device.build_output_stream(
				&build_config,
				move |data: &mut [f32], info: &cpal::OutputCallbackInfo| {
					AudioEngine::process_audio::<f32>(data, info, &project, &mut stream_config);
				},
				err_fn,
				None,
			),

			// TODO: Support other sample formats
			_ => panic!("Whoops!"),
		}
		.ok()
	}

	/// Process all synths, effects, and mixer channels.
	/// TODO: Implement! Get audio based on project play state (eg. "Playing", "Stopped", etc).
	fn process_audio<T: cpal::Sample + cpal::FromSample<f64> + std::ops::Add<Output = T>>(
		data: &mut [T],
		info: &cpal::OutputCallbackInfo,
		project: &Arc<RwLock<Project>>,
		config: &mut EngineConfig,
	) {
		config.buffer_start = Some(info.timestamp().playback);
		let project = project.read().unwrap();

		// config.sample_rate = data.len() as f64 * 50.0;
		if config.real_buffer_size.is_none() {
			config.real_buffer_size = Some(data.len());
		}

		let mix =
			project
				.mixer
				.render_buffer(&project, config.real_buffer_size.unwrap(), info, config);

		for (index, sample) in data.iter_mut().enumerate() {
			*sample = T::from_sample(mix[index]);
		}
	}
}
