use std::error::Error;
use crate::server::models::certificates::db::CsrGenerationParams;

pub mod db;
pub mod repository;
pub mod seed;

pub trait KeyPair {
    fn generate_key_pair(&self) -> Result<(String, String), Box<dyn std::error::Error>>;
    fn verify_key_pair(
        &self,
        private_key_pem: String,
        public_key_pem: String,
    ) -> Result<(), Box<dyn Error>>;
}


pub trait GenerateCertificateSigningRequest {
    fn generate_csr(
        &self,
        private_key_pem: &str,
        params: &CsrGenerationParams,
    ) -> Result<String, Box<dyn std::error::Error>>;
}
