use std::cmp;
use sysinfo::{ComponentExt, PidExt, ProcessExt, System, SystemExt};

/// Structure containing
///
/// `pid: u32`
///
/// `name: String`
///
/// `cpu_usage: f32`
pub struct ProcessStruct {
	pub pid: u32,
	pub name: String,
	pub cpu_usage: f32,
}

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

/// Returns a vector of `ProcessStruct` containing the information of `min(n, vector.len())`.
pub fn top_processes(sys: &System, n: u8) -> Vec<ProcessStruct> {
	let mut processes: Vec<ProcessStruct> = Vec::new();

	for (pid, process) in sys.processes().iter() {
		processes.push(ProcessStruct {
			pid: pid.as_u32(),
			name: process.name().to_string(),
			cpu_usage: process.cpu_usage(),
		});
	}

	processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
	processes.truncate(cmp::min(n.into(), processes.len()));
	processes
}
