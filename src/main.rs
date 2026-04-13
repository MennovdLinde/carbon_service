use actix_web::{web, App, HttpServer, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CarbonRequest {
    bytes: u64,
}

#[derive(Serialize)]
struct CarbonResponse {
    bytes: u64,
    carbon_grams: f64,
}

async fn calculate_carbon(req: web::Json<CarbonRequest>) -> HttpResponse {
    // Convert bytes to megabytes
    let mb = req.bytes as f64 / 1_000_000.0;
    
    // ~2g CO2 per MB (industry standard)
    let carbon_grams = mb * 2.0;
    
    HttpResponse::Ok().json(CarbonResponse {
        bytes: req.bytes,
        carbon_grams: carbon_grams.round(),
    })
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8001".to_string());
    let bind_addr = format!("0.0.0.0:{}", port);
    println!("🦀 Carbon service starting on {}", bind_addr);

    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health))
            .route("/calculate", web::post().to(calculate_carbon))
    })
    .bind(&bind_addr)?
    .run()
    .await
}