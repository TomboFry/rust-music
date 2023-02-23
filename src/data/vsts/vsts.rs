use crate::resources::strings;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use vst::host::{Host, PluginInstance, PluginLoadError, PluginLoader};
use vst::plugin::Plugin;

pub struct SampleHost;

impl Host for SampleHost {
	fn automate(&self, index: i32, value: f32) {
		println!("Parameter {index} changed to {value}");
	}
}

pub struct VstBasicInfo {
	pub name: String,
	pub path: String,
	pub category: vst::plugin::Category,
}

#[derive(Default)]
pub struct VstSettings {
	pub vst_list: Vec<VstBasicInfo>,
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
				let path_str = entry.path();
				let path_str = path_str.as_os_str().to_str().unwrap();

				let instance = VstSettings::load_vst(&entry.path());
				if instance.is_err() {
					return None;
				}
				let mut instance = instance.unwrap();

				instance.stop_process();
				instance.suspend();
				let info = instance.get_info();

				println!("{:?}", instance.get_info().category);

				Some(VstBasicInfo {
					name: info.name,
					path: path_str.to_string(),
					category: info.category,
				})
			})
			.collect();
	}

	pub fn load_vst(entry: &PathBuf) -> Result<PluginInstance, PluginLoadError> {
		// Give every plugin its own host?
		let host = Arc::new(Mutex::new(SampleHost));
		let path = Path::new(&entry);

		println!("\nLoading {:?}", path);
		let loader = PluginLoader::load(path, host.clone());
		if let Err(err) = loader {
			println!("{err:?} - {path:?}");
			return Err(err);
		}

		loader.unwrap().instance()
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
