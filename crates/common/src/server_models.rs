use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub save: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterResponse {
    pub id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRegisterRequest {
    pub id: Uuid,
    pub save: Vec<u8>,
}

#[derive(Debug, Deserialize)]
pub struct GetRegisterRequest {
    pub id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct GetRegisterResponse {
    pub id: Uuid,
    pub save: Vec<u8>,
    pub updated: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct GetRegistersResponse {
    pub registers: Vec<GetRegisterResponse>,
    pub count: usize,
}
