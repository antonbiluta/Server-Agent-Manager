use jsonwebtoken::{decode, DecodingKey, Validation};
use tokio::signal::{self};
use tonic::{transport::Server, Request, Response, Status};
use tonic_reflection::server::Builder;
use crate::config::{Claims, ServerConfig};
use server_client_utility::auth_protocol::server_admin_server::{ServerAdmin, ServerAdminServer};
use server_client_utility::auth_protocol::{AuthRequest, AuthResponse};

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
}

#[tonic::async_trait]
impl ServerAdmin for MyServerAdmin {
    async fn get_server_info(&self, request: Request<AuthRequest>) -> Result<Response<AuthResponse>, Status> {
        let req = request.into_inner();
        let mut name = "".to_string();
        let mut description = "".to_string();
        let mut status = "failed".to_string();
        match decode::<Claims>(&req.token, &DecodingKey::from_secret(self.config.password.as_ref()), &Validation::default()) {
            Ok(_) => {
                name = self.config.name.clone();
                description = self.config.description.clone();
                status = "OK".to_string();
            },
            Err(_) => {}
        };
        let info = AuthResponse {
            name: name,
            description: description,
            status: status,
        };
        Ok(Response::new(info))
    }
}

pub async fn start_server(config: ServerConfig) -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:5000".parse().unwrap();
    let admin = MyServerAdmin { config };

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