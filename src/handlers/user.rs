use std::sync::Arc;

use axum::extract::State;
use axum::{
    Extension, Json, Router,
    extract::Query,
    middleware,
    response::IntoResponse,
    routing::{get, put},
};
use validator::Validate;
use std::collections::HashMap;

use crate::{
    AppState,
    db::UserExt,
    dtos::{
        FilterUserDto, RequestQueryDto, Response, UserData, UserListResponseDto, UserResponseDto,
    },
    error::{ErrorMessage, HttpError},
    middleware::{JWTAuthMiddleware, role_check},
    utils::password,
};

pub fn users_handler() -> Router<Arc<AppState>> {
    Router::new().route("/me", get(get_me))
    .route("/users/search", get(search_users))
}

pub async fn get_me(
    State(app_state): State<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
    let filtered_user = FilterUserDto::filter_user(&user.user);

    let response = UserResponseDto {
        status: "success".to_string(),
        user: filtered_user,
    };

    Ok(Json(response))
}

pub async fn get_users(
    Query(query_params): Query<RequestQueryDto>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    query_params
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);

    let users = app_state
        .db_client
        .get_users(page as u32, limit as u32)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(users))
}

pub async fn search_users(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddleware>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, HttpError> {
    let username = query.get("username").cloned().unwrap_or_default();
    let users = state
        .db_client
        .search_users_by_username(&username, user.user.id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;
    Ok(Json(users))
}
