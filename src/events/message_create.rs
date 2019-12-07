use serenity::prelude::{Context};
use serenity::model::channel::{Message};
use std::thread;

use crate::command_handler;

pub fn on_message(_ctx: Context, msg: Message) {
    if !msg.author.bot {
        thread::spawn(||
            command_handler::handle(msg)
        );
    }
}