use std::{thread, time::Duration, sync::atomic::AtomicU64, net::SocketAddr};

use axum::{Router, routing::get};
use prometheus_client::{registry::Registry, metrics::{family::Family, gauge::Gauge}, encoding::{EncodeLabelSet, text::encode}};
use sysinfo::{System, SystemExt};
use system_scraper::{cpu_temperature, top_processes, ProcessInfo};

// fn main() {
// 	let mut sys = System::new_all();

// 	let mut processes: Vec<ProcessInfo>;

// 	loop {
// 		sys.refresh_all();

// 		// dbg!(cpu_usage());
// 		dbg!(sys.load_average());
// 		cpu_temperature(&sys);
// 		processes = top_processes(&sys, 5);

// 		for process in processes.iter() {
// 			println!(
// 				"Process {} ({}): {}",
// 				process.name, process.pid, process.cpu_usage
// 			);
// 		}
// 		println!();

// 		thread::sleep(Duration::from_millis(2000));
// 	}
// }

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/metrics", get(get_metrics));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
struct CPUTempLabels {
    core: String,
}

async fn get_metrics() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();
    let mut registry = Registry::default();
    let cpu_temperatures = Family::<CPUTempLabels, Gauge<f64, AtomicU64>>::default();
    let temps = cpu_temperature(&sys);
    for (label, temp) in temps {
        cpu_temperatures.get_or_create(&CPUTempLabels { core: label.to_string() }).set(temp.into());
    }
    registry.register("cpu_temperature", "The temperature for each CPU core", cpu_temperatures.clone());
    let mut buffer = String::new();
    encode(&mut buffer, &registry).expect("Should not have died here!");
    buffer
}