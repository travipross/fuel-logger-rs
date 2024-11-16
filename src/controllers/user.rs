use sqlx::{query, query_as, PgPool, Row};
use uuid::Uuid;

use crate::{
    error::ApiError,
    models::{
        api::{
            CreateUserBody, CreateUserResponse, DeleteUserResponse, ListUsersResponse,
            ReadUserResponse, UpdateUserBody, UpdateUserResponse,
        },
        db::User as DbUser,
    },
};

pub async fn read(pool: &PgPool, id: &Uuid) -> Result<ReadUserResponse, ApiError> {
    let sql = "SELECT * FROM users WHERE id = $1";

    let user = query_as::<_, DbUser>(sql).bind(id).fetch_one(pool).await?;

    Ok(user.into())
}

pub async fn list(pool: &PgPool) -> Result<ListUsersResponse, ApiError> {
    let sql = "SELECT * FROM users";
    let users = sqlx::query_as::<_, DbUser>(sql).fetch_all(pool).await?;

    Ok(users.into_iter().map(Into::into).collect())
}

pub async fn create(pool: &PgPool, body: CreateUserBody) -> Result<CreateUserResponse, ApiError> {
    let user = DbUser::from_api_type(&Uuid::new_v4(), body);
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
    user_id: &Uuid,
    body: UpdateUserBody,
) -> Result<UpdateUserResponse, ApiError> {
    let user = DbUser::from_api_type(user_id, body);
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

pub async fn delete(pool: &PgPool, user_id: &Uuid) -> Result<DeleteUserResponse, ApiError> {
    let sql = "DELETE FROM users where id = $1";
    let res = query(sql).bind(user_id).execute(pool).await?;
    if res.rows_affected() < 1 {
        Err(ApiError::ResourceNotFound)
    } else {
        Ok(DeleteUserResponse)
    }
}

#[cfg(test)]
mod database_tests {
    use super::*;
    use fake::{Fake, Faker};

    #[sqlx::test]
    async fn can_create_and_read(pool: PgPool) {
        // Arrange
        let user_body = Faker.fake::<CreateUserBody>();

        // Act
        let res = create(&pool, user_body.clone())
            .await
            .expect("could not create resource");
        let created_result = read(&pool, &res.id).await.expect("could not read resource");

        // Assert
        assert_eq!(created_result.first_name, user_body.first_name);
        assert_eq!(created_result.last_name, user_body.last_name);
        assert_eq!(created_result.username, user_body.username);
        assert_eq!(created_result.email, user_body.email);
    }

    #[sqlx::test]
    async fn can_create_and_list(pool: PgPool) {
        // Arrange
        let user_body_1 = Faker.fake::<CreateUserBody>();
        let user_body_2 = Faker.fake::<CreateUserBody>();

        // Act
        create(&pool, user_body_1.clone())
            .await
            .expect("could not create resource");
        create(&pool, user_body_2.clone())
            .await
            .expect("could not create resource");
        let created_result = list(&pool).await.expect("could not list resources");

        // Assert
        assert_eq!(created_result.len(), 2);
        for (created_item, body_item) in created_result.iter().zip(vec![user_body_1, user_body_2]) {
            assert_eq!(created_item.first_name, body_item.first_name);
            assert_eq!(created_item.last_name, body_item.last_name);
            assert_eq!(created_item.username, body_item.username);
            assert_eq!(created_item.email, body_item.email);
        }
    }

    #[sqlx::test]
    async fn can_update(pool: PgPool) {
        // Arrange
        let initial_user_body = Faker.fake::<CreateUserBody>();
        let updated_user_body = Faker.fake::<UpdateUserBody>();

        // Act
        let res = create(&pool, initial_user_body.clone())
            .await
            .expect("could not create resource");
        let updated_result = update(&pool, &res.id, updated_user_body.clone())
            .await
            .expect("could not update resource");

        // Assert
        assert_eq!(updated_result.first_name, updated_user_body.first_name);
        assert_eq!(updated_result.last_name, updated_user_body.last_name);
        assert_eq!(updated_result.username, updated_user_body.username);
        assert_eq!(updated_result.email, updated_user_body.email);
    }

    #[sqlx::test]
    async fn can_delete(pool: PgPool) {
        // Arrange
        let user_body = Faker.fake::<CreateUserBody>();

        // Act
        let res = create(&pool, user_body.clone())
            .await
            .expect("could not create resource");
        read(&pool, &res.id).await.expect("could not read resource");
        delete(&pool, &res.id)
            .await
            .expect("could not read resource");
        let created_result = read(&pool, &res.id)
            .await
            .expect_err("expected_failure_did_not_occur");

        assert!(matches!(created_result, ApiError::ResourceNotFound));
    }
}
