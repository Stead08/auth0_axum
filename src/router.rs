use axum::extract::State;
use axum::http::{HeaderValue, Method, Request, StatusCode};
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{middleware, Router};
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, ORIGIN};

use tower_http::cors::CorsLayer;

use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use sea_orm::ColIdx;

use serde_json::Value;

use crate::handlers::{add_user, delete_user, get_user_by_id, get_users};
use crate::AppState;

pub fn create_router(state: AppState) -> Router {
    //create api route
    let api_route = api_router(state);
    Router::new().nest("/api", api_route)
}

pub fn api_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec![ACCEPT, AUTHORIZATION, ORIGIN, CONTENT_TYPE])
        .allow_origin("http://127.0.0.1:3000".parse::<HeaderValue>().unwrap());

    let user_router = Router::new()
        .route("/", get(get_users))
        .route("/:user_id", get(get_user_by_id))
        .route("/", post(add_user))
        .route("/:user_id", delete(delete_user))
        .with_state(state.clone())
        .route_layer(middleware::from_fn_with_state(state, validate_session));

    Router::new()
        .route("/", get(health_check))
        .nest("/users", user_router)
        .layer(cors)
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK".to_string())
}

pub async fn validate_session<B>(
    State(state): State<AppState>,
    // Request<B> と Next<B> は axum の関数からのミドルウェアに必要な型
    request: Request<B>,
    next: Next<B>,
) -> axum::response::Response {
    let jwks = state.jwks;
    //AUTHORIZATION ヘッダを取得
    let Some(authorization_header) = request.headers().get("AUTHORIZATION") else {
        return (StatusCode::UNAUTHORIZED, "no authorization header".to_string()).into_response() };

    let Ok(authorization) = authorization_header.to_str() else { return StatusCode::UNAUTHORIZED.into_response() };

    // jwt tokenだけ剥がす
    let Some(jwt_token) = authorization.strip_prefix("Bearer ") else {
        return (StatusCode::UNAUTHORIZED, "No Bearer".to_string()).into_response() };
    // tokenをdecodeする
    let Ok(header) = decode_header(jwt_token) else { return (StatusCode::UNAUTHORIZED, "failed to decode header".to_string()).into_response() };
    //kidを取得
    let Some(kid) = header.kid else { return (StatusCode::UNAUTHORIZED, "no valied kid".to_string()).into_response() };
    //kidに対応するjwkを取得
    let Some(jwk) = jwks.find(kid.as_str()) else { return (StatusCode::UNAUTHORIZED, "no valid jwk".to_string()).into_response() };
    // jwkからDecodingKeyを生成
    let decoding_key = DecodingKey::from_jwk(jwk).expect("failed to decode key");
    // RS256を指定
    let validation = Validation::new(Algorithm::RS256);

    match decode::<Value>(jwt_token, &decoding_key, &validation) {
        // JWTのデコードと検証を行う
        Ok(value) => {
            eprintln!("{:?}", value);
            next.run(request).await},
        Err(_) => StatusCode::UNAUTHORIZED.into_response(),
    }
}
