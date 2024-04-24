use std::{env, sync::Arc};

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use smart_notes_cli::{get_note_service, Note, NoteService};
use tokio;

struct Handler {
    service: Arc<Mutex<NoteService>>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
        if msg.content == "!notes" {
            let notes = self.read_notes().await;
            for (i, note) in notes.iter().enumerate() {
                let note_str = format!("----------------- note {i} ------------------\n{note}");
                if let Err(why) = msg.channel_id.say(&ctx.http, note_str).await {
                    println!("Error sending message: {why:?}")
                }
            }
        }
    }
}
impl Handler {
    async fn read_notes(&self) -> Vec<Note> {
        let notes = self.service.lock().await.list_all_notes().unwrap();
        notes
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let service = get_note_service()?;
    let service = Arc::new(Mutex::new(service));
    let handler = Handler { service };
    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
    Ok(())
}
