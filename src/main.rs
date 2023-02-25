use sysinfo::{System, SystemExt};
use system_scraper::{cpu_temperature, cpu_usage, load_average, top_processes};

fn main() {
	let mut sys = System::new_all();

	loop {
		sys.refresh_all();
		//dbg!(cpu_usage());
		//dbg!(load_average());
		//dbg!(top_processes());
		cpu_temperature(&sys);
		std::thread::sleep(std::time::Duration::from_millis(500));
	}
}
