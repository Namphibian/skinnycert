use crate::server::routes::health_check::dto::{HealthCheckResponse, MemoryInfo};
use actix_web::{HttpResponse, Responder};
use std::fs;

/// Handles the `GET /health` endpoint.
///
/// This endpoint performs a basic health check of the running process
/// by returning information about:
/// - **Total system memory (KB)**
/// - **Free system memory (KB)**
/// - **Available system memory (KB)**
/// - **Memory used by the current process (KB)**
///
/// It returns a JSON payload of type [`HealthCheckResponse`], which wraps
/// a [`MemoryInfo`] structure containing the memory statistics.
///
/// # Example JSON Response
/// ```json
/// {
///   "memory_info": {
///     "total_memory_kb": 16000000,
///     "free_memory_kb": 1823920,
///     "available_memory_kb": 12000000,
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
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Basic health check", body = HealthCheckResponse)
    )
)]
#[tracing::instrument(name = "Healthcheck GET Request.")]
pub async fn get_health() -> impl Responder {
    // --- System memory info ---
    let (total_kb, free_kb, available_kb) = get_system_memory();

    // --- Current process memory usage ---
    let process_memory_kb = get_process_memory();

    // Build the response structure
    let mem_info = MemoryInfo {
        total_memory_kb: total_kb,
        free_memory_kb: free_kb,
        available_memory_kb: available_kb,
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
pub async fn post_health() -> impl Responder {
    HttpResponse::MethodNotAllowed()
}

/// Reads system memory information from `/proc/meminfo`.
///
/// Returns a tuple containing:
/// 1. Total memory (KB)
/// 2. Free memory (KB)
/// 3. Available memory (KB)
///
/// This function is **Linux-specific**.
///
/// # Panics
/// Panics if `/proc/meminfo` cannot be read or parsed.
#[tracing::instrument(name = "Get System Memory.")]
fn get_system_memory() -> (u64, u64, u64) {
    let mem_info = fs::read_to_string("/proc/meminfo").expect("Failed to read /proc/meminfo");

    let mut total = 0;
    let mut free = 0;
    let mut available = 0;

    for line in mem_info.lines() {
        if line.starts_with("MemTotal:") {
            total = parse_meminfo_line(line);
        } else if line.starts_with("MemFree:") {
            free = parse_meminfo_line(line);
        } else if line.starts_with("MemAvailable:") {
            available = parse_meminfo_line(line);
        }
    }

    // Fallback for older kernels where MemAvailable might not be present
    if available == 0 {
        available = free;
    }

    (total, free, available)
}

fn parse_meminfo_line(line: &str) -> u64 {
    line.split_whitespace()
        .nth(1)
        .and_then(|val| val.parse::<u64>().ok())
        .unwrap_or(0)
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
