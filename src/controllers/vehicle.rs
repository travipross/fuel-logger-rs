use sqlx::{query, query_as, PgPool, Row};
use uuid::Uuid;

use crate::{
    error::ApiError,
    models::{
        api::{
            CreateVehicleBody, CreateVehicleResponse, DeleteVehicleResponse, ListVehiclesResponse,
            ReadVehicleResponse, UpdateVehicleBody, UpdateVehicleResponse,
        },
        db::Vehicle as DbVehicle,
    },
};

#[tracing::instrument(name = "vehicle_controller_read", skip(pool), err)]
pub async fn read(pool: &PgPool, id: &Uuid) -> Result<ReadVehicleResponse, ApiError> {
    tracing::debug!("reading vehicle");
    let sql = "SELECT * FROM vehicles WHERE id = $1";
    let vehicle = query_as::<_, DbVehicle>(sql)
        .bind(id)
        .fetch_one(pool)
        .await?;
    tracing::info!(?vehicle, "vehicle found");
    vehicle.try_into()
}

#[tracing::instrument(name = "vehicle_controller_list", skip(pool), err)]
pub async fn list(pool: &PgPool) -> Result<ListVehiclesResponse, ApiError> {
    tracing::debug!("listing vehicles");
    let sql = "SELECT * FROM vehicles";
    let vehicles = sqlx::query_as::<_, DbVehicle>(sql).fetch_all(pool).await?;
    vehicles.into_iter().map(TryInto::try_into).collect()
}

#[tracing::instrument(name = "vehicle_controller_create", skip(pool), err)]
pub async fn create(
    pool: &PgPool,
    body: CreateVehicleBody,
) -> Result<CreateVehicleResponse, ApiError> {
    tracing::debug!("creating vehicle");
    let vehicle = DbVehicle::from_api_type(&Uuid::new_v4(), body);
    let sql = "
        INSERT INTO vehicles (
            owner_id,
            make, 
            model, 
            year, 
            odometer_unit
        ) VALUES (
            $1, 
            $2, 
            $3, 
            $4,
            $5
        ) RETURNING id";

    let res = query(sql)
        .bind(vehicle.owner_id)
        .bind(vehicle.make)
        .bind(vehicle.model)
        .bind(vehicle.year)
        .bind(vehicle.odometer_unit)
        .fetch_one(pool)
        .await?;

    let id = res.try_get::<Uuid, _>("id")?;

    Ok(CreateVehicleResponse { id })
}

#[tracing::instrument(name = "vehicle_controller_update", skip(pool), err)]
pub async fn update(
    pool: &PgPool,
    vehicle_id: &Uuid,
    body: UpdateVehicleBody,
) -> Result<UpdateVehicleResponse, ApiError> {
    tracing::debug!("updating vehicle");
    let vehicle = DbVehicle::from_api_type(vehicle_id, body);
    let sql = "
        UPDATE vehicles 
        SET 
            make = $1, 
            model = $2, 
            year = $3, 
            odometer_unit = $4
        WHERE id = $5
        RETURNING *";
    let updated_vehicle = query_as::<_, DbVehicle>(sql)
        .bind(vehicle.make)
        .bind(vehicle.model)
        .bind(vehicle.year)
        .bind(vehicle.odometer_unit)
        .bind(vehicle.id)
        .fetch_one(pool)
        .await?;

    updated_vehicle.try_into()
}

#[tracing::instrument(name = "vehicle_controller_delete", skip(pool), err)]
pub async fn delete(pool: &PgPool, vehicle_id: &Uuid) -> Result<DeleteVehicleResponse, ApiError> {
    tracing::debug!("deleting vehicle");
    let sql = "DELETE FROM vehicles WHERE id = $1 RETURNING *";
    let res = query(sql)
        .bind(vehicle_id)
        .fetch_one(pool)
        .await
        .map(|_| DeleteVehicleResponse)?;
    tracing::info!("vehicle deleted");
    Ok(res)
}

#[cfg(test)]
mod database_tests {
    use super::*;
    use crate::utils::test_utils::db::seed_user;
    use fake::{Fake, Faker};

    #[sqlx::test]
    async fn can_create_and_read(pool: PgPool) {
        // Arrange
        let owner_id = seed_user(&pool).await;
        let vehicle_body = CreateVehicleBody {
            owner_id,
            ..Faker.fake()
        };

        // Act
        let res = create(&pool, vehicle_body.clone())
            .await
            .expect("could not create resource");
        let created_result = read(&pool, &res.id).await.expect("could not read resource");

        // Assert
        assert_eq!(created_result.make, vehicle_body.make);
        assert_eq!(created_result.model, vehicle_body.model);
        assert_eq!(created_result.year, vehicle_body.year);
        assert_eq!(created_result.owner_id, vehicle_body.owner_id);
        assert_eq!(
            created_result.odometer_unit,
            vehicle_body.odometer_unit.unwrap_or_default()
        );
    }

    #[sqlx::test]
    async fn can_create_and_list(pool: PgPool) {
        // Arrange
        let owner_id = seed_user(&pool).await;
        let vehicle_body_1 = CreateVehicleBody {
            owner_id,
            ..Faker.fake()
        };
        let vehicle_body_2 = CreateVehicleBody {
            owner_id,
            ..Faker.fake()
        };

        // Act
        create(&pool, vehicle_body_1.clone())
            .await
            .expect("could not create resource");
        create(&pool, vehicle_body_2.clone())
            .await
            .expect("could not create resource");
        let created_result = list(&pool).await.expect("could not list resources");

        // Assert
        assert_eq!(created_result.len(), 2);
        for (created_item, body_item) in created_result
            .iter()
            .zip(vec![vehicle_body_1, vehicle_body_2])
        {
            assert_eq!(created_item.make, body_item.make);
            assert_eq!(created_item.model, body_item.model);
            assert_eq!(created_item.year, body_item.year);
            assert_eq!(created_item.owner_id, body_item.owner_id);
            assert_eq!(
                created_item.odometer_unit,
                body_item.odometer_unit.unwrap_or_default()
            );
        }
    }

    #[sqlx::test]
    async fn can_update(pool: PgPool) {
        // Arrange
        let owner_id = seed_user(&pool).await;
        let initial_vehicle_body = CreateVehicleBody {
            owner_id,
            ..Faker.fake()
        };
        let updated_vehicle_body = UpdateVehicleBody {
            owner_id,
            ..Faker.fake()
        };

        // Act
        let res = create(&pool, initial_vehicle_body.clone())
            .await
            .expect("could not create resource");
        let updated_result = update(&pool, &res.id, updated_vehicle_body.clone())
            .await
            .expect("could not update resource");

        // Assert
        assert_eq!(updated_result.make, updated_vehicle_body.make);
        assert_eq!(updated_result.model, updated_vehicle_body.model);
        assert_eq!(updated_result.year, updated_vehicle_body.year);
        assert_eq!(updated_result.owner_id, updated_vehicle_body.owner_id);
        assert_eq!(
            updated_result.odometer_unit,
            updated_vehicle_body.odometer_unit.unwrap_or_default()
        );
    }

    #[sqlx::test]
    async fn can_delete(pool: PgPool) {
        // Arrange
        let owner_id = seed_user(&pool).await;
        let vehicle_body = CreateVehicleBody {
            owner_id,
            ..Faker.fake()
        };

        // Act
        let res = create(&pool, vehicle_body.clone())
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
