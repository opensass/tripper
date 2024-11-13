#![allow(unused)]
#![allow(dead_code)]

use bson::doc;
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::server::auth::controller::auth;
use crate::server::trip::model::Trip;
use crate::server::trip::model::Detail;
use crate::server::trip::request::AIRequest;
use crate::server::trip::request::CompleteTripRequest;
use crate::server::trip::request::GenerateTripRequest;
use crate::server::trip::request::GenerateDetailContentRequest;
use crate::server::trip::request::GetTripForUserRequest;
use crate::server::trip::request::GetTripsForUserRequest;
use crate::server::trip::request::GetDetailContentRequest;
use crate::server::trip::request::StoreTripRequest;
use crate::server::trip::request::UpdateTripContentRequest;
use crate::server::trip::response::TripResponse;
use crate::server::trip::response::GenerateTripOutlineResponse;
use crate::server::trip::response::{
    AIUsageStats, AnalyticsData, EngagementStats, PredictiveStats,
};
use crate::server::common::response::SuccessResponse;
use std::env;

use bson::oid::ObjectId;
use chrono::prelude::*;
use futures_util::StreamExt;
use futures_util::TryStreamExt;
use regex::Regex;
#[cfg(feature = "server")]
use {
    crate::ai::get_ai,
    crate::db::get_client,
    crate::unsplash::get_unsplash_client,
    crate::server::conversation::controller::BedrockConverseError,
    crate::server::conversation::controller::get_converse_output_text,
    http_api_isahc_client::{Client as _, IsahcClient},
    rand::thread_rng,
    rand::Rng,
    unsplash_api::endpoints::common::EndpointRet,
    unsplash_api::endpoints::search_photos::SearchPhotos,
    unsplash_api::endpoints::search_photos::SearchPhotosResponseBodyOkJson,
    unsplash_api::objects::pagination::Pagination,
    unsplash_api::objects::rate_limiting::RateLimiting,
};

#[cfg(feature = "server")]
use aws_config::BehaviorVersion;
#[cfg(feature = "server")]
use aws_sdk_bedrockruntime::{
    operation::converse::{ConverseError, ConverseOutput},
    types::{ContentBlock, ConversationRole, Message as BedrockMessage},
    Client,
};

#[server]
pub async fn store_trip(
    req: StoreTripRequest,
) -> Result<SuccessResponse<TripResponse>, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let trip_collection = db.collection::<Trip>("trips");

    let photo_url = fetch_cover(req.title.to_string()).await?;

    let new_trip = Trip {
        id: ObjectId::new(),
        user: user.id,
        title: req.title,
        subtitle: Some(req.subtitle),
        trip_type: req.trip_type,
        cover: photo_url,
        completed: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    trip_collection.insert_one(new_trip.clone()).await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: TripResponse { id: new_trip.id },
    })
}

#[server]
pub async fn fetch_cover(topic: String) -> Result<Option<String>, ServerFnError> {
    let client = get_unsplash_client().await.lock().await;

    let search_photos = SearchPhotos::new(
        &env::var("UNSPLASH_API_KEY").expect("UNSPLASH_API_KEY must be set."),
        topic,
    );

    let response: EndpointRet<(SearchPhotosResponseBodyOkJson, Pagination, RateLimiting)> =
        client.respond_endpoint(&search_photos).await?;

    let mut extracted_data = Vec::new();

    if let EndpointRet::Ok((ok_json, _pagination, _rate_limiting)) = response {
        for photo in ok_json.results {
            let image_url = photo.urls.regular.to_string();

            extracted_data.push(image_url);
        }
    } else {
        tracing::error!("Unexpected response type");
    }

    if extracted_data.is_empty() {
        return Ok(None);
    }

    let mut rng = thread_rng();
    let random_index = rng.gen_range(0..extracted_data.len());
    Ok(Some(extracted_data[random_index].clone()))
}

#[server]
pub async fn update_detail_content(
    req: UpdateTripContentRequest,
) -> Result<SuccessResponse<String>, ServerFnError> {
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let trip_collection = db.collection::<Trip>("trips");

    let trip_id =
        ObjectId::parse_str(&req.trip_id).map_err(|_| ServerFnError::new("Invalid trip ID"))?;

    trip_collection
        .update_one(
            doc! { "_id": trip_id },
            doc! { "$set": { "content": req.new_content, "updatedAt": Utc::now() } },
        )
        .await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: "Trip updated successfully".into(),
    })
}

#[server]
pub async fn complete_trip(
    req: CompleteTripRequest,
) -> Result<SuccessResponse<String>, ServerFnError> {
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let trip_collection = db.collection::<Trip>("trips");

    trip_collection
        .update_one(
            doc! { "_id": req.trip_id },
            doc! { "$set": { "completed": true, "updatedAt": Utc::now() } },
        )
        .await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: "Trip marked as completed".into(),
    })
}

