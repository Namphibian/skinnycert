use actix_web::{test, App};
use serde_json::Value;
use uuid::Uuid;
use chrono::{Utc, Duration};
use sqlx::PgPool;

pub async fn insert_test_certificates(pool: &PgPool, count: usize) {
    for i in 0..count {
        let created_on = Utc::now() - Duration::minutes(i as i64);

        sqlx::query(
            r#"
            INSERT INTO certificates (
                id, csr_pem, key_pem, public_key_pem, key_algorithm_id,
                organization, country, created_on, updated_on
            )
            VALUES ($1, 'csr', 'key', 'pub', gen_random_uuid(),
                    'Org', 'AU', $2, $2)
            "#,
        )
            .bind(Uuid::new_v4())
            .bind(created_on)
            .execute(pool)
            .await
            .unwrap();
    }
}

pub fn extract_tokens(body: &Value) -> (Option<String>, Option<String>) {
    let next = body["nextPageToken"].as_str().map(|s| s.to_string());
    let prev = body["prevPageToken"].as_str().map(|s| s.to_string());
    (next, prev)
}
