use rtrb::{Consumer, Producer, RingBuffer};
use std::{thread, time::Duration};

/// 2 seconds of stereo 192kHz audio.
const MAX_SAMPLES: usize = 192000 * 2 * 2;

pub struct AudioEngine {
	// producer: Producer<f32>,
	pub thread: thread::JoinHandle<()>,
}

impl AudioEngine {
	pub fn new() -> (Self, Consumer<f32>) {
		let (mut producer, consumer) = RingBuffer::<f32>::new(MAX_SAMPLES);

		let thread = thread::spawn(move || loop {
			// TODO: Detect changes to audio settings so that the
			//       engine is re-initialised correctly.

			AudioEngine::process_audio(&mut producer);

			// TODO: Calculate the correct sleep value.
			std::thread::sleep(Duration::from_millis(20));
		});

		(Self { thread }, consumer)
	}

	pub fn process_audio(producer: &mut Producer<f32>) {
		// TODO: Read SystemState from t'other thread to generate sound

		let mut chunk = producer.write_chunk(4096).unwrap();
		let (a, b) = chunk.as_mut_slices();

		for sample in a {
			*sample = (rand::random::<f32>() * 0.5) - 0.25;
		}

		for sample in b {
			*sample = (rand::random::<f32>() * 0.5) - 0.25;
		}

		chunk.commit_all();
	}
}
