use aws_config::BehaviorVersion;
use aws_sdk_bedrockruntime::{
    error::ProvideErrorMetadata,
    operation::converse_stream::ConverseStreamError,
    types::{
        error::ConverseStreamOutputError, ContentBlock, ConversationRole,
        ConverseStreamOutput as ConverseStreamOutputType, Message,
    },
    Client,
};
use std::env;
use tokio::sync::{Mutex, OnceCell};

const CLAUDE_REGION: &str = "us-east-1";

static AI: OnceCell<Mutex<Client>> = OnceCell::const_new();

async fn init_ai_with_model() -> &'static Mutex<Client> {
    let sdk_config = aws_config::defaults(BehaviorVersion::latest())
        .region(CLAUDE_REGION)
        .load()
        .await;

    AI.get_or_init(|| async {
        let client = Client::new(&sdk_config);
        Mutex::new(client)
    })
    .await
}

pub async fn get_ai() -> &'static Mutex<Client> {
    init_ai_with_model().await
}
