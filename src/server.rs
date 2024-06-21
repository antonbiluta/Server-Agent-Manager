use jsonwebtoken::{decode, DecodingKey, Validation};
use sysinfo::{System, Disks, Networks};
use tokio::signal::{self};
use tonic::{transport::Server, Request, Response, Status};
use tokio::sync::Mutex;
use std::sync::Arc;
use tonic_reflection::server::Builder;
use crate::config::{Claims, ServerConfig};
use server_client_utility::auth_protocol::server_admin_server::{ServerAdmin, ServerAdminServer};
use server_client_utility::auth_protocol::{AuthRequest, AuthResponse, CpuUsageResponse, DiskUsageResponse, DiskUsage, MemoryUsageResponse, MemoryUsage, NetworkUsageResponse, NetworkUsage};

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            name: String::new(),
            location: String::new(),
            ip_port: String::new(),
            description: String::new(),
            password: String::new(),
            token: String::new()
        }
    }
}

#[derive(Default)]
pub struct MyServerAdmin {
    pub config: ServerConfig,
    pub system: Arc<Mutex<System>>,
}

impl MyServerAdmin {
    pub fn new(config: ServerConfig) -> Self {
        let system = Arc::new(Mutex::new(System::new_all()));
        MyServerAdmin { config, system }
    }
}

#[tonic::async_trait]
impl ServerAdmin for MyServerAdmin {
    async fn get_server_info(&self, request: Request<AuthRequest>) -> Result<Response<AuthResponse>, Status> {
        let req = request.into_inner();
        match decode::<Claims>(&req.token, &DecodingKey::from_secret(self.config.password.as_ref()), &Validation::default()) {
            Ok(_) => {
                let name = self.config.name.clone();
                let description = self.config.description.clone();
                let status = "Worked".to_string();
                let info = AuthResponse {
                    name: name,
                    description: description,
                    status: status,
                };
                Ok(Response::new(info))
            },
            Err(_) => {
                Err(Status::unauthenticated("Authentication failed"))
            }
        }
    }

    async fn get_cpu_usage(&self, request: Request<AuthRequest>) -> Result<Response<CpuUsageResponse>, Status> {
        let req = request.into_inner();
        match decode::<Claims>(&req.token, &DecodingKey::from_secret(self.config.password.as_ref()), &Validation::default()) {
            Ok(_) => {
                let mut system = self.system.lock().await;
                system.refresh_cpu();
                let cpu_usage = system.global_cpu_info().cpu_usage();
                let cpu_usage_procent = (cpu_usage.clone() * 100.0).round() / 100.0;
                let response = CpuUsageResponse {
                    cpu_usage: cpu_usage as f64,
                    cpu_usage_procent: cpu_usage_procent as f64,
                };
                Ok(Response::new(response))
            },
            Err(_) => {
                Err(Status::unauthenticated("Authentication failed"))
            }
        }
    }

    async fn get_memory_usage(&self, request: Request<AuthRequest>) -> Result<Response<MemoryUsageResponse>, Status> {
        let req = request.into_inner();
        match decode::<Claims>(&req.token, &DecodingKey::from_secret(self.config.password.as_ref()), &Validation::default()) {
            Ok(_) => {
                let mut system = self.system.lock().await;
                system.refresh_memory();
                let total_memory = system.total_memory();
                let used_memory = system.used_memory();
                let free_memory = total_memory - used_memory;
                let total_memory_mb = total_memory.clone() / 1024 / 1024;
                let used_memory_mb = used_memory.clone() / 1024 / 1024;
                let free_memory_mb = free_memory.clone() / 1024 / 1024;
                let total_memory_gb = total_memory_mb.clone() / 1024;
                let used_memory_gb = used_memory_mb.clone() / 1024;
                let free_memory_gb = free_memory_mb.clone() / 1024;
                let kb = MemoryUsage {
                    total_memory: total_memory,
                    used_memory: used_memory,
                    free_memory: free_memory,
                    digital_type: "KB".to_string()
                };
                let mb = MemoryUsage {
                    total_memory: total_memory_mb,
                    used_memory: used_memory_mb,
                    free_memory: free_memory_mb,
                    digital_type: "MB".to_string()
                };
                let gb = MemoryUsage {
                    total_memory: total_memory_gb,
                    used_memory: used_memory_gb,
                    free_memory: free_memory_gb,
                    digital_type: "GB".to_string()
                };
                let mut memories = Vec::new();
                memories.push(kb);
                memories.push(mb);
                memories.push(gb);
                let response = MemoryUsageResponse {
                    memory_usage: memories
                };
                Ok(Response::new(response))
            },
            Err(_) => {
                Err(Status::unauthenticated("Authentication failed"))
            }
        }
    }

