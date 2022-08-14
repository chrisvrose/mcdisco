use serenity::async_trait;
use serenity::framework::standard::macros::{group, hook};
use serenity::framework::standard::StandardFramework;
use serenity::model::channel::Message;
use serenity::model::prelude::{Activity, Ready};
use serenity::prelude::*;

use std::env;

mod commands;
use commands::*;

mod data;
use data::*;

#[group]
#[commands(ping, start, status)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        ctx.set_presence(
            Some(Activity::listening("~help")),
            serenity::model::user::OnlineStatus::Online,
        )
        .await;
        println!("Ready!")
    }
}

#[hook]
async fn unrecognised_command_hook(_ctx: &Context, msg: &Message, unrecognised_command_name: &str) {
    println!(
        "A user named {:?} tried to execute an unknown command: {}",
        msg.author.name, unrecognised_command_name
    );
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .unrecognised_command(unrecognised_command_hook)
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .type_map_insert::<Serv>(Serv::new())
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

