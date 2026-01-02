use crate::server::routes::health_check::dto::{HealthCheckResponse, MemoryInfo};
use actix_web::{HttpResponse, Responder};
use std::fs;

/// Handles the `GET /health` endpoint.
///
/// This endpoint performs a basic health check of the running process
/// by returning information about:
/// - **Free system memory (KB)**
/// - **Memory used by the current process (KB)**
///
/// It returns a JSON payload of type [`HealthCheckResponse`], which wraps
/// a [`MemoryInfo`] structure containing the memory statistics.
///
/// # Example JSON Response
/// ```json
/// {
///   "memory_info": {
///     "free_memory_kb": 1823920,
///     "process_memory_kb": 48212
///   }
/// }
/// ```
///
/// # Returns
/// - [`HttpResponse::Ok`] with a JSON payload on success.
///
/// # Errors
/// Panics if `/proc/meminfo` or `/proc/self/status` cannot be read.
/// This behavior is fine for internal health checks but can be
/// changed to return HTTP 500 if desired.
#[tracing::instrument(name = "Healthcheck GET Request.")]
pub async fn get_handler() -> impl Responder {
    // --- Free system memory ---
    let free_memory_kb = get_free_memory();

    // --- Current process memory usage ---
    let process_memory_kb = get_process_memory();

    // Build the response structure
    let mem_info = MemoryInfo {
        free_memory_kb,
        process_memory_kb,
    };
    let health_check_response = HealthCheckResponse {
        memory_info: mem_info,
    };

    HttpResponse::Ok().json(health_check_response)
}

/// Handles the `POST /health` endpoint.
///
/// By design, the health check endpoint only supports `GET` requests.
/// Any `POST` request will return a **405 Method Not Allowed** response.
///
/// # Returns
/// - [`HttpResponse::MethodNotAllowed`]
#[tracing::instrument(name = "Healthcheck POST Request.")]
pub async fn post_handler() -> impl Responder {
    HttpResponse::MethodNotAllowed()
}

/// Reads the amount of free system memory (in KB) from `/proc/meminfo`.
///
/// This function is **Linux-specific** and parses the `MemFree` field
/// from `/proc/meminfo`.
///
/// # Panics
/// Panics if `/proc/meminfo` cannot be read or parsed.
#[tracing::instrument(name = "Get Free Memory.")]
fn get_free_memory() -> u64 {
    let mem_info = fs::read_to_string("/proc/meminfo").expect("Failed to read /proc/meminfo");

    mem_info
        .lines()
        .find(|line| line.starts_with("MemFree:"))
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|val| val.parse::<u64>().ok())
        .expect("Failed to parse MemFree")
}

/// Reads the current process's resident memory usage (in KB)
/// from `/proc/self/status`.
///
/// This function looks for the `VmRSS` field, which reports the amount of
/// physical memory currently used by the running process.
///
/// # Panics
/// Panics if `/proc/self/status` cannot be read or parsed.
#[tracing::instrument(name = "Get Process Memory.")]
fn get_process_memory() -> u64 {
    let status = fs::read_to_string("/proc/self/status").expect("Failed to read /proc/self/status");

    status
        .lines()
        .find(|line| line.starts_with("VmRSS:"))
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|val| val.parse::<u64>().ok())
        .expect("Failed to parse VmRSS")
}