#[server]
pub async fn get_trips_for_user(
    req: GetTripsForUserRequest,
) -> Result<SuccessResponse<Vec<Trip>>, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let trip_collection = db.collection::<Trip>("trips");

    let trips = trip_collection
        .find(doc! { "user": user.id })
        .await?
        .try_collect()
        .await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: trips,
    })
}

#[server]
pub async fn get_trip_for_user(
    req: GetTripForUserRequest,
) -> Result<SuccessResponse<Trip>, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let trip_collection = db.collection::<Trip>("trips");

    let trip_id =
        ObjectId::parse_str(&req.trip_id).map_err(|_| ServerFnError::new("Invalid trip ID"))?;

    let trip = trip_collection
        .find_one(doc! { "_id": trip_id, "user": user.id })
        .await?
        .ok_or(ServerFnError::new("Trip not found"))?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: trip,
    })
}

#[server]
pub async fn generate_trip_outline(
    req: GenerateTripRequest,
) -> Result<SuccessResponse<GenerateTripOutlineResponse>, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let mut client = get_ai().await.lock().await;

    let system_prompt = format!(
        "
        **System Prompt (SP):** You are an expert in trip creation, generating a structured outline.

        **Prompt (P):** Generate an outline for a trip titled '{title}', with subtitle '{subtitle}'. Main topic is '{title}' in {language}. The trip should contain {details} details covering {subtopics} subtopics. Provide an estimated duration for each detail.

        **Expected Format (EF):**
        ### Detail [number]: [Detail Title]
        **Estimated Duration:** [Duration] minutes

        **Roleplay (RP):** As a trip editor, create an engaging outline.
        ",
        title = req.title,
        subtitle = req.subtitle,
        details = req.details,
        subtopics = req.subtopics,
        language = req.language,
    );

    let mut outline = "".to_string();

    let response = client
        .converse()
        .model_id("anthropic.claude-3-haiku-20240307-v1:0")
        .messages(
            BedrockMessage::builder()
                .role(ConversationRole::User)
                .content(ContentBlock::Text(system_prompt.to_string()))
                .build()
                .map_err(|_| "failed to build message").unwrap(),
        )
        .send()
        .await;

    match response {
        Ok(output) => {
            outline = get_converse_output_text(output)?;
        },
        Err(e) => {
            return Err(e
                .as_service_error()
                .map(BedrockConverseError::from)
                .unwrap_or_else(|| BedrockConverseError("Unknown service error".into())).into());
        }
    }

    let db_client = get_client().await;
    let db = db_client
        .database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let trip_collection = db.collection::<Trip>("trips");

    let photo_url = fetch_cover(req.title.clone()).await?;

    let trip = Trip {
        id: ObjectId::new(),
        user: user.id,
        title: req.title.clone(),
        subtitle: Some(req.subtitle.clone()),
        trip_type: Some(req.title.clone()),
        completed: false,
        cover: photo_url,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    trip_collection.insert_one(trip.clone()).await?;

    let details = parse_outline(outline.clone(), trip.id, req.language)?;

    let details_collection = db.collection::<Detail>("details");
    details_collection.insert_many(details.clone()).await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: GenerateTripOutlineResponse {
            trip: trip.clone(),
            details: details.clone(),
        },
    })
}

