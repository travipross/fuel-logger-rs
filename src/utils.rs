#[cfg(test)]
pub mod test_utils {
    pub fn merge_json_objects(target: &mut serde_json::Value, source: serde_json::Value) {
        // Convert both to objects
        let target_obj = target
            .as_object_mut()
            .expect("could not convert target to mutable object");
        let source_obj = source
            .as_object()
            .expect("could not convert source to object");

        // Merge source into target
        for (k, v) in source_obj {
            target_obj.insert(k.clone(), v.clone());
        }
    }

    pub mod db {
        use crate::models::api::CreateVehicleBody;
        use fake::{Fake, Faker};
        use sqlx::PgPool;
        use uuid::Uuid;

        pub async fn seed_user_and_vehicle(pool: &PgPool) -> Uuid {
            let user_id = crate::controllers::user::create(&pool, Faker.fake())
                .await
                .expect("failed to seed user")
                .id;

            crate::controllers::vehicle::create(
                &pool,
                CreateVehicleBody {
                    owner_id: user_id,
                    ..Faker.fake()
                },
            )
            .await
            .expect("could not seed vehicle")
            .id
        }

        pub async fn seed_user(pool: &PgPool) -> Uuid {
            crate::controllers::user::create(&pool, Faker.fake())
                .await
                .expect("failed to seed user")
                .id
        }
    }
}
