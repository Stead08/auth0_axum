use crate::models::users;
use crate::models::users::ActiveModel;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryTrait};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::default::Default;

pub async fn get_users(State(state): State<AppState>) -> impl IntoResponse {
    let query = users::Entity::find().all(&state.postgres).await;

    match query {
        Ok(users) => {
            let json_users = json!(users);
            Json(json_users).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn get_user_by_id(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    let Ok(user_id) = user_id.parse::<i32>() else {
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response() };
    let query = users::Entity::find_by_id(user_id)
        .one(&state.postgres)
        .await;

    match query {
        Ok(user) => {
            if let Some(user) = user {
                Json(user).into_response()
            } else {
                StatusCode::NOT_FOUND.into_response()
            }
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
pub async fn add_user(
    State(state): State<AppState>,
    Json(user): Json<RegisterUser>,
) -> impl IntoResponse {
    let new_user = ActiveModel {
        id: Default::default(),
        first_name: Set(user.first_name),
        last_name: Set(user.last_name),
        email: Set(user.email),
        created_at: Default::default(),
    };
    let query = new_user.insert(&state.postgres).await;

    match query {
        Ok(res) => (StatusCode::CREATED, format!("user created {:?}", res)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    let Ok(user_id) = user_id.parse::<i32>() else {
        return (StatusCode::NOT_FOUND).into_response() };

    let query = users::Entity::delete_by_id(user_id)
        .exec(&state.postgres)
        .await;

    match query {
        Ok(_) => (StatusCode::OK, "deleted".to_string()).into_response(),
        Err(err) => {
            eprintln!("{}", err.to_string());
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}
