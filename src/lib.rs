use sysinfo::{ComponentExt, System, SystemExt};

/// Returns the current total CPU usage.
pub fn cpu_usage() -> f32 {
	todo!()
}

/// Returns the load average in the format reported by `uptime` (1 min, 5 min, 15 min)
pub fn load_average() -> (f32, f32, f32) {
	todo!()
}

/// Returns the CPU package temperature.
pub fn cpu_temperature(sys: &System) -> f32 {
	for com in sys.components() {
		let temp = com.temperature();
		println!("{}, {}:", com.label(), temp);
	}

	0.
}

/// Returns the command line of top CPU heavy processes.
pub fn top_processes() -> Vec<(Vec<String>, f32)> {
	todo!()
}
