use system_scraper::{cpu_temperature, cpu_usage, load_average, top_processes};

fn main() {
	dbg!(cpu_usage());
	dbg!(load_average());
	dbg!(cpu_temperature());
	dbg!(top_processes());
}
