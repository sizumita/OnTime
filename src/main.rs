use std::env;
use serenity::Client;
use serenity::client::bridge::gateway::GatewayIntents;

mod event;
mod command;

struct Handler;

#[tokio::main]
async fn main() {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    let token = env::var("DISCORD_BOT_TOKEN").expect("invalid token");

    let application_id: u64 = env::var("DISCORD_APPLICATION_ID")
        .expect("Expected an application id in the environment")
        .parse()
        .expect("application id is not a valid id");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .application_id(application_id)
        .intents(GatewayIntents::GUILDS | GatewayIntents::GUILD_MEMBERS)

        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why)
    }
}
