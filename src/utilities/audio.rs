use cpal::traits::HostTrait;
use cpal::Device;

pub struct DeviceResult {
	pub input_default: Device,
	pub output_default: Device,
	pub input_list: Vec<Device>,
	pub output_list: Vec<Device>,
}

impl DeviceResult {
	pub fn get_devices() -> DeviceResult {
		let host = cpal::default_host();
		DeviceResult {
			input_default: host.default_input_device().unwrap(),
			input_list: host.input_devices().unwrap().collect(),

			output_default: host.default_output_device().unwrap(),
			output_list: host.output_devices().unwrap().collect(),
		}
	}
}
