use imgui::{Condition, Context};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use winit::{
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::WindowBuilder,
};


pub fn tectflow_window() {
	let event_loop = EventLoop::new();
	let window = WindowBuilder::new().build(&event_loop).unwrap();

	let dpi_mode =
		if let Ok(factor) = std::env::var("IMGUI_EXAMPLE_FORCE_DPI_FACTOR") {
			// Allow forcing of HiDPI factor for debugging purposes
			match factor.parse::<f64>() {
				Ok(f) => HiDpiMode::Locked(f),
				Err(e) => panic!("Invalid scaling factor: {}", e),
			}
		} else {
			HiDpiMode::Default
		};

	let mut imgui = Context::create();

	let platform = WinitPlatform::init(&mut imgui);

	platform.attach_window(imgui.io_mut(), &window, dpi_mode);


	let mut count = 0;

	// Trying to figure it out from:
	// https://github.com/imgui-rs/imgui-rs/blob/main/imgui-examples/examples/support/mod.rs
	// https://docs.rs/imgui-winit-support/0.8.2/imgui_winit_support/

	event_loop.run(move |event, _, control_flow| {
		*control_flow = ControlFlow::Poll;

		match event {
			Event::WindowEvent {
				event: WindowEvent::CloseRequested,
				..
			} => {
				println!("Exiting tectflow frontend");
				*control_flow = ControlFlow::Exit
			}
			Event::MainEventsCleared => {
				//TODO: render a window

				let ui = imgui.frame();


				ui.window("Hey look it's a menu")
					.size([300.0, 110.0], Condition::FirstUseEver)
					.build(|| {
						ui.text_wrapped("i guess here's some text");
						if ui.button(format!("Click me: {}", count)) {
							count += 1;
						}

						let mouse_position = ui.io().mouse_pos;
						ui.text(format!(
							"{}, {}",
							mouse_position[0], mouse_position[1]
						));
					});
			}
			_ => (),
		}
	});
}
