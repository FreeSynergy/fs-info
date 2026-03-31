// grpc.rs — gRPC service implementation for fs-info.
//
// Wraps an Arc<FsInfo> and exposes it via the InfoService proto.
// Routes:
//   SystemInfo / CpuUsage / MemoryInfo / DiskInfo / Health

use tonic::{Request, Response, Status};
use tracing::instrument;

use crate::facade::{FsInfo, SystemInfo};

// Include the generated tonic code.
pub mod proto {
    #![allow(clippy::all, clippy::pedantic, warnings)]
    tonic::include_proto!("info");
}

pub use proto::info_service_server::{InfoService, InfoServiceServer};
pub use proto::{
    CpuUsageRequest, CpuUsageResponse, DiskInfoRequest, DiskInfoResponse, HealthRequest,
    HealthResponse, LoadAverageProto, MemoryInfoRequest, MemoryInfoResponse, PartitionProto,
    SystemInfoRequest, SystemInfoResponse,
};

// ── GrpcInfo ──────────────────────────────────────────────────────────────────

/// gRPC service wrapper around [`FsInfo`].
pub struct GrpcInfo {
    info: FsInfo,
}

impl GrpcInfo {
    /// Create a new gRPC info service.
    #[must_use]
    pub fn new() -> Self {
        Self {
            info: FsInfo::new(),
        }
    }
}

impl Default for GrpcInfo {
    fn default() -> Self {
        Self::new()
    }
}

#[tonic::async_trait]
impl InfoService for GrpcInfo {
    #[instrument(name = "grpc.info.system_info", skip(self))]
    async fn system_info(
        &self,
        _req: Request<SystemInfoRequest>,
    ) -> Result<Response<SystemInfoResponse>, Status> {
        let os = self.info.os();
        let uptime = self.info.uptime();
        let cpu = self.info.cpu();
        #[allow(clippy::cast_possible_truncation)]
        Ok(Response::new(SystemInfoResponse {
            hostname: os.hostname,
            os_type: os.os_type.label().to_owned(),
            os_version: os.version,
            kernel: os.kernel,
            arch: os.arch,
            uptime_secs: uptime.seconds,
            cpu_brand: cpu.brand,
            core_count: cpu.core_count as u32,
        }))
    }

    #[instrument(name = "grpc.info.cpu_usage", skip(self))]
    async fn cpu_usage(
        &self,
        _req: Request<CpuUsageRequest>,
    ) -> Result<Response<CpuUsageResponse>, Status> {
        let cpu = self.info.cpu();
        #[allow(clippy::cast_possible_truncation)]
        Ok(Response::new(CpuUsageResponse {
            usage_percent: cpu.usage_percent,
            core_count: cpu.core_count as u32,
            per_core_percent: cpu.per_core_percent,
            load_average: Some(LoadAverageProto {
                one: cpu.load_average.one,
                five: cpu.load_average.five,
                fifteen: cpu.load_average.fifteen,
            }),
            brand: cpu.brand,
        }))
    }

    #[instrument(name = "grpc.info.memory_info", skip(self))]
    async fn memory_info(
        &self,
        _req: Request<MemoryInfoRequest>,
    ) -> Result<Response<MemoryInfoResponse>, Status> {
        let mem = self.info.memory();
        Ok(Response::new(MemoryInfoResponse {
            total_bytes: mem.total_bytes,
            used_bytes: mem.used_bytes,
            available_bytes: mem.available_bytes,
            used_percent: mem.used_percent(),
            swap_total_bytes: mem.swap_total_bytes,
            swap_used_bytes: mem.swap_used_bytes,
        }))
    }

    #[instrument(name = "grpc.info.disk_info", skip(self))]
    async fn disk_info(
        &self,
        _req: Request<DiskInfoRequest>,
    ) -> Result<Response<DiskInfoResponse>, Status> {
        let disk = self.info.disk();
        let partitions = disk
            .partitions
            .iter()
            .map(|p| PartitionProto {
                name: p.fs_type.clone(),
                mount_point: p.mount_point.clone(),
                total_bytes: p.total_bytes,
                used_bytes: p.used_bytes(),
                available_bytes: p.available_bytes,
                used_percent: p.used_percent(),
            })
            .collect();
        Ok(Response::new(DiskInfoResponse { partitions }))
    }

    async fn health(
        &self,
        _req: Request<HealthRequest>,
    ) -> Result<Response<HealthResponse>, Status> {
        Ok(Response::new(HealthResponse {
            ok: true,
            version: env!("CARGO_PKG_VERSION").to_owned(),
        }))
    }
}
