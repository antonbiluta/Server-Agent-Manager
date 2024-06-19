// mod server_admin {
//     tonic::include_proto!("Auth");
// }

mod config;
mod location;
mod server;

use crate::config::ServerConfig;
use crate::location::get_location;
use crate::server::start_server;
use std::io::{self, Write};

fn prompt_user(prompt: &str, default: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    if input.is_empty() {
        default.to_string()
    } else {
        input.to_string()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_file = "server_config.json";

    let config: ServerConfig = if let Ok(config) = ServerConfig::load_from_file(config_file) {
        config
    } else {
        let (ip, country, city) = get_location().await.unwrap_or_else(|_| ("0.0.0.0".to_string(), "Unknown Country".to_string(), "Unknown City".to_string()));

        let name = prompt_user("Enter server name (default: MyServer)", "MyServer");
        let location = prompt_user(&format!("Enter your server location (default: {}): ", format!("{}, {}", country, city)), &format!("{}, {}", country, city));
        let ip_port = prompt_user(&format!("Enter the address where the server is accessible on the Internet (default: {}:5000): ", ip), &format!("{}:5000", ip));
        let description = prompt_user("Provide a description of the server (Optional): ", "");

        let config = ServerConfig::new(name, location, ip_port, description);
        config.save_to_file(config_file)?;

        config
    };


    println!("Your token is: scu://{}@{}", config.token, config.ip_port);

    start_server(config).await?;

    Ok(())
}