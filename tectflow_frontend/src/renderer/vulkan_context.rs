use std::{error::Error, ffi::CStr, ffi::CString};

use ash::{
	extensions::{
		ext::DebugUtils,
		khr::{Surface, Swapchain as SwapchainLoader},
	},
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
			ash_window::create_surface(&entry, &instance, window, None)?
		};

		debug!("Creating Vulkan physical device");

		let devices = unsafe { instance.enumerate_physical_devices()? };

		let mut graphics = None;
		let mut present = None;
		let physical_device = {
			devices.into_iter().find(|device| {
				let device = *device;

				let properties = unsafe {
					instance.get_physical_device_queue_family_properties(device)
				};

				for (index, family) in
					properties.iter().filter(|f| f.queue_count > 0).enumerate()
				{
					let index = index as u32;

					graphics = None;
					present = None;

					if family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
						&& family.queue_flags.contains(vk::QueueFlags::COMPUTE)
					{
						graphics = Some(index);
					}

					let present_support = unsafe {
						surface.get_physical_device_surface_support(
							device,
							index,
							surface_khr,
						).expect("Could not get surface support for physical device")
					};
					if present_support {
						present = Some(index);
					}

					if graphics.is_some() && present.is_some() {
						break;
					}
				}

				let extension_properties = unsafe {
					instance
						.enumerate_device_extension_properties(device)
						.expect("Failed to get device extension properties")
				};

                let extension_support = extension_properties.iter().any(|ext| {
                    let name = unsafe {
                        CStr::from_ptr(ext.extension_name.as_ptr())
                    };

                    SwapchainLoader::name() == name
                });

                let formats = unsafe {
                    surface
                        .get_physical_device_surface_formats(device, surface_khr)
                        .expect("Failed to get physical device surface formats")
                };

                let present_modes = unsafe {
                    surface.get_physical_device_surface_present_modes(device, surface_khr)
                        .expect("Failed to get physical device surface present modes")
                };

                graphics.is_some()
                && present.is_some()
                && extension_support
                && !formats.is_empty()
                && ! present_modes.is_empty()
			}).expect("Could not find a compatible device")
		};
		let graphics_q_index = graphics.unwrap();
		let present_q_index = present.unwrap();
		std::mem::drop((graphics, present));

		unsafe {
			let properties =
				instance.get_physical_device_properties(physical_device);
			let device_name = CStr::from_ptr(properties.device_name.as_ptr());
			debug!("Selected physical device: {device_name:?}");
		}


		debug!("Creating vulkan device and queues");

		let queue_priorities = [1.0f32];
		let queue_create_infos = {
			let mut indices = vec![graphics_q_index, present_q_index];
			indices.dedup();

			indices
				.iter()
				.map(|index| {
					vk::DeviceQueueCreateInfo::builder()
						.queue_family_index(*index)
						.queue_priorities(&queue_priorities)
						.build()
				})
				.collect::<Vec<_>>()
		};

		let device_extension_ptrs = [SwapchainLoader::name().as_ptr()];

		let device_create_info = vk::DeviceCreateInfo::builder()
			.queue_create_infos(&queue_create_infos)
			.enabled_extension_names(&device_extension_ptrs);

		let device = unsafe {
			instance.create_device(
				physical_device,
				&device_create_info,
				None,
			)?
		};

		let graphics_queue =
			unsafe { device.get_device_queue(graphics_q_index, 0) };
		let present_queue =
			unsafe { device.get_device_queue(present_q_index, 0) };


		let command_pool = {
			let command_pool_info = vk::CommandPoolCreateInfo::builder()
				.queue_family_index(graphics.unwrap())
				.flags(vk::CommandPoolCreateFlags::empty());
			unsafe { device.create_command_pool(&command_pool_info, None)? }
		};

		Ok(Self {
			_entry: entry,
			instance,
			debug_utils,
			debug_utils_messenger,
			surface,
			surface_khr,
			physical_device,
			graphics_q_index,
			present_q_index,
			device,
			graphics_queue,
			present_queue,
			command_pool,
		})
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
		.application_version(vk::make_api_version(
			0, app_major, app_minor, app_patch,
		))
		.engine_name(engine_name.as_c_str())
		.engine_version(vk::make_api_version(
			0,
			engine_major,
			engine_minor,
			engine_patch,
		))
		.api_version(vk::make_api_version(0, 1, 0, 0));

	unimplemented!(); //TODO: unimplemented
}
