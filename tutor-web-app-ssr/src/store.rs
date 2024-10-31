use crate::errors::EzyTutorError;
use crate::model::*;

use sqlx::postgres::PgPool;

pub async fn get_user_record(pg_pool: &PgPool, username: &str) -> Result<User, EzyTutorError> {
    sqlx::query_as!(
        User,
        "SELECT * FROM ezyweb_user WHERE username = $1",
        username,
    )
    .fetch_optional(pg_pool)
    .await?
    .ok_or_else(|| EzyTutorError::NotFound("User name not found".to_string()))
}

pub async fn post_new_user(pg_pool: &PgPool, new_user: User) -> Result<User, EzyTutorError> {
    let User {
        username,
        user_password,
        tutor_id,
    } = new_user;
    let user = sqlx::query_as!(
        User,
        "INSERT INTO ezyweb_user(
        username, user_password, tutor_id)
        VALUES ($1, $2, $3)
        RETURNING username, user_password, tutor_id",
        username,
        user_password,
        tutor_id,
    )
    .fetch_one(pg_pool)
    .await?;

    Ok(user)
}
