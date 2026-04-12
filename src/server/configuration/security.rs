use openssl::rand::rand_bytes;

/// Generates random bytes and checks for errors.
///
/// This function attempts to fill a buffer with 16 random bytes using the openssl `rand_bytes` function.
/// This is to make sure that the environment is secure for cryptographic applications.
///
/// # Returns
///
/// * `Ok(())` - If random bytes are successfully generated and the buffer is filled.
/// * `Err(Box<dyn std::error::Error>)` - If an error occurs while generating random bytes.
///
/// # Errors
///
/// This function propagates any errors returned by the `rand_bytes` function.

pub fn check_rng() -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Checking OpenSSL random bytes...");
    let mut buf = [0u8; 16];
    rand_bytes(&mut buf)?;
    Ok(())
}