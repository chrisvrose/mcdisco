use std::env::{self, set_current_dir};
use std::path::Path;
use std::process::{Child, Command};
use std::sync::Arc;

use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::macros::{command, group, hook};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::model::prelude::{Activity, Ready};
use serenity::prelude::*;
use tokio::io::BufReader;

struct Serv {
    pub child_process: Option<Child>,
}
impl Serv {
    fn new() -> Self {
        Serv {
            child_process: None,
        }
    }
    fn new_with_child(x: Child) -> Self {
        Serv {
            child_process: Some(x),
        }
    }
}
impl TypeMapKey for Serv {
    type Value = Serv;
}

#[group]
#[commands(ping, start, status)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        ctx.set_presence(
            Some(Activity::listening("Your commands to start the server")),
            serenity::model::user::OnlineStatus::Online,
        )
        .await;
        println!("Ready!")
    }
}

#[hook]
async fn unrecognised_command_hook(ctx: &Context, msg: &Message, unrecognised_command_name: &str) {
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

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn start(ctx: &Context, msg: &Message) -> CommandResult {
    let path = env::var("ENV_PATH").expect("ENV_PATH env var missing");
    let script = env::var("RUNSCRIPT").expect("RUNSCRIPT env var missing");
    set_current_dir(Path::new(path.as_str())).expect("Could not change directory");

    let mut child = {
        Command::new(script.as_str())
            // .args(["-jar", "server.jar"])
            .spawn()
    };

    if let Ok(command_child) = child {
        // do it in a block so the lock releases outside the block
        {
            let mut command_lock = ctx.data.write().await;
            command_lock.insert::<Serv>(Serv::new_with_child(command_child));
            command_lock.downgrade()
        };
        // x.
        msg.reply(ctx, format!("Started server!",)).await?;
    } else {
        msg.reply(ctx, format!("Could not start! Refer logs",)).await?;
    }

    Ok(())
}

#[command]
async fn status(ctx: &Context, msg: &Message) -> CommandResult {
    let child_status = {
        let mut x = ctx.data.write().await;
        let mut command = x.get_mut::<Serv>().expect("Could not unwrap data");
        let exit_status = command
            .child_process
            .as_mut()
            .and_then(|v| v.try_wait().expect("It should be ok")); //.expect("An error has occured while checking for return!")

        if let Some(_) = exit_status {
            //we have an exit status, return true
            false
        } else {
            true
        }
    };
    // x.
    msg.reply(
        ctx,
        format!(
            "Server status: {}",
            if child_status { "running" } else { "stopped" }
        ),
    )
    .await?;

    Ok(())
}

// #[command]
// async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
//     msg.reply(ctx, "Stopping server").await?;
//     Ok(())
// }
