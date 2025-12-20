mod handler;
mod dto;



macro_rules! ecdsa_path {
    () => { "/keys/ecdsa" };
    ($suffix:literal) => { concat!("/keys/ecdsa", $suffix) };
}

const PATH: &str = ecdsa_path!();
const PATH_WITH_ID: &str = ecdsa_path!("/{id}");
const PATH_WITH_ID_KEYPAIR: &str = ecdsa_path!("/{id}/keypair");

