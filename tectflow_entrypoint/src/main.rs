use log::info;

pub struct AppVersion {
	major: u32,
	minor: u32,
	patch: u32,
}

pub const APP_NAME: &'static str = "Tectflow Tectonic Plate Simulator";
pub const APP_VERSION: AppVersion = AppVersion {
	major: 0,
	minor: 1,
	patch: 0,
};

fn init_logging() {
	let mut builder = env_logger::Builder::new();

	builder.filter(None, log::LevelFilter::Info);
	builder.init();
}

fn main() {
	init_logging();
	info!("Initializing tectflow");

	tectflow_frontend::tectflow_window();
}
