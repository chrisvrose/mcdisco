use std::process::Child;

use serenity::prelude::TypeMapKey;

/// Contains the data maintained by the bot
pub struct Serv {
    /// Server child process
    pub child_process: Option<Child>,
}
impl Serv {
    /// Creates a new instance, childless
    pub fn new() -> Self {
        Serv {
            child_process: None,
        }
    }
    /// create with a child
    pub fn new_with_child(x: Child) -> Self {
        Serv {
            child_process: Some(x),
        }
    }
}
impl TypeMapKey for Serv {
    type Value = Serv;
}