fn parse_outline(
    outline: String,
    trip_id: ObjectId,
    language: String,
) -> Result<Vec<Detail>, ServerFnError> {
    let mut details = Vec::new();

    let re =
        Regex::new(r"### Detail (\d+):\s*(.*?)\s*\n\*\*Estimated Duration:\*\*\s*(\d+)\s*minutes")
            .unwrap();

    let mut current_position = 0;
    while let Some(caps) = re.captures(&outline[current_position..]) {
        let title = &caps[2];
        let estimated_duration = caps[3].parse().unwrap_or(0);

        let next_detail_pos = outline[current_position..]
            .find("### Detail ")
            .unwrap_or(outline.len());

        let detail_content = &outline[current_position..current_position + next_detail_pos];

        let bullet_points_re = Regex::new(r"\* .+").unwrap();
        let bullet_points = bullet_points_re
            .find_iter(detail_content)
            .map(|mat| mat.as_str())
            .collect::<Vec<&str>>()
            .join("\n");

        details.push(Detail {
            id: ObjectId::new(),
            trip_id,
            title: title.to_string(),
            estimated_duration,
            html: String::new(),
            completed: false,
            language: language.clone(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });

        current_position += next_detail_pos + detail_content.len();
    }

    Ok(details)
}

#[server]
pub async fn generate_detail_content(
    req: GenerateDetailContentRequest,
) -> Result<SuccessResponse<String>, ServerFnError> {
    let mut client = get_ai().await.lock().await;

    let system_prompt = format!(
        "
        **System Prompt (SP):** You are writing detailed content for a trip detail.

        **Prompt (P):** Write content for detail '{detail_title}' of the trip '{trip_title}' in {language}. Ensure clarity, detailed explanations, and structured markdown.

        **Expected Format (EF):**
        - detailed markdown format for this detail.

        **Roleplay (RP):** Provide as much educational content as possible.
        ",
        detail_title = req.detail_title,
        trip_title = req.trip_title,
        language = req.language,
    );

    let mut markdown = "".to_string();

    let response = client
        .converse()
        .model_id("anthropic.claude-3-haiku-20240307-v1:0")
        .messages(
            BedrockMessage::builder()
                .role(ConversationRole::User)
                .content(ContentBlock::Text(system_prompt.to_string()))
                .build()
                .map_err(|_| "failed to build message").unwrap(),
        )
        .send()
        .await;
    
    match response {
        Ok(output) => {
            markdown = get_converse_output_text(output)?;
        },
        Err(e) => {
            return Err(e
                .as_service_error()
                .map(BedrockConverseError::from)
                .unwrap_or_else(|| BedrockConverseError("Unknown service error".into())).into());
        }
    }

    let content_prompt = format!(
        "Generate a comprehensive HTML-formatted trip detail with examples, links and images, based on the outline: '{}' in {language}. \
        Each section should be structured with appropriate HTML tags, including <h1> for the main title, \
        <h2> for detail titles, <h3> for subheadings, and <p> for paragraphs. \
        Include well-organized, readable content that aligns with the trip's outline, ensuring each section is \
        clear and logically flows from one to the next. Avoid markdown format entirely, and provide inline HTML styling \
        if necessary to enhance readability. The HTML content should be well-formatted, semantically correct, and \
        cover all relevant subtopics in depth to create an engaging reading experience. \
        Make sure to always return back with html formmatted text and not empty response.
        ",
        markdown.clone(),
        language = req.language,
    );

    let mut html = "".to_string();

    let response = client
        .converse()
        .model_id("anthropic.claude-3-haiku-20240307-v1:0")
        .messages(
            BedrockMessage::builder()
                .role(ConversationRole::User)
                .content(ContentBlock::Text(content_prompt.to_string()))
                .build()
                .map_err(|_| "failed to build message").unwrap(),
        )
        .send()
        .await;
    
    match response {
        Ok(output) => {
            html = get_converse_output_text(output)
            .map_err(ServerFnError::new)?
            .trim_start_matches("```html")
            .trim_end_matches("```")
            .trim()
            .to_string();
        },
        Err(e) => {
            return Err(e
                .as_service_error()
                .map(BedrockConverseError::from)
                .unwrap_or_else(|| BedrockConverseError("Unknown service error".into())).into());
        }
    }

    html = update_detail_content(UpdateTripContentRequest {
        trip_id: req.detail_id.to_string(),
        new_content: html.clone(),
    }).await?.data;

    Ok(SuccessResponse {
        status: "success".into(),
        data: html,
    })
}

#[server]
pub async fn fetch_analytics_data(
    token: String,
) -> Result<SuccessResponse<AnalyticsData>, ServerFnError> {
    let user = auth(token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));

    let trips_collection = db.collection::<Trip>("trips");
    let details_collection = db.collection::<Detail>("details");

    // Engagement Metrics
    let total_trips = trips_collection
        .count_documents(doc! { "user": user.id })
        .await?;
    let mut total_details = 0;
    if total_trips > 0 {
        total_details = details_collection.count_documents(doc! {}).await?;
    }
    let avg_details_per_trip = if total_trips > 0 {
        total_details as f64 / total_trips as f64
    } else {
        0.0
    };

    let mut total_estimated_duration = 0.0;
    let total_ai_details = total_details as u64;

    // AI Usage Metrics
    if total_trips > 0 {
        let total_estimated_duration: u64 = details_collection
            .aggregate(vec![
                doc! { "$group": { "_id": null, "total_duration": { "$sum": "$estimated_duration" } } },
            ])
            .await?
            .next()
            .await
            .and_then(|doc| doc.ok()?.get_i64("total_duration").ok())
            .unwrap_or(0) as u64;
    }

    let avg_gen_time = if total_ai_details > 0 {
        total_estimated_duration as f64 / total_ai_details as f64
    } else {
        0.0
    };

    let success_rate = 100.0;

    // Trending Topic
    let trending_topic = trips_collection
        .aggregate(vec![
            doc! { "$match": { "user_id": user.id } },
            doc! { "$group": { "_id": "$title", "count": { "$sum": 1 } } },
            doc! { "$sort": { "count": -1 } },
            doc! { "$limit": 1 },
        ])
        .await?
        .next()
        .await
        .and_then(|doc| doc.ok()?.get_str("_id").ok().map(|s| s.to_string()))
        .unwrap_or_else(|| "Unknown".to_string());

    // Projected Growth
    let monthly_trip_growth = trips_collection
        .aggregate(vec![
            doc! { "$match": { "user_id": user.id } },
            doc! { "$group": {
                "_id": { "month": { "$month": "$created_at" }, "year": { "$year": "$created_at" } },
                "count": { "$sum": 1 }
            }},
            doc! { "$sort": { "_id.year": 1, "_id.month": 1 } },
        ])
        .await?;

    let growth_rates: Vec<f64> = monthly_trip_growth
        .collect::<Vec<_>>()
        .await
        .windows(2)
        .filter_map(|window| {
            if let (Ok(prev), Ok(curr)) = (window[0].as_ref(), window[1].as_ref()) {
                let prev_count = prev
                    .get_document("count")
                    .unwrap_or(&doc! {})
                    .get_i32("count")
                    .unwrap_or(1) as f64;
                let curr_count = curr
                    .get_document("count")
                    .unwrap_or(&doc! {})
                    .get_i32("count")
                    .unwrap_or(1) as f64;
                Some(((curr_count - prev_count) / prev_count) * 100.0)
            } else {
                None
            }
        })
        .collect();

    let projected_growth = growth_rates.last().cloned().unwrap_or(0.0);

    Ok(SuccessResponse {
        status: "success".into(),
        data: AnalyticsData {
            engagement: EngagementStats {
                total_trips,
                total_details,
                avg_details_per_trip,
            },
            ai_usage: AIUsageStats {
                total_ai_details,
                avg_gen_time,
                success_rate,
            },
            predictions: PredictiveStats {
                trending_genre: trending_topic,
                projected_growth,
            },
        },
    })
}


