use sqlx::{postgres::PgRow, query, query_as, FromRow, PgPool, QueryBuilder, Row};
use uuid::Uuid;

use crate::{
    error::ApiError,
    models::{
        api::{
            CreateLogRecordBody, CreateLogRecordResponse, DeleteLogRecordResponse,
            ListLogRecordsResponse, ReadLogRecordResponse, UpdateLogRecordBody,
            UpdateLogRecordResponse,
        },
        db::LogRecord as DbLogRecord,
    },
    types::LogType,
};

pub async fn read(pool: &PgPool, id: &Uuid) -> Result<ReadLogRecordResponse, ApiError> {
    let sql = "SELECT * FROM log_records WHERE id = $1";

    let log_record = query_as::<_, DbLogRecord>(sql)
        .bind(id)
        .fetch_one(pool)
        .await?;

    log_record.try_into()
}

pub async fn list(pool: &PgPool) -> Result<ListLogRecordsResponse, ApiError> {
    let sql = "SELECT * FROM log_records";
    let log_records = sqlx::query_as::<_, DbLogRecord>(sql)
        .fetch_all(pool)
        .await?;

    log_records.into_iter().map(TryInto::try_into).collect()
}

pub async fn create(
    pool: &PgPool,
    body: CreateLogRecordBody,
) -> Result<CreateLogRecordResponse, ApiError> {
    let log_record = DbLogRecord::from_api_type(&Uuid::new_v4(), body)?;

    // Initialize query builder with INSERT statement
    let mut qb = QueryBuilder::<sqlx::Postgres>::new("INSERT INTO log_records(");
    let mut separated = qb.separated(", ");

    // Add common fields
    separated.push("id");
    separated.push("vehicle_id");
    separated.push("log_date");
    separated.push("odometer");
    separated.push("log_type");
    separated.push("notes");

    // Conditionally add columns based on log type
    match log_record.log_type {
        LogType::FuelUp { .. } => {
            separated.push("fuel_amount");
        }
        LogType::TireChange { ref rotation, .. } => {
            separated.push("tire_type");
            separated.push("new_tires");
            if rotation.is_some() {
                separated.push("tire_rotation_type");
            }
        }
        LogType::BrakeReplacement { .. } => {
            separated.push("brake_location");
            separated.push("brake_part");
        }
        LogType::Fluids(_) => {
            separated.push("fluid_type");
        }
        LogType::TireRotation(_) => {
            separated.push("tire_rotation_type");
        }
        _ => {}
    }

    // Begin VALUES
    separated.push_unseparated(") VALUES (");

    // Push bindings for common required fields
    separated.push_bind_unseparated(log_record.id);
    separated.push_bind(log_record.vehicle_id);
    separated.push_bind(log_record.date);
    separated.push_bind(log_record.odometer);
    separated.push_bind(log_record.log_type.to_string());
    separated.push_bind(log_record.notes);

    // Push bindings for type-specific fields
    match log_record.log_type {
        LogType::FuelUp { fuel_amount } => {
            separated.push_bind(fuel_amount);
        }
        LogType::TireChange {
            rotation,
            tire_type,
            new,
        } => {
            separated.push_bind(tire_type);
            separated.push_bind(new);
            if let Some(rotation_type) = rotation {
                separated.push_bind(rotation_type);
            }
        }
        LogType::BrakeReplacement {
            location,
            component,
        } => {
            separated.push_bind(location);
            separated.push_bind(component);
        }
        LogType::Fluids(fluid_type) => {
            separated.push_bind(fluid_type);
        }
        LogType::TireRotation(rotation_type) => {
            separated.push_bind(rotation_type);
        }
        _ => {}
    }

    // Terminate statement
    separated.push_unseparated(") RETURNING id");

    // Build and execute
    let query_to_execute = qb.build();
    let res = query_to_execute.fetch_one(pool).await?;
    let id = res.try_get::<Uuid, _>("id")?;

    Ok(CreateLogRecordResponse { id })
}

pub async fn update(
    pool: &PgPool,
    log_record_id: &Uuid,
    body: UpdateLogRecordBody,
) -> Result<UpdateLogRecordResponse, ApiError> {
    let existing_val = read(pool, log_record_id).await?;
    if std::mem::discriminant(&body.log_type) == std::mem::discriminant(&existing_val.log_type) {
        let log_record = DbLogRecord::from_api_type(log_record_id, body)?;
        let mut qb = QueryBuilder::<sqlx::Postgres>::new("UPDATE log_records SET ");
        let mut separated = qb.separated(", ");

        separated.push("log_date = ");
        separated.push_bind_unseparated(log_record.date);
        separated.push("odometer = ");
        separated.push_bind_unseparated(log_record.odometer);
        separated.push("log_type = ");
        separated.push_bind_unseparated(log_record.log_type.to_string());
        separated.push("notes = ");
        separated.push_bind_unseparated(log_record.notes);

        // Push bindings for type-specific fields
        match log_record.log_type {
            LogType::FuelUp { fuel_amount } => {
                separated.push("fuel_amount = ");
                separated.push_bind_unseparated(fuel_amount);
            }
            LogType::TireChange {
                rotation,
                tire_type,
                new,
            } => {
                separated.push("tire_type = ");
                separated.push_bind_unseparated(tire_type);
                separated.push("new_tires = ");
                separated.push_bind_unseparated(new);
                if let Some(rotation_type) = rotation {
                    separated.push("tire_rotation_type = ");
                    separated.push_bind_unseparated(rotation_type);
                }
            }
            LogType::BrakeReplacement {
                location,
                component,
            } => {
                separated.push("brake_location = ");
                separated.push_bind_unseparated(location);
                separated.push("brake_part = ");
                separated.push_bind_unseparated(component);
            }
            LogType::Fluids(fluid_type) => {
                separated.push("fluid_type = ");
                separated.push_bind_unseparated(fluid_type);
            }
            LogType::TireRotation(rotation_type) => {
                separated.push("tire_rotation_type = ");
                separated.push_bind_unseparated(rotation_type);
            }
            _ => {}
        }

        qb.push(" WHERE id = ");
        qb.push_bind(log_record_id);
        qb.push(" RETURNING *");

        let q = qb.build();
        let updated_record = <DbLogRecord as FromRow<PgRow>>::from_row(&q.fetch_one(pool).await?)?;

        updated_record.try_into()
    } else {
        // If types don't match, record shouldn't be updated
        Err(ApiError::WrongLogRecordType)
    }
}

