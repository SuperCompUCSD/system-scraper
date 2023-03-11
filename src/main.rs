use std::{thread, time::Duration};

use sysinfo::{System, SystemExt};
use system_scraper::{cpu_temperature, top_processes, ProcessInfo};

fn main() {
	let mut sys = System::new_all();

	let mut processes: Vec<ProcessInfo>;

	loop {
		sys.refresh_all();

		// dbg!(cpu_usage());
		dbg!(sys.load_average());
		cpu_temperature(&sys);
		processes = top_processes(&sys, 5);

		for process in processes.iter() {
			println!(
				"Process {} ({}): {}",
				process.name, process.pid, process.cpu_usage
			);
		}
		println!();

		thread::sleep(Duration::from_millis(2000));
	}
}
