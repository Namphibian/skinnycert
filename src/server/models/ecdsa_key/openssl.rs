//! Retrieves a list of all built-in Elliptic Curve (EC) curves available in OpenSSL.
//!
//! # Description
//! This function interacts with the OpenSSL library to retrieve and return the details of
//! all built-in EC curves. Each curve is represented as a tuple containing its OpenSSL
//! `Nid` (numerical identifier) and a human-readable description string (comment).
//!
//! # Returns
//! A vector of tuples where each tuple consists of:
//! - `Nid`: The OpenSSL identifier for the elliptic curve.
//! - `String`: A human-readable description or comment for the elliptic curve.
//!
//! # Example
//! ```rust
//! let curves = builtin_curves();
//! for (nid, comment) in curves {
//!     println!("Curve NID: {:?}, Comment: {}", nid, comment);
//! }
//! ```
//!
//! # Safety
//! This function makes use of unsafe code to interact with the C API of OpenSSL:
//! - The `EC_get_builtin_curves` function is called to retrieve the built-in curves.
//! - Memory safety is manually managed, including pre-allocating the vector with
//!   `Vec::with_capacity` and setting its length with `set_len`. Ensure the OpenSSL
//!   library is correctly initialized and in use.
//!
//! # Panics
//! The function will panic if:
//! - The number of curves actually populated does not match the count returned
//!   during initialization (`assert_eq!(got, count)`) due to unexpected behavior from
//!   OpenSSL.
//!
//! # Requirements
//! Ensure that the OpenSSL library linked to your program contains EC curve support.
//! If EC curves are not supported or the library is incorrectly initialized, this
//! function may not behave as expected.
//!
//! # See Also
//! - [OpenSSL documentation on EC curves](https://www.openssl.org/docs)
//! - [`Nid`](https://docs.rs/openssl/latest/openssl/nid/struct.Nid.html): Numerical
//!   Identifiers for OpenSSL objects such as EC curves.
//!
//! # FFI Details
//! - `EC_get_builtin_curves(r: *mut EcBuiltinCurve, n: c_int) -> c_int`:
//!   Retrieves OpenSSL's built-in EC curves. The first argument is a pointer where
//!   the data will be stored, and the second argument specifies the number of curves
//!   to retrieve. If the pointer is NULL, the function returns the total number of
//!   curves available.
//!
//! - `EcBuiltinCurve`: Represents an elliptic curve with its numerical ID (`nid`) and
//!   a pointer to a comment string.
//!
//! # Unsafe Behavior
//! Handle the unsafe blocks carefully:
//! - Ensure that the library call to `EC_get_builtin_curves` is valid.
//! - Properly interpret the returned `*const c_char` to a Rust string
//!   to avoid undefined behavior or crashes.
use openssl::nid::Nid;
use sqlx::PgPool;
use std::ffi::{c_char, c_int};

#[repr(C)]
#[derive(Debug)]
pub struct EcBuiltinCurve {
    nid: c_int,
    comment: *const c_char,
}

unsafe extern "C" {
    fn EC_get_builtin_curves(r: *mut EcBuiltinCurve, n: c_int) -> c_int;
}
//
// fn extract_curve_size(comment: &str) -> Option<i32> {
//     let re = regex::Regex::new(r"(\d+)\s*bit").unwrap();
//     re.captures(comment)
//         .and_then(|cap| cap.get(1))
//         .and_then(|m| m.as_str().parse::<i32>().ok())
// }
//
// /// Returns all builtin EC curves from OpenSSL
// pub fn builtin_curves() -> Vec<(Nid, String)> {
//     unsafe {
//         // First call with null to get the count
//         let count = EC_get_builtin_curves(std::ptr::null_mut(), 0);
//         let mut curves: Vec<EcBuiltinCurve> = Vec::with_capacity(count as usize);
//
//         // Fill the vector
//         let got = EC_get_builtin_curves(curves.as_mut_ptr(), count);
//         assert_eq!(got, count);
//
//         curves.set_len(count as usize);
//
//         curves
//             .into_iter()
//             .map(|c| {
//                 let nid = Nid::from_raw(c.nid);
//                 let comment = if c.comment.is_null() {
//                     "".to_string()
//                 } else {
//                     std::ffi::CStr::from_ptr(c.comment)
//                         .to_string_lossy()
//                         .into_owned()
//                 };
//                 (nid, comment)
//             })
//             .collect()
//     }
// }
//
// // --- Seeding function ---
//
// pub async fn configure_default_ecdsa_algorithm(pool: &PgPool) -> Result<(), sqlx::Error> {
//     let curves = builtin_curves();
//
//     for (nid, comment) in curves {
//         let nid_value = nid.as_raw();
//         let display_name = comment.clone();
//         let curve_size = extract_curve_size(&comment).unwrap_or(0);
//
//         sqlx::query!(
//             r#"
//             INSERT INTO ecdsa_key_algorithm (algorithm, nid_value, display_name, curve_size)
//             VALUES ($1, $2, $3, $4)
//             ON CONFLICT (nid_value) DO UPDATE
//             SET display_name = EXCLUDED.display_name,
//                 curve_size   = EXCLUDED.curve_size
//             "#,
//             "ECDSA",
//             nid_value,
//             display_name,
//             curve_size,
//         )
//         .execute(pool)
//         .await?;
//     }
//
//     Ok(())
// }
