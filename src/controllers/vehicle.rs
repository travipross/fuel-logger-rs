use std::error::Error;

use fake::{Fake, Faker};
use sqlx::{query_as, Pool, Postgres};
use uuid::Uuid;

use crate::models::Vehicle;

pub async fn read(pool: Pool<Postgres>, id: Uuid) -> Result<Vehicle, Box<dyn Error>> {
    let record = query_as!(
        Vehicle,
        "SELECT * FROM vehicles WHERE id=$1",
        id.to_string()
    )
    .fetch_one(&pool)
    .await?;

    Vehicle {
        id,
        ..Faker.fake::<Vehicle>()
    }
}
