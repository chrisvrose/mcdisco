use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::prelude::*;

use std::env::{self, set_current_dir};

use std::path::Path;
use std::process::Command;

use crate::data::Serv;

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
pub async fn status(ctx: &Context, msg: &Message) -> CommandResult {
    let child_status = {
        let mut x = ctx.data.write().await;
        let mut command = x.get_mut::<Serv>().expect("Could not unwrap data");
        
        match &mut command.child_process {
            Some(process) => {
                let res_return = process.try_wait();
                match res_return {
                    Ok(Some(_exit_code)) => true,
                    Ok(None) => false,
                    Err(_) => true,
                }
            }
            None => false,
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

#[command]
pub async fn start(ctx: &Context, msg: &Message) -> CommandResult {
    let path = env::var("ENV_PATH").expect("ENV_PATH env var missing");
    let script = env::var("RUNSCRIPT").expect("RUNSCRIPT env var missing");
    set_current_dir(Path::new(path.as_str())).expect("Could not change directory");

    let child = {
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
        msg.reply(ctx, format!("Could not start! Refer logs",))
            .await?;
    }

    Ok(())
}
