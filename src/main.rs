use std::{net::SocketAddr, sync::atomic::AtomicU64};

use axum::{routing::get, Router};
use prometheus_client::{
	encoding::{text::encode, EncodeLabelSet},
	metrics::{family::Family, gauge::Gauge},
	registry::Registry,
};
use sysinfo::{System, SystemExt};
use system_scraper::temperatures;
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

	let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
	tracing::info!("Listening on {addr}");
	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
struct TempLabels {
	core: String,
}

async fn get_metrics() -> String {
	let mut sys = System::new_all();
	sys.refresh_all();
	let mut registry = Registry::default();
	let family = Family::<TempLabels, Gauge<f64, AtomicU64>>::default();
	let temps = temperatures(&sys);
	for (label, temp) in temps {
		family
			.get_or_create(&TempLabels {
				core: label.to_string(),
			})
			.set(temp.into());
	}
	registry.register(
		"temperatures",
		"The temperature for each system component",
		family,
	);
	let mut buffer = String::new();
	encode(&mut buffer, &registry).expect("could not encode registry's metrics");
	buffer
}
