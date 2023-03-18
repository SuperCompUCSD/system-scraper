use std::{cmp, collections::HashMap};
use sysinfo::{ComponentExt, PidExt, ProcessExt, System, SystemExt};

/// Structure containing
///
/// `pid: u32`
///
/// `name: String`
///
/// `cpu_usage: f32`
pub struct ProcessInfo {
	pub pid: u32,
	pub name: String,
	pub cpu_usage: f32,
}

/// Returns the current total CPU usage.
pub fn cpu_usage() -> f32 {
	todo!()
}

/// Returns the CPU package temperature.
pub fn cpu_temperature(sys: &System) -> Vec<(&str, f32)> {
	sys.components()
		.iter()
		.map(|component| (component.label(), component.temperature()))
		.collect()
}

/// Returns a vector of `ProcessStruct` containing the information of no more than n processes.
pub fn top_processes(sys: &System, n: u8) -> Vec<ProcessInfo> {
	let mut processes: Vec<ProcessInfo> = Vec::new();

	for (pid, process) in sys.processes().iter() {
		processes.push(ProcessInfo {
			pid: pid.as_u32(),
			name: process.name().to_string(),
			cpu_usage: process.cpu_usage(),
		});
	}

	processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
	processes.truncate(cmp::min(n.into(), processes.len()));
	processes
}
