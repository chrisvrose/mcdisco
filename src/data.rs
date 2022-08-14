use std::process::Child;

use serenity::prelude::TypeMapKey;

pub struct Serv {
    pub child_process: Option<Child>,
}
impl Serv {
    pub fn new() -> Self {
        Serv {
            child_process: None,
        }
    }
    pub fn new_with_child(x: Child) -> Self {
        Serv {
            child_process: Some(x),
        }
    }
}
impl TypeMapKey for Serv {
    type Value = Serv;
}