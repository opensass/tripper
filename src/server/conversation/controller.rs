#![allow(unused)]
#![allow(dead_code)]

use bson::doc;
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::server::auth::controller::auth;
use crate::server::common::response::SuccessResponse;
use crate::server::conversation::model::Conversation;
use crate::server::conversation::model::Message;
use crate::server::conversation::request::CreateConversationRequest;
use crate::server::conversation::request::GetConversationsRequest;
use crate::server::conversation::request::GetMessagesRequest;
use crate::server::conversation::request::SendQueryRequest;
use crate::server::conversation::response::ConversationResponse;
use crate::server::conversation::response::ConversationsListResponse;
use crate::server::conversation::response::MessageResponse;
use crate::server::conversation::response::MessagesListResponse;
use crate::server::trip::model::Detail;
use crate::server::trip::model::Trip;
#[cfg(feature = "server")]
use aws_config::BehaviorVersion;
#[cfg(feature = "server")]
use aws_sdk_bedrockruntime::{
    operation::converse::{ConverseError, ConverseOutput},
    types::{ContentBlock, ConversationRole, Message as BedrockMessage},
    Client,
};
use bson::oid::ObjectId;
use chrono::prelude::*;
use futures_util::TryStreamExt;
use std::env;
#[cfg(feature = "server")]
use {crate::ai::get_ai, crate::db::get_client};

#[derive(Debug)]
#[cfg(feature = "server")]
pub struct BedrockConverseError(pub String);
#[cfg(feature = "server")]
impl std::fmt::Display for BedrockConverseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Can't invoke '{}'. Reason: {}",
            "anthropic.claude-3-haiku-20240307-v1:0", self.0
        )
    }
}
#[cfg(feature = "server")]
impl std::error::Error for BedrockConverseError {}
#[cfg(feature = "server")]
impl From<&str> for BedrockConverseError {
    fn from(value: &str) -> Self {
        BedrockConverseError(value.to_string())
    }
}
#[cfg(feature = "server")]
impl From<&ConverseError> for BedrockConverseError {
    fn from(value: &ConverseError) -> Self {
        BedrockConverseError::from(match value {
            ConverseError::ModelTimeoutException(_) => "Model took too long",
            ConverseError::ModelNotReadyException(_) => "Model is not ready",
            _ => "Unknown",
        })
    }
}

#[server]
pub async fn create_conversation(
    req: CreateConversationRequest,
) -> Result<ConversationResponse, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;
    let db_client = get_client().await;
    let db = db_client
        .database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let conversation_collection = db.collection::<Conversation>("conversations");

    let trip_id =
        ObjectId::parse_str(&req.trip_id).map_err(|_| ServerFnError::new("Invalid trip ID"))?;

    let conversation = Conversation {
        id: ObjectId::new(),
        user: user.id,
        trip: trip_id,
        title: req.title,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    conversation_collection
        .insert_one(conversation.clone())
        .await
        .map_err(|e| ServerFnError::new(&e.to_string()))?;
    Ok(ConversationResponse {
        status: "success".to_string(),
        data: conversation,
    })
}

#[server]
pub async fn get_conversations(
    req: GetConversationsRequest,
) -> Result<ConversationsListResponse, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let db_client = get_client().await;
    let db = db_client
        .database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let conversation_collection = db.collection::<Conversation>("conversations");

    let filter = doc! {"user": user.id, "trip": req.trip_id};
    let cursor = conversation_collection
        .find(filter)
        .await
        .map_err(|e| ServerFnError::new(&e.to_string()))?;
    let conversations: Vec<Conversation> = cursor
        .try_collect()
        .await
        .map_err(|e| ServerFnError::new(&e.to_string()))?;

    Ok(ConversationsListResponse {
        status: "success".to_string(),
        data: conversations,
    })
}

#[server]
pub async fn save_message_to_db(message: Message) -> Result<(), ServerFnError> {
    let db_client = get_client().await;
    let db = db_client
        .database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let messages_collection = db.collection::<Message>("messages");

    messages_collection
        .insert_one(message)
        .await
        .map_err(|e| ServerFnError::new(&e.to_string()))?;
    Ok(())
}

#[server]
pub async fn get_messages(req: GetMessagesRequest) -> Result<MessagesListResponse, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let db_client = get_client().await;
    let db = db_client
        .database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let messages_collection = db.collection::<Message>("messages");

    let filter = doc! {"conversation": req.conversation_id};
    let cursor = messages_collection
        .find(filter)
        .await
        .map_err(|e| ServerFnError::new(&e.to_string()))?;
    let messages: Vec<Message> = cursor
        .try_collect()
        .await
        .map_err(|e| ServerFnError::new(&e.to_string()))?;

    Ok(MessagesListResponse {
        status: "success".to_string(),
        data: messages,
    })
}

