mod config;
mod db;
mod dtos;
mod error;
mod handlers;
mod middleware;
mod models;
mod router;
mod utils;
mod websocket;

use axum::Router;
use axum::extract::ws::{CloseFrame, Message, WebSocket};
use axum::http::{
    HeaderValue, Method,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, COOKIE},
};
use axum::routing::get;
use config::Config;
use db::{ChatExt, DBClient, MessageExt, UserExt};
use dotenv::dotenv;
use router::create_router;
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
use tracing_subscriber::filter::LevelFilter;
use uuid::Uuid;

#[derive(Debug)]
pub struct AppState {
    pub env: Config,
    pub db_client: DBClient,
    pub active_sessions: Arc<Mutex<HashMap<Uuid, mpsc::UnboundedSender<Message>>>>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            env: self.env.clone(),
            db_client: self.db_client.clone(),
            active_sessions: Arc::clone(&self.active_sessions),
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    dotenv().ok();
    let config = Config::init();
    let port = config.port;

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("Connected to the database successfully.");
            pool
        }
        Err(e) => {
            println!("Failed to connect to the database: {:?}", e);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
            "https://ferri-front.vercel.app"
                .parse::<HeaderValue>()
                .unwrap(),
        ])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE, ACCEPT, COOKIE])
        .allow_credentials(true);

    let db_client = DBClient::new(pool);
    let app_state = Arc::new(AppState {
        env: config,
        db_client,
        active_sessions: Arc::new(Mutex::new(HashMap::new())),
    });

    let app = create_router(app_state.clone()).layer(cors);

    println!(
        "{}",
        format!(" Server is running on http://localhost:{}", &port)
    );

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
