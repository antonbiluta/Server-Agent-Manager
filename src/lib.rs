#[path = "auth.protocol.rs"]
pub mod auth_protocol;

pub const AUTH_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("auth_descriptor");