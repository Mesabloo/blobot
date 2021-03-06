use serenity::prelude::{Context};
use serenity::model::channel::{Message};
use std::thread;

use crate::command_handler;

pub fn on_message(ctx: Context, msg: Message) {
    if !msg.author.bot {
        thread::spawn(move ||
            command_handler::handle(ctx, msg)
        );
    }
}