pub async fn delete(
    pool: &PgPool,
    log_record_id: &Uuid,
) -> Result<DeleteLogRecordResponse, ApiError> {
    let sql = "DELETE FROM log_records WHERE id = $1 RETURNING *";
    Ok(query(sql)
        .bind(log_record_id)
        .fetch_one(pool)
        .await
        .map(|_| DeleteLogRecordResponse)?)
}

#[cfg(test)]
mod database_tests {
    use super::*;
    use crate::utils::test_utils::db::seed_user_and_vehicle;
    use fake::{Fake, Faker};
    use itertools::izip;

    #[sqlx::test]
    async fn can_create_and_read(pool: PgPool) {
        // Arrange
        let vehicle_id = seed_user_and_vehicle(&pool).await;
        let date = Faker.fake();
        let notes = Faker.fake();
        let log_record_body = CreateLogRecordBody {
            date: Some(date),
            notes,
            vehicle_id,
            ..Faker.fake()
        };

        // Act
        let res = create(&pool, log_record_body.clone())
            .await
            .expect("could not create resource");
        let created_result = read(&pool, &res.id).await.expect("could not read resource");

        // Assert
        assert_eq!(
            created_result.date.timestamp_millis(),
            date.timestamp_millis()
        );
        assert_eq!(created_result.vehicle_id, log_record_body.vehicle_id);
        assert_eq!(created_result.log_type, log_record_body.log_type);
        assert_eq!(created_result.odometer, log_record_body.odometer);
        assert_eq!(created_result.notes, log_record_body.notes);
    }

    #[sqlx::test]
    async fn can_create_and_list(pool: PgPool) {
        // Arrange
        let date_1 = Faker.fake();
        let date_2 = Faker.fake();

        let notes_1 = Faker.fake();
        let notes_2 = Faker.fake();

        let vehicle_id = seed_user_and_vehicle(&pool).await;
        let log_record_body_1 = CreateLogRecordBody {
            notes: notes_1,
            date: Some(date_1),
            vehicle_id,
            ..Faker.fake()
        };
        let log_record_body_2 = UpdateLogRecordBody {
            notes: notes_2,
            date: Some(date_2),
            vehicle_id,
            ..Faker.fake()
        };

        // Act
        create(&pool, log_record_body_1.clone())
            .await
            .expect("could not create resource");
        create(&pool, log_record_body_2.clone())
            .await
            .expect("could not create resource");
        let created_result = list(&pool).await.expect("could not list resources");

        // Assert
        assert_eq!(created_result.len(), 2);
        for (created_item, body_item, date_item) in izip!(
            created_result,
            vec![log_record_body_1, log_record_body_2],
            vec![date_1, date_2]
        ) {
            assert_eq!(
                created_item.date.timestamp_millis(),
                date_item.timestamp_millis()
            );
            assert_eq!(created_item.vehicle_id, body_item.vehicle_id);
            assert_eq!(created_item.log_type, body_item.log_type);
            assert_eq!(created_item.odometer, body_item.odometer);
            assert_eq!(created_item.notes, body_item.notes);
        }
    }

    #[sqlx::test]
    async fn can_update(pool: PgPool) {
        // Arrange
        let vehicle_id = seed_user_and_vehicle(&pool).await;
        let date = Faker.fake();
        let notes = Faker.fake();
        let log_type = Faker.fake::<LogType>();
        let initial_log_record_body = CreateLogRecordBody {
            vehicle_id,
            log_type: log_type.clone(),
            ..Faker.fake()
        };
        let updated_log_record_body = CreateLogRecordBody {
            date: Some(date),
            notes,
            log_type,
            vehicle_id,
            ..Faker.fake()
        };

        // Act
        let res = create(&pool, initial_log_record_body.clone())
            .await
            .expect("could not create resource");
        let updated_result = update(&pool, &res.id, updated_log_record_body.clone())
            .await
            .expect("could not update resource");

        // Assert
        assert_eq!(
            updated_result.date.timestamp_millis(),
            date.timestamp_millis()
        );
        assert_eq!(
            updated_result.vehicle_id,
            updated_log_record_body.vehicle_id
        );
        assert_eq!(updated_result.notes, updated_log_record_body.notes);
        assert_eq!(updated_result.odometer, updated_log_record_body.odometer);
        assert_eq!(updated_result.log_type, updated_log_record_body.log_type);
    }

    #[sqlx::test]
    async fn can_delete(pool: PgPool) {
        // Arrange
        let vehicle_id = seed_user_and_vehicle(&pool).await;
        let log_record_body = CreateLogRecordBody {
            vehicle_id,
            ..Faker.fake()
        };

        // Act
        let res = create(&pool, log_record_body.clone())
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
