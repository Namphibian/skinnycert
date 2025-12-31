use std::error::Error;

pub mod db;
pub mod seed;
pub mod repository;

pub trait KeyPair {
    fn generate_key_pair(&self) -> Result<(String, String), Box<dyn std::error::Error>>;
    fn verify_key_pair(
        &self,
        private_key_pem: String,
        public_key_pem: String,
    ) -> Result<(), Box<dyn Error>>;
}
