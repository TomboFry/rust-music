use crate::resources::strings;
use std::path::Path;
use std::sync::{Arc, Mutex};
use vst::host::{Host, PluginLoader};
use vst::plugin::Plugin;

pub struct SampleHost;

impl Host for SampleHost {
	fn automate(&self, index: i32, value: f32) {
		println!("Parameter {index} changed to {value}");
	}
}

#[derive(Default)]
pub struct VstSettings {
	pub vst_list: Vec<(String, String)>,
}

impl VstSettings {
	pub fn reload_vsts(&mut self) {
		let mut vst_path_list = Vec::with_capacity(10);
		for path in strings::VST_PATHS {
			vst_path_list.append(&mut get_files_recursively(path));
		}

		self.vst_list = vst_path_list
			.iter()
			.filter_map(|entry| {
				// Give every plugin its own host?
				let host = Arc::new(Mutex::new(SampleHost));
				let path_str = entry.path();
				let path_str = path_str.as_os_str().to_str().unwrap();
				let path = Path::new(&path_str);

				println!("\nLoading {:?}", path);
				let loader = PluginLoader::load(path, host.clone());
				if let Err(err) = loader {
					println!("{err:?} - {path:?}");
					return None;
				}

				let mut instance = loader.unwrap().instance().unwrap();
				instance.stop_process();
				instance.suspend();

				Some((instance.get_info().name, path_str.to_owned()))
			})
			.collect();
	}
}

fn get_files_recursively(path: &str) -> Vec<std::fs::DirEntry> {
	let mut dlls: Vec<std::fs::DirEntry> = vec![];
	let paths = std::fs::read_dir(path).unwrap();
	for path in paths {
		if path.is_err() {
			continue;
		}

		let path = path.unwrap();
		let full_path = path.path();
		let full_path = full_path.as_os_str().to_str().unwrap();
		let is_dir = path.metadata().unwrap().is_dir();

		if is_dir {
			let mut more_paths = get_files_recursively(&full_path);
			dlls.append(&mut more_paths);
			continue;
		}

		if full_path.ends_with(".dll") {
			dlls.push(path);
		}
	}

	dlls
}
