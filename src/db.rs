use crate::models::{Chat, ChatWithParticipant, Message, User};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct DBClient {
    pub pool: Pool<Postgres>,
}

impl DBClient {
    pub fn new(pool: Pool<Postgres>) -> Self {
        DBClient { pool }
    }
}

#[async_trait]
pub trait UserExt {
    async fn get_user(
        &self,
        id: Option<Uuid>,
        name: Option<&str>,
        email: Option<&str>,
    ) -> Result<Option<User>, sqlx::Error>;

    async fn get_users(&self, page: u32, limit: u32) -> Result<Vec<User>, sqlx::Error>;

    async fn save_user<T: Into<String> + Send>(
        &self,
        username: T,
        name: T,
        email: T,
        password: T,
    ) -> Result<User, sqlx::Error>;

    async fn update_user_name<T: Into<String> + Send>(
        &self,
        user_id: Uuid,
        name: T,
    ) -> Result<User, sqlx::Error>;

    async fn search_users_by_username(
        &self,
        username: &str,
        exclude_id: Uuid,
    ) -> Result<Vec<User>, sqlx::Error>;
}

#[async_trait]
pub trait ChatExt {
    async fn get_chat(&self, chat_id: Uuid) -> Result<Option<Chat>, sqlx::Error>;

    async fn get_user_chats(&self, user_id: Uuid) -> Result<Vec<Chat>, sqlx::Error>;

    async fn create_chat(&self, user1_id: Uuid, user2_id: Uuid) -> Result<Chat, sqlx::Error>;

    async fn delete_chat(&self, chat_id: Uuid) -> Result<(), sqlx::Error>;

    async fn get_user_chats_with_participants(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<ChatWithParticipant>, sqlx::Error>;
}

#[async_trait]
pub trait MessageExt {
    async fn create_message(
        &self,
        chat_id: Uuid,
        sender_id: Uuid,
        content: &str,
    ) -> Result<Message, sqlx::Error>;

    async fn get_chat_messages(
        &self,
        chat_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Message>, sqlx::Error>;

    async fn edit_message(&self, message_id: Uuid, new_content: &str) -> Result<Message, sqlx::Error>;

    async fn delete_message(&self, message_id: Uuid) -> Result<(), sqlx::Error>;

}

#[async_trait]
impl UserExt for DBClient {
    async fn get_user(
        &self,
        user_id: Option<Uuid>,
        name: Option<&str>,
        email: Option<&str>,
    ) -> Result<Option<User>, sqlx::Error> {
        let mut user: Option<User> = None;

        if let Some(user_id) = user_id {
            user = sqlx::query_as!(User, r#"SELECT id, name, username, email, password, created_at, updated_at FROM users WHERE id = $1 LIMIT 1"#, user_id)
                .fetch_optional(&self.pool)
                .await?
        } else if let Some(name) = name {
            user = sqlx::query_as!(User, r#"SELECT id, name, username, email, password, created_at, updated_at FROM users WHERE name = $1 LIMIT 1"#, name)
                .fetch_optional(&self.pool)
                .await?
        } else if let Some(email) = email {
            user = sqlx::query_as!(User, r#"SELECT id, name, username, email, password, created_at, updated_at FROM users WHERE email = $1 LIMIT 1"#, email)
                .fetch_optional(&self.pool)
                .await?
        }

        Ok(user)
    }

    async fn save_user<T: Into<String> + Send>(
        &self,
        username: T,
        name: T,
        email: T,
        password: T,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (username, name, email, password)
             VALUES ($1, $2, $3, $4)
             RETURNING id, name, username, email, password, created_at, updated_at",
            username.into(),
            name.into(),
            email.into(),
            password.into()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get_users(&self, page: u32, limit: u32) -> Result<Vec<User>, sqlx::Error> {
        let offset = (page - 1) * limit;

        let users = sqlx::query_as!(
            User,
            r#"
            SELECT
                id,
                name,
                username,
                email,
                password,
                created_at,
                updated_at
            FROM users
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    async fn update_user_name<T: Into<String> + Send>(
        &self,
        user_id: Uuid,
        name: T,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "UPDATE users 
SET name = $1, updated_at = NOW()
WHERE id = $2
RETURNING id, name, username, email, password, created_at, updated_at",
            name.into(),
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn search_users_by_username(&self, username: &str, exclude_id: Uuid) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        "SELECT id, name, username, email, password, created_at, updated_at 
         FROM users WHERE username ILIKE $1 AND id != $2 LIMIT 10",
        format!("%{}%", username),
        exclude_id
    )
    .fetch_all(&self.pool)
    .await
}
}

#[async_trait]
impl ChatExt for DBClient {
    async fn get_chat(&self, chat_id: Uuid) -> Result<Option<Chat>, sqlx::Error> {
        let chat = sqlx::query_as!(Chat, "SELECT * FROM chats WHERE id = $1 LIMIT 1", chat_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(chat)
    }

    async fn get_user_chats(&self, user_id: Uuid) -> Result<Vec<Chat>, sqlx::Error> {
        let chats = sqlx::query_as!(
            Chat,
            "SELECT * FROM chats WHERE user1_id = $1 OR user2_id = $1 ORDER BY created_at DESC",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(chats)
    }

    async fn create_chat(&self, user1_id: Uuid, user2_id: Uuid) -> Result<Chat, sqlx::Error> {
        let chat = sqlx::query_as!(
            Chat,
            "INSERT INTO chats (user1_id, user2_id) VALUES ($1, $2) RETURNING id, user1_id, user2_id, created_at",
            user1_id,
            user2_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(chat)
    }

    async fn delete_chat(&self, chat_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM chats WHERE id = $1", chat_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_user_chats_with_participants(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<ChatWithParticipant>, sqlx::Error> {
        let chats = sqlx::query_as!(
            ChatWithParticipant,
            r#"
        SELECT 
            c.id,
            c.user1_id,
            c.user2_id,
            c.created_at,
            u.id as participant_id,
            u.name as participant_name,
            u.username as participant_username
        FROM chats c
        JOIN users u ON u.id = CASE 
            WHEN c.user1_id = $1 THEN c.user2_id
            ELSE c.user1_id
        END
        WHERE c.user1_id = $1 OR c.user2_id = $1
        ORDER BY c.created_at DESC
        "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(chats)
    }
}


#[async_trait]
impl MessageExt for DBClient {
    async fn create_message(
        &self,
        chat_id: Uuid,
        sender_id: Uuid,
        content: &str,
    ) -> Result<Message, sqlx::Error> {
        let msg = sqlx::query_as!(
            Message,
            r#"
            INSERT INTO messages (chat_id, sender_id, content)
            VALUES ($1, $2, $3)
            RETURNING id, chat_id, sender_id, content, created_at
            "#,
            chat_id,
            sender_id,
            content
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(msg)
    }

    async fn get_chat_messages(
        &self,
        chat_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Message>, sqlx::Error> {
        let msgs = sqlx::query_as!(
            Message,
            r#"
            SELECT id, chat_id, sender_id, content, created_at
            FROM messages
            WHERE chat_id = $1
            ORDER BY created_at ASC
            LIMIT $2 OFFSET $3
            "#,
            chat_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(msgs)
    }

    async fn edit_message(&self, message_id: Uuid, new_content: &str) -> Result<Message, sqlx::Error> {
        let msg = sqlx::query_as!(
            Message,
            r#"
            UPDATE messages
            SET content = $1
            WHERE id = $2
            RETURNING id, chat_id, sender_id, content, created_at
            "#,
            new_content,
            message_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(msg)
    }

    async fn delete_message(&self, message_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM messages WHERE id = $1", message_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

}

