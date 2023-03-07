use sysinfo::{System, SystemExt};
use system_scraper::{cpu_temperature, top_processes};

fn main() {
	let mut sys = System::new_all();
	let mut processes: Vec<(u32, String, f32)>;

	loop {
		sys.refresh_all();

		//dbg!(cpu_usage());
		//dbg!(load_average());
		cpu_temperature(&sys);
		processes = top_processes(&sys, 5);

		for (pid, name, cpu_usage) in processes {
			println!("{} {} {}", pid, name, cpu_usage);
		}
		println!();

		std::thread::sleep(std::time::Duration::from_millis(1000));
	}
}
