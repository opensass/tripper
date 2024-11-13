use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoreTripRequest {
    pub token: String,
    pub title: String,
    pub subtitle: String,
    pub content: String,
    pub trip_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateTripContentRequest {
    pub trip_id: String,
    pub new_content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompleteTripRequest {
    pub trip_id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetTripsForUserRequest {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerateTripRequest {
    pub title: String,
    pub subtitle: String,
    pub token: String,
    pub model: String,
    pub subtopics: u64,
    pub details: u64,
    pub language: String,
    pub max_length: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerateDetailContentRequest {
    pub detail_title: String,
    pub detail_id: ObjectId,
    pub trip_title: String,
    pub language: String,
    pub model: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetTripForUserRequest {
    pub token: String,
    pub trip_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AIRequest {
    pub token: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetDetailContentRequest {
    pub trip_id: String,
}
