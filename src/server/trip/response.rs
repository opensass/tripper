use crate::server::trip::model::Trip;
use crate::server::trip::model::Detail;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TripResponse {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerateTripOutlineResponse {
    pub details: Vec<Detail>,
    pub trip: Trip,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AnalyticsData {
    pub engagement: EngagementStats,
    pub ai_usage: AIUsageStats,
    pub predictions: PredictiveStats,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct EngagementStats {
    pub total_trips: u64,
    pub total_details: u64,
    pub avg_details_per_trip: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AIUsageStats {
    pub total_ai_details: u64,
    pub avg_gen_time: f64,
    pub success_rate: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PredictiveStats {
    pub trending_genre: String,
    pub projected_growth: f64,
}
