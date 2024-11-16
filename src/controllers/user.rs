use sqlx::{query, query_as, PgPool, Row};
use uuid::Uuid;

use crate::{
    error::ApiError,
    models::{
        api::{
            CreateUserResponse, DeleteUserResponse, ListUsersResponse, ReadUserResponse,
            UpdateUserResponse,
        },
        db::User as DbUser,
    },
};

pub async fn read(pool: &PgPool, id: Uuid) -> Result<ReadUserResponse, ApiError> {
    let sql = "SELECT * FROM users WHERE id = $1";

    let user = query_as::<_, DbUser>(sql).bind(id).fetch_one(pool).await?;

    Ok(user.into())
}

pub async fn list(pool: &PgPool) -> Result<ListUsersResponse, ApiError> {
    let sql = "SELECT * FROM users";
    let users = sqlx::query_as::<_, DbUser>(sql).fetch_all(pool).await?;

    Ok(users.into_iter().map(Into::into).collect())
}

pub async fn create(pool: &PgPool, user: DbUser) -> Result<CreateUserResponse, ApiError> {
    let sql = "
        INSERT INTO users (
            first_name, 
            last_name, 
            username, 
            email
        ) VALUES (
            $1, 
            $2, 
            $3, 
            $4
        ) RETURNING id";

    let res = query(sql)
        .bind(user.first_name)
        .bind(user.last_name)
        .bind(user.username)
        .bind(user.email)
        .fetch_one(pool)
        .await?;

    let id = res.try_get::<Uuid, _>("id")?;

    Ok(CreateUserResponse { id })
}

pub async fn update(
    pool: &PgPool,
    user_id: Uuid,
    user: DbUser,
) -> Result<UpdateUserResponse, ApiError> {
    let sql = "
        UPDATE users 
        SET 
            first_name = $1, 
            last_name = $2, 
            username = $3, 
            email = $4
        WHERE id = $5 
        RETURNING *";
    let updated_user = query_as::<_, DbUser>(sql)
        .bind(user.first_name)
        .bind(user.last_name)
        .bind(user.username)
        .bind(user.email)
        .bind(user_id)
        .fetch_one(pool)
        .await?;

    Ok(updated_user.into())
}

pub async fn delete(pool: &PgPool, user_id: Uuid) -> Result<DeleteUserResponse, ApiError> {
    let sql = "DELETE FROM users where id = $1";
    let res = query(sql).bind(user_id).execute(pool).await?;
    if res.rows_affected() < 1 {
        Err(ApiError::ResourceNotFound)
    } else {
        Ok(DeleteUserResponse)
    }
}
