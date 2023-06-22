use crate::router::create_router;
use anyhow::Error;
use hyper::body::to_bytes;
use hyper::Client;
use hyper_tls::HttpsConnector;
use jsonwebtoken::jwk::JwkSet;
use sea_orm::{DatabaseConnection, SqlxPostgresConnector};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::SocketAddr;
use std::time::Duration;
use tracing_subscriber::fmt;

mod handlers;
mod models;
mod router;

#[derive(Clone)]
pub struct AppState {
    postgres: DatabaseConnection,
    pgpool: PgPool,
    authority: String,
    jwks: JwkSet,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    fmt::init();
    let db_address = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let postgres = PgPoolOptions::new()
        .max_connections(5)
        .idle_timeout(Some(Duration::from_secs(1)))
        .connect(&db_address)
        .await
        .expect("Failed to connect to Postgres!");
    let conn = SqlxPostgresConnector::from_sqlx_postgres_pool(postgres.clone());
    sqlx::migrate!()
        .run(&postgres)
        .await
        .expect("Failed to run migrations!");
    let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");
    let jwks = get_jwks(&authority).await.expect("failed to fetch jwks");
    let state = AppState {
        postgres: conn,
        pgpool: postgres,
        authority,
        jwks,
    };

    let app = create_router(state);

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a number!");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Listening on Port: {}", port);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_jwks(authority: &str) -> anyhow::Result<JwkSet, Error> {
    //fetch jwks
    let jwks_uri = format!("{}{}", authority, "/.well-known/jwks.json")
        .parse()
        .expect("Invalid uri");

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let response = client.get(jwks_uri).await.expect("failed to fetch jwks");

    let body_bytes = to_bytes(response.into_body()).await?;
    let jwks: JwkSet = serde_json::from_slice(&body_bytes)?;
    Ok(jwks)
}