#[server]
pub async fn send_query_to_bedrock(
    req: SendQueryRequest,
) -> Result<MessageResponse, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let messages_collection = db.collection::<Message>("messages");
    let trip_collection = db.collection::<Trip>("trips");
    let details_collection = db.collection::<Detail>("details");

    let mut client = get_ai().await.lock().await;

    let trip_id =
        ObjectId::parse_str(&req.trip).map_err(|_| ServerFnError::new("Invalid trip ID"))?;

    let trip = trip_collection
        .find_one(doc! { "_id": trip_id, "user": user.id })
        .await?
        .ok_or(ServerFnError::new("Trip not found"))?;

    let detail_id =
        ObjectId::parse_str(&req.detail).map_err(|_| ServerFnError::new("Invalid detail ID"))?;

    let detail = details_collection
        .find_one(doc! { "_id": detail_id })
        .await?
        .ok_or(ServerFnError::new("Detail not found"))?;

    let system_prompt = format!(
        "
        **System Prompt (SP):** You are a knowledgeable assistant specializing in providing in-depth responses based on specific trip details. You understand the structure, themes, and content of trips, and you answer questions with context and precision.
        Generate your response as HTML-formatted response with examples, links and images, based on the query: '{user_query}'. \
        Each section should be structured with appropriate HTML tags, including <h1> for the main title, \
        <h2> for detail titles, <h3> for subheadings, and <p> for paragraphs. \
        Include well-organized, readable content that aligns with the trip's title {trip_title}, ensuring each section is \
        clear and logically flows from one to the next. Avoid markdown format entirely, and provide inline HTML styling \
        if necessary to enhance readability. The HTML content should be well-formatted, semantically correct, and \
        cover all relevant subtopics in depth to create an engaging reading experience.

        **Context Information:**
        - Trip Title: '{trip_title}'
        - Detail: '{detail}'
    
        **User Query:** The user has a question related to the trip detail provided. Use the context of the trip and detail to respond accurately and engage thoughtfully.
    
        **Prompt (P):** Answer the user's question in detail, focusing on information specific to the detail content '{detail}' from the trip '{trip_title}'. 
        - Explain complex concepts in an accessible way if the user’s query requires it.
        - Where applicable, relate your answer back to key themes and ideas presented in this detail.
        - If the detail has distinct characters, events, or themes, draw on these to enhance your response.
        - Provide direct and actionable information if the question is specific, or a comprehensive overview if the question is broad.
    
        **Expected Format (EF):**
        - Begin with a brief introduction if the question pertains to a major theme or character in the detail.
        - Answer in a clear, step-by-step, or structured format when applicable.
        - For complex queries, summarize the response in the last sentence to ensure clarity for the user.
    
        Make sure to always return back with html formmatted text and not empty response. If the user asks to translate a detail, always respond with the corresponding translation and never reject the request.

        **Roleplay (RP):** Act as a well-read, insightful assistant dedicated to enhancing the reader’s understanding of the material in this trip detail. Aim to be both informative and engaging in your response.
    
        **User Query:** '{user_query}'
        ",
        trip_title = trip.title,
        detail = detail.html,
        user_query = req.query
    );

    let response = client
        .converse()
        .model_id("anthropic.claude-3-haiku-20240307-v1:0")
        .messages(
            BedrockMessage::builder()
                .role(ConversationRole::User)
                .content(ContentBlock::Text(system_prompt.to_string()))
                .build()
                .map_err(|_| "failed to build message")
                .unwrap(),
        )
        .send()
        .await;

    match response {
        Ok(output) => {
            let text = get_converse_output_text(output)?;
            let response_message = Message {
                id: ObjectId::new(),
                conversation: req.conversation_id,
                sender: "gemini".to_string(),
                content: text.clone(),
                timestamp: Utc::now(),
            };

            messages_collection
                .insert_one(response_message.clone())
                .await
                .map_err(|e| ServerFnError::new(&e.to_string()))?;

            Ok(MessageResponse {
                status: "success".to_string(),
                data: response_message,
            })
        }
        Err(e) => Err(e
            .as_service_error()
            .map(BedrockConverseError::from)
            .unwrap_or_else(|| BedrockConverseError("Unknown service error".into()))
            .into()),
    }
}

#[cfg(feature = "server")]
pub fn get_converse_output_text(output: ConverseOutput) -> Result<String, BedrockConverseError> {
    let text = output
        .output()
        .ok_or("no output")?
        .as_message()
        .map_err(|_| "output not a message")?
        .content()
        .first()
        .ok_or("no content in message")?
        .as_text()
        .map_err(|_| "content is not text")?
        .to_string();
    Ok(text)
}
