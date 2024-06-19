use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::fs;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub location: String,
    pub ip_port: String,
    pub description: String,
    pub uuid: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerConfig {
    pub name: String,
    pub location: String,
    pub ip_port: String,
    pub description: String,
    pub password: String,
    pub token: String,
}

impl ServerConfig {
    pub fn new(name: String, location: String, ip_port: String, description: String) -> Self {
        let password = Uuid::new_v4().to_string();

        let claims = Claims {
            location: location.clone(),
            ip_port: ip_port.clone(),
            description: description.clone(),
            uuid: password.clone(),
            exp: 2000000000, // expiration time
        };

        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(password.as_ref()))
            .expect("Failed to encode JWT");

        ServerConfig {
            name,
            location,
            ip_port,
            description,
            password,
            token,
        }
    }

    pub fn load_from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_data = fs::read_to_string(file_path)?;
        let config = serde_json::from_str(&config_data)?;
        Ok(config)
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config_data = serde_json::to_string(self)?;
        fs::write(file_path, config_data)?;
        Ok(())
    }
}