/// Returns the current total CPU usage.
pub fn cpu_usage() -> f32 {
	todo!()
}

/// Returns the load average in the format reported by `uptime` (1 min, 5 min, 15 min)
pub fn load_average() -> (f32, f32, f32) {
	todo!()
}

/// Returns the CPU package temperature.
pub fn cpu_temperature() -> f32 {
	todo!()
}

/// Returns the command line of top CPU heavy processes.
pub fn top_processes() -> Vec<(Vec<String>, f32)> {
	todo!()
}