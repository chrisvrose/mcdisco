use log::{debug, warn};
use serenity::prelude::TypeMap;
use tokio::sync::RwLockWriteGuard;

use crate::data::Serv;

/// get whether child has exit
pub async fn poll_child<'a>(x: &mut RwLockWriteGuard<'a, TypeMap>) -> bool {
    let child_status = {
        let command = x.get_mut::<Serv>().expect("Could not unwrap data");

        match &mut command.child_process {
            Some(process) => {
                debug!("Waiting for child process");
                let res_return = process.try_wait();
                match res_return {
                    Ok(Some(_exit_code)) => {
                        debug!("Exit code {}", _exit_code);
                        true
                    }
                    Ok(None) => {
                        debug!("Server has not exit yet");
                        false
                    }
                    Err(e) => {
                        warn!("Could not wait for child process: {}", e.to_string());
                        true
                    }
                }
            }
            None => {
                debug!("No child process for status command to query");
                true
            }
        }
    };
    child_status
}