#[server]
pub async fn get_details_for_trip(
    req: GetDetailContentRequest,
) -> Result<SuccessResponse<Vec<Detail>>, ServerFnError> {
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let trip_collection = db.collection::<Detail>("details");

    let trip_object_id =
        ObjectId::parse_str(&req.trip_id).map_err(|_| ServerFnError::new("Invalid trip ID"))?;

    let mut details = trip_collection
        .find(doc! { "trip_id": trip_object_id })
        .await?
        .try_collect::<Vec<Detail>>()
        .await?;

    for detail in details.iter_mut() {
        if detail.html.is_empty() {
            let markdown_content = detail.html.clone();

            let content_prompt = format!(
                "Generate a comprehensive HTML-formatted trip trip with examples, links and images, based on the outline: '{}' in {language}. \
                Each section should be structured with appropriate HTML tags, including <h1> for the main title, \
                <h2> for trip titles, <h3> for subheadings, and <p> for paragraphs. \
                Include well-organized, readable content that aligns with the trip's outline, ensuring each section is \
                clear and logically flows from one to the next. Avoid markdown format entirely, and provide inline HTML styling \
                if necessary to enhance readability. The HTML content should be well-formatted, semantically correct, and \
                cover all relevant subtopics in depth to create an engaging reading experience. \
                Make sure to always return back with html formmatted text and not empty response.",
                markdown_content,
                language = detail.language,
            );

            let mut ai_client = get_ai().await.lock().await;

            let mut html_content = "".to_string();

            let response = ai_client
                .converse()
                .model_id("anthropic.claude-3-haiku-20240307-v1:0")
                .messages(
                    BedrockMessage::builder()
                        .role(ConversationRole::User)
                        .content(ContentBlock::Text(content_prompt.to_string()))
                        .build()
                        .map_err(|_| "failed to build message").unwrap(),
                )
                .send()
                .await;
            
            match response {
                Ok(output) => {
                    html_content = get_converse_output_text(output)
                    .map_err(ServerFnError::new)?
                    .trim_start_matches("```html")
                    .trim_end_matches("```")
                    .trim()
                    .to_string();
                },
                Err(e) => {
                    return Err(e
                        .as_service_error()
                        .map(BedrockConverseError::from)
                        .unwrap_or_else(|| BedrockConverseError("Unknown service error".into())).into());
                }
            }

            trip_collection
                .update_one(
                    doc! { "_id": detail.id },
                    doc! { "$set": { "html": html_content.clone(), "updatedAt": Utc::now() } },
                )
                .await?;

            detail.html = html_content;
        }
    }

    Ok(SuccessResponse {
        status: "success".into(),
        data: details,
    })
}
