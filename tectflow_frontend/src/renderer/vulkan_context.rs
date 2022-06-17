use std::{error::Error, ffi::CString};

use ash::{
	extensions::{ext::DebugUtils, khr::Surface},
	vk::{
		self, ApplicationInfo, CommandPool, DebugUtilsMessengerEXT,
		PhysicalDevice, SurfaceKHR,
	},
	Device, Entry, Instance,
};

use log::*;

use winit::window::Window;

use crate::{
	FrontendAppVersion, TECTFLOW_FRONTEND_ENGINE_NAME,
	TECTFLOW_FRONTEND_ENGINE_VERSION,
};

pub(super) struct VulkanContext {
	_entry: Entry,
	pub instance: Instance,
	debug_utils: DebugUtils,
	debug_utils_messenger: DebugUtilsMessengerEXT,
	surface: Surface,
	surface_khr: SurfaceKHR,
	pub physical_device: PhysicalDevice,

	graphics_q_index: u32,
	present_q_index: u32,

	pub device: Device,
	pub graphics_queue: vk::Queue,
	present_queue: vk::Queue,
	pub command_pool: CommandPool,
}

impl VulkanContext {
	pub fn new(
		window: &Window,
		app_name: &str,
		app_version: FrontendAppVersion,
	) -> Result<Self, Box<dyn Error>> {
		let entry = Entry::linked();
		let tectflow_vk_instance =
			create_vulkan_instance(&entry, window, app_name, app_version)?;
		let (instance, debug_utils, debug_utils_messenger) = (
			tectflow_vk_instance.instance,
			tectflow_vk_instance.debug_utils,
			tectflow_vk_instance.debug_utils_messenger,
		);

		let surface = Surface::new(&entry, &instance);
		let surface_khr = unsafe {
			//    ash_window
		};
		unimplemented!(); //TODO: unimplemented
	}
}


struct TectflowVkInstance {
	instance: Instance,
	debug_utils: DebugUtils,
	debug_utils_messenger: DebugUtilsMessengerEXT,
}

fn create_vulkan_instance(
	entry: &Entry,
	window: &Window,
	app_name: &str,
	app_version: FrontendAppVersion,
) -> Result<TectflowVkInstance, Box<dyn Error>> {
	debug!("Creating a vulkan instance");

	let app_name = CString::new(app_name)?;
	let app_major = app_version.major;
	let app_minor = app_version.minor;
	let app_patch = app_version.patch;
	let engine_name = CString::new(TECTFLOW_FRONTEND_ENGINE_NAME)?;
	let engine_major = TECTFLOW_FRONTEND_ENGINE_VERSION.major;
	let engine_minor = TECTFLOW_FRONTEND_ENGINE_VERSION.minor;
	let engine_patch = TECTFLOW_FRONTEND_ENGINE_VERSION.patch;
	let app_info = ApplicationInfo::builder()
		.application_name(app_name.as_c_str())
		.application_version(vk::make_version(app_major, app_minor, app_patch))
		.engine_name(engine_name.as_c_str())
		.engine_version(vk::make_version(
			engine_major,
			engine_minor,
			engine_patch,
		))
		.api_version(vk::make_api_version(0, 1, 0, 0));

	unimplemented!(); //TODO: unimplemented
}
