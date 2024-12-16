use std::env;
use std::error::Error;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::all::Message;
use async_openai::{types::CreateChatCompletionRequestArgs, Client};
use async_openai::types::{ ChatCompletionRequestUserMessageArgs, ChatCompletionRequestSystemMessageArgs };

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!gpt ") {
            let s = msg.content.strip_prefix("!gpt ").unwrap().trim().to_string();

            let client = Client::new();
    
            let messages = vec![
                ChatCompletionRequestSystemMessageArgs::default()
                    .content("You are a helpful assistant.")
                    .build()
                    .map(Into::into),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(s)
                    .build()
                    .map(Into::into),
            ];

            if let Ok(messages) = messages.into_iter().collect::<Result<Vec<_>, _>>() {
                if let Ok(request) = CreateChatCompletionRequestArgs::default()
                    .model("gpt-3.5-turbo")
                    .messages(messages)
                    .max_tokens(40_u32)
                    .build()
                {
                    if let Ok(response) = client.chat().create(request).await {
                        if let Some(choice) = response.choices.first() {
                            if let Some(content) = &choice.message.content {
                                if let Err(why) = msg.channel_id.say(&ctx.http, content).await {
                                    println!("Error sending message: {why:?}");
                                }
                            }
                        }
                    }
                }
            }
        }
    }}


#[tokio::main]async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot
    let mut client = serenity::Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Start listening for events by starting a single shart
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }

    Ok(())
}