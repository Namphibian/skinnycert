use crate::server::models::rsa_keys::db::RSAKeyAlgorithm;
use sqlx::PgPool;
use std::error::Error;
use uuid::Uuid;

pub struct RsaKeyRepository {
    pool: PgPool,
}

impl RsaKeyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        display_name: &str,
        rsa_key_size: i32,
    ) -> Result<Uuid, Box<dyn Error>> {
        let mut tx = self.pool.begin().await?;
        let rsa_id: Uuid = sqlx::query_scalar(
            r"
                    INSERT INTO rsa_key_algorithm
                    (
                        algorithm,
                        rsa_key_size,
                        display_name
                    )
                    VALUES
                        (
                            'RSA',
                            #1::rsa_key_size,
                            #2::display_name
                            
                        );
                    RETURNING id
            ",
        )
        .bind(display_name)
        .bind(rsa_key_size)
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(rsa_id)
    }
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<RSAKeyAlgorithm>, Box<dyn Error>> {
        let result = sqlx::query_as::<_, RSAKeyAlgorithm>(
            r#"
            SELECT * FROM rsa_key_algorithm WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }
    pub async fn find_all(&self) -> Result<Vec<RSAKeyAlgorithm>, Box<dyn Error>> {
        let results = sqlx::query_as::<_, RSAKeyAlgorithm>(
            r#"
            SELECT * FROM rsa_key_algorithm
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }
}
