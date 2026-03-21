use std::sync::atomic::AtomicU16;

use axum::{Router, extract::Request};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 3000)]
    port: u16,
}

static PORT: AtomicU16 = AtomicU16::new(0);

#[tokio::main]
async fn main() {
    let args = Args::parse();
    PORT.store(args.port, std::sync::atomic::Ordering::Relaxed);
    println!("Starting server on port: {}", args.port);

    // build our application with a single fallback route
    let app = Router::new().fallback(default_handler);

    // run our app with hyper, listening globally on the specified port
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn default_handler(request: Request) -> String {
    println!("Received request: {:?}", request);
    format!(
        "Server: {}, Request: {:?}",
        PORT.load(std::sync::atomic::Ordering::Relaxed),
        request
    )
}
