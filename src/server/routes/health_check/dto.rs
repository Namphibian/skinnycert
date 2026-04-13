use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Represents the JSON payload returned by the `/health` endpoint.
///
/// The health check response wraps a detailed system and processes
/// memory usage information inside a [`MemoryInfo`] struct.
///
/// # Example
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
/// This structure is serialized automatically by Actix Web when returned
/// with `HttpResponse::Ok().json(HealthCheckResponse { ... })`.
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckResponse {
    /// Contains system and process memory metrics.
    pub memory_info: MemoryInfo,
}

/// Represents system and process memory statistics, measured in kilobytes (KB).
///
/// This struct is nested inside [`HealthCheckResponse`] and provides
/// low-level runtime information useful for diagnostics, monitoring,
/// and container health checks.
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MemoryInfo {
    /// Total amount of physical RAM, in kilobytes.
    pub total_memory_kb: u64,

    /// Amount of free system memory (RAM) that is completely unused, in kilobytes.
    ///
    /// This value is parsed from `/proc/meminfo` → `MemFree`.
    pub free_memory_kb: u64,

    /// Amount of system memory (RAM) available for starting new applications, in kilobytes.
    ///
    /// This value is parsed from `/proc/meminfo` → `MemAvailable`.
    pub available_memory_kb: u64,

    /// Memory currently used by this process (resident set size), in kilobytes.
    ///
    /// This value is parsed from `/proc/self/status` → `VmRSS`.
    pub process_memory_kb: u64,
}
