use std::sync::Arc;

use argon2::PasswordHash;
use axum::extract::State;
use axum::{
    Extension, Json, Router,
    http::{HeaderMap, StatusCode, header},
    response::IntoResponse,
    routing::post,
};
use axum_extra::extract::cookie::Cookie;
use validator::Validate;

use crate::{
    AppState,
    db::UserExt,
    dtos::{LoginUserDto, RegisterUserDto, Response, UserLoginResponseDto},
    error::{ErrorMessage, HttpError},
    utils::{password, token},
};

pub fn auth_handler() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
}

pub async fn register(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<RegisterUserDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(format!("Validation error: {}", e)))?;

    let hashed_password = password::hash_password(&body.password)
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let result = app_state
        .db_client
        .save_user(&body.username, &body.name, &body.email, &hashed_password)
        .await;

    match result {
        Ok(_user) => Ok((
            StatusCode::CREATED,
            Json(Response {
                status: "success",
                message: "Registration successful!".to_string(),
            }),
        )),

        Err(sqlx::Error::Database(db_err)) => {
            Err(HttpError::bad_request(db_err.message().to_string()))
        }

        Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}

pub async fn login(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<LoginUserDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(format!("Validation error: {}", e)))?;

    let result = app_state
        .db_client
        .get_user(None, None, Some(&body.email))
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let user = result.ok_or(HttpError::unauthorized(
        ErrorMessage::WrongCredentials.to_string(),
    ))?;

    let password_matched = password::compare_password(&user.password, &body.password)
        .map_err(|_| HttpError::bad_request(ErrorMessage::WrongCredentials.to_string()))?;

    if password_matched {
        let token = token::create_token(
            &user.id.to_string(),
            &app_state.env.jwt_secret.as_bytes(),
            app_state.env.jwt_maxage,
        )
        .map_err(|e| HttpError::server_error(e.to_string()))?;

        let cookie_duration = time::Duration::minutes(app_state.env.jwt_maxage * 60);
        let cookie = Cookie::build(("access_token", token.clone()))
            .path("/")
            .http_only(true)
            .secure(true)
            .same_site(axum_extra::extract::cookie::SameSite::None)
            .max_age(cookie_duration)
            .build();

        let response = axum::response::Json(UserLoginResponseDto {
            status: "success".to_string(),
            token,
        });

        let mut headers = HeaderMap::new();

        let cookie_val = cookie.to_string().parse()
            .map_err(|e| HttpError::server_error(format!("Failed to parse cookie: {}", e)))?;
        headers.append(header::SET_COOKIE, cookie_val);

        let mut response = response.into_response();
        response.headers_mut().extend(headers);

        Ok(response)
    } else {
        Err(HttpError::bad_request(
            ErrorMessage::WrongCredentials.to_string(),
        ))
    }
}

pub async fn logout(State(app_state): State<Arc<AppState>>) -> Result<impl IntoResponse, HttpError> {
    let cookie = axum::http::HeaderValue::from_str(
        "access_token=; Path=/; HttpOnly; Max-Age=0; SameSite=None; Secure",
    )
    .map_err(|e| HttpError::server_error(e.to_string()))?;

    let mut headers = HeaderMap::new();
    headers.insert(header::SET_COOKIE, cookie);

    Ok((
        headers,
        Json(Response {
            status: "success",
            message: "Logged out successfully".to_string(),
        }),
    ))
}
