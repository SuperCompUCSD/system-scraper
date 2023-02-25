use sysinfo::{ComponentExt, CpuExt, System, SystemExt};

/// Returns the current total CPU usage.
pub fn cpu_usage(sys: &System) {
    println!("Our overall cpu usage is {}%",sys.global_cpu_info().cpu_usage());

    //return sys.global_cpu_info().cpu_usage();
}

/// Returns the load average in the format reported by `uptime` (1 min, 5 min, 15 min)
pub fn load_average() -> (f32, f32, f32) {
	todo!()
}

/// Returns the CPU package temperature.
pub fn cpu_temperature(sys: &System) {
	for com in sys.components() {
		let temp = com.temperature();
		println!("{}, {}:", com.label(), temp);
	}
}

/// Returns the command line of top CPU heavy processes.
pub fn top_processes() -> Vec<(Vec<String>, f32)> {
	todo!()
}
