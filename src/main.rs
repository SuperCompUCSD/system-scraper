use sysinfo::{System, SystemExt};
use system_scraper::{cpu_temperature, top_processes, ProcessStruct};

fn main() {
	let mut sys = System::new_all();
	let mut processes: Vec<ProcessStruct>;

	loop {
		sys.refresh_all();

		//dbg!(cpu_usage());
		//dbg!(load_average());
		cpu_temperature(&sys);
		processes = top_processes(&sys, 3);

		for process in processes.iter() {
			println!("{} {} {}", process.pid(), process.name(), process.cpu_usage());
		}
		println!();

		std::thread::sleep(std::time::Duration::from_millis(1000));
	}
}
