use sysinfo::{System, SystemExt};
use system_scraper::{cpu_temperature, cpu_usage};

fn main() {
	let mut sys = System::new_all();
	loop {
	    sys.refresh_all();

		cpu_usage(&sys);
		//dbg!(load_average());
		//dbg!(top_processes());
		cpu_temperature(&sys);

		std::thread::sleep(std::time::Duration::from_millis(500));
	}
}
