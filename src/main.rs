use std::{net::SocketAddr, sync::atomic::AtomicU64};

use axum::{routing::get, Router};
use prometheus_client::{
	encoding::{text::encode, EncodeLabelSet},
	metrics::{family::Family, gauge::Gauge},
	registry::Registry,
};
use sysinfo::{System, SystemExt};
use system_scraper::cpu_temperature;
use tower::ServiceBuilder;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

#[tokio::main]
async fn main() {
    // Add HTTP tracing to the Info level for HTTP requests.
	tracing_subscriber::fmt()
		.with_target(false)
		.with_max_level(Level::INFO)
		.compact()
		.init();

	let app = Router::new().route("/metrics", get(get_metrics)).layer(
		ServiceBuilder::new().layer(
			TraceLayer::new_for_http()
				.make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
				.on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
		),
	);

	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	tracing::info!("Listening on {addr}");
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
		cpu_temperatures
			.get_or_create(&CPUTempLabels {
				core: label.to_string(),
			})
			.set(temp.into());
	}
	registry.register(
		"cpu_temperature",
		"The temperature for each CPU core",
		cpu_temperatures,
	);
	let mut buffer = String::new();
	encode(&mut buffer, &registry).expect("Should not have died here!");
	buffer
}
