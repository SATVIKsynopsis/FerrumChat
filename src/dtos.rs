use crate::models::User;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct RegisterUserDto {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,

    #[validate(length(min = 3, message = "Username must be at least 3 characters long"))]
    pub username: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters long"))]
    pub password: String,

    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    pub password_confirm: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct LoginUserDto {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters long"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponseDto {
    pub status: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct FilterUserDto {
    pub id: Uuid,
    pub name: String,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FilterUserDto {
    pub fn filter_user(user: &User) -> FilterUserDto {
        FilterUserDto {
            id: user.id,
            name: user.name.clone(),
            username: user.username.clone(),
            email: user.email.clone(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct UserData {
    pub user: FilterUserDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDto {
    pub status: String,
    pub user: FilterUserDto,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct UserLoginResponseDto {
    pub status: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserListResponseDto {
    pub status: String,
    pub users: Vec<FilterUserDto>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct RequestQueryDto {
    #[validate(range(min = 1))]
    pub page: Option<usize>,
    #[validate(range(min = 1, max = 50))]
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatDto {
    pub id: Uuid,
    pub chat_id: Uuid,
    pub content: String,
    pub receiver_id: Uuid,
    pub sender_id: Uuid,
    pub user1_id: Uuid,
    pub user2_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub r#type: Option<String>,
    pub sent: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateChatDto {
    pub receiver_id: Uuid,
}

#[derive(Serialize)]
pub struct WsMessageOut {
    pub sender_id: Uuid,
    pub content: String,
    pub chat_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatListResponseDto {
    pub status: String,
    pub chats: Vec<ChatDto>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageDto {
    pub id: Uuid,
    pub chat_id: Uuid,
    pub sender_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct EditMessageDto {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct SendMessageDto {
    pub chat_id: Uuid,

    #[validate(length(min = 1, message = "Message cannot be empty"))]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageListResponseDto {
    pub status: String,
    pub messages: Vec<MessageDto>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WsEvent {
    SendMessage(SendMessageDto),
    Typing { chat_id: Uuid },
    StopTyping { chat_id: Uuid },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WsMessageBroadcast {
    pub chat_id: Uuid,
    pub message: MessageDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}
