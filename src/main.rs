use std::{sync::atomic::AtomicU64, net::SocketAddr};

use axum::{Router, routing::get};
use prometheus_client::{registry::Registry, metrics::{family::Family, gauge::Gauge}, encoding::{EncodeLabelSet, text::encode}};
use sysinfo::{System, SystemExt};
use tower::{ServiceBuilder};
use tower_http::trace::{TraceLayer, self};
use system_scraper::{cpu_temperature};
use tracing::Level;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
    .with_target(false)
    .with_max_level(Level::INFO)
    .compact()
    .init();
    // build our application with a route
    let app = Router::new()
        .route("/metrics", get(get_metrics))
        .layer(ServiceBuilder::new()
        .layer(TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
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
        cpu_temperatures.get_or_create(&CPUTempLabels { core: label.to_string() }).set(temp.into());
    }
    registry.register("cpu_temperature", "The temperature for each CPU core", cpu_temperatures);
    let mut buffer = String::new();
    encode(&mut buffer, &registry).expect("Should not have died here!");
    buffer
}