    async fn get_disk_usage(&self, request: Request<AuthRequest>) -> Result<Response<DiskUsageResponse>, Status> {
        let req = request.into_inner();
        match decode::<Claims>(&req.token, &DecodingKey::from_secret(self.config.password.as_ref()), &Validation::default()) {
            Ok(_) => {
                let disks = Disks::new_with_refreshed_list();
                let mut disks_info = Vec::new();
                for disk in disks.list() {
                    disks_info.push(DiskUsage {
                        name: disk.name().to_str().unwrap_or_default().to_string(),
                        file_system: disk.file_system().to_str().unwrap_or_default().to_string(),
                        mount_point: disk.mount_point().to_str().unwrap_or_default().to_string(),
                        total_space: disk.total_space() / 1024 / 1024,
                        available_space: disk.available_space() / 1024 / 1024,
                    });
                }
                let info = DiskUsageResponse {
                    disks: disks_info
                };
                Ok(Response::new(info))
            },
            Err(_) => {
                Err(Status::unauthenticated("Authentication failed"))
            }
        }
    }

    async fn get_network_usage(&self, request: Request<AuthRequest>) -> Result<Response<NetworkUsageResponse>, Status> {
        let req = request.into_inner();
        match decode::<Claims>(&req.token, &DecodingKey::from_secret(self.config.password.as_ref()), &Validation::default()) {
            Ok(_) => {
                let networks = Networks::new_with_refreshed_list();
                let total_received: u64 = networks.iter().map(|(_, data)| data.total_received()).sum();
                let total_transmitted: u64 = networks.iter().map(|(_,data)| data.total_transmitted()).sum();
                let kb = NetworkUsage {
                    total_received: total_received,
                    total_transmitted: total_transmitted,
                    digital_type: "KB".to_string()
                };
                let mb = NetworkUsage {
                    total_received: (total_received / 1024 / 1024),
                    total_transmitted: (total_transmitted / 1024 / 1024),
                    digital_type: "MB".to_string()
                };
                let gb = NetworkUsage {
                    total_received: (total_received / 1024 / 1024 / 1024),
                    total_transmitted: (total_transmitted / 1024 / 1024 / 1024),
                    digital_type: "GB".to_string()
                };
                let mut networks = Vec::new();
                networks.push(kb);
                networks.push(mb);
                networks.push(gb);
                let response = NetworkUsageResponse {
                    network_usage: networks
                };
                Ok(Response::new(response))
            },
            Err(_) => {
                Err(Status::unauthenticated("Authentication failed"))
            }
        }
    }

}

pub async fn start_server(config: ServerConfig) -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:5000".parse().unwrap();
    let admin = MyServerAdmin::new(config);

    let reflection_service = Builder::configure()
        .register_encoded_file_descriptor_set(server_client_utility::AUTH_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .add_service(ServerAdminServer::new(admin))
        .add_service(reflection_service)
        .serve_with_shutdown(addr, ctrl_c())
        .await?;

    Ok(())
}

async fn ctrl_c() -> () {
    let _ = signal::ctrl_c().await;
}