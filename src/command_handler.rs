use serenity::prelude::{Context};
use serenity::model::channel::Message;
use log::{error};
use std::env;
use crate::commands::command;
use crate::commands::*;
use nom::*;

pub fn handle(ctx: Context, msg: Message) {

    let channel = msg.channel_id.to_channel(&ctx).expect("channel not found");
    match channel.guild() {
        None                => return,
        Some(guild_channel) => {
            let content = &msg.content;
            if let Ok(("", cmd)) = parse_command(content) {
                command::execute(&ctx, (*guild_channel.read()).clone(), cmd)
            } else {
                error!("Could not parse entire command. Ignoring...");
            }
        }
    }
}

named!(parse_command<&str,command::Command>,
    complete!(do_parse!(prefix >> cmd: alt!(help::parse | code::parse) >> (cmd)))
);

named!(prefix<&str,&str>,
    tag!(&*env::var("PREFIX").unwrap_or(String::from("B>")))
);