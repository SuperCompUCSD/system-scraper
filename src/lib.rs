use sysinfo::{ComponentExt, System, SystemExt, PidExt, ProcessExt};
use std::cmp;

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
pub fn top_processes(sys: &System, n: u8) -> Vec<(u32, String, f32)> {
	let mut processes: Vec<(u32, String, f32)> = Vec::new();

	for (pid, process) in sys.processes().iter() {
		if process.cpu_usage() > 0.0 {
			processes.push((pid.as_u32(), process.name().to_string(), process.cpu_usage()));
		}
	}

	processes.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
	processes.truncate(cmp::min(n.into(), processes.len()));
	processes
}