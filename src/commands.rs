use serenity::framework::standard::macros::{command, help};
use serenity::framework::standard::{
    help_commands, Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::model::channel::Message;
use serenity::model::prelude::UserId;
use serenity::prelude::*;

use log::{debug, error, trace, warn, info};
use std::collections::HashSet;
use std::env::{self, set_current_dir};

use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::data::Serv;
use crate::server_helper::poll_child;

#[help]
#[individual_command_tip = "Hello!\n\n\
If you want more information about a specific command, just pass the command as argument."]
#[strikethrough_commands_tip_in_guild = ""]
pub async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[command]
#[description("Get a pong message")]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    debug!("Ping command reply for {}", msg.author);
    Ok(())
}

#[command]
#[description("Query the status of the minecraft server")]
pub async fn status(ctx: &Context, msg: &Message) -> CommandResult {
    let child_status = {
        let mut command_lock = ctx.data.write().await;
        !poll_child(&mut command_lock).await
    };

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

#[command]
#[description("Start the minecraft server")]
pub async fn start(ctx: &Context, msg: &Message) -> CommandResult {
    let path = env::var("ENV_PATH").expect("ENV_PATH env var missing");
    let script = env::var("RUNSCRIPT").expect("RUNSCRIPT env var missing");

    let mut command_lock = ctx.data.write().await;
    let has_exit = poll_child(&mut command_lock).await;
    set_current_dir(Path::new(path.as_str())).expect("Could not change directory");
    
    if has_exit{
        let child = {
            Command::new(script.as_str())
                // .args(["-jar", "server.jar"])
                .stdin(Stdio::piped())
                .spawn()
        };
        match child {
            Ok(command_child) => {
                // do it in a block so the lock releases outside the block
    
                command_lock.insert::<Serv>(Serv::new_with_child(command_child));
    
                // x.
                msg.reply(ctx, format!("Started server!",)).await?;
                trace!("Started minecraft server");
            }
            Err(err) => {
                msg.reply(ctx, format!("Could not start! Refer logs",))
                    .await?;
                error!("Could not start server: {}", err.to_string());
            }
        };

    }else{ 
        info!("Tried to start server when already running!");
        msg.reply(ctx, "The server is already running").await?;
    }

    Ok(())
}

#[command]
#[description = "Stop the minecraft server"]
pub async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
    let mut command_lock = ctx.data.write().await;
    let running = poll_child(&mut command_lock).await;
    let x = command_lock
        .get_mut::<Serv>()
        .and_then(|x| x.child_process.as_mut());

    if !running {
        match x {
            Some(child) => {
                let stdin = child.stdin.as_mut().unwrap();
                stdin.write_all(b"stop\n").unwrap();
                // drop(stdin);
            }
            None => {
                warn!("Server has closed since poll - exit now");
            }
        }
        msg.reply(ctx, "Stopping the server").await?;
    } else {
        msg.reply(ctx, "The server is already stopped").await?;
    }
    Ok(())
}
