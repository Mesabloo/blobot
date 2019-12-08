use serenity::prelude::{Context};
use serenity::model::channel::Message;
use log::{debug};
use std::env;
use crate::commands::command;
use crate::commands::command::Command::*;

pub fn handle(ctx: Context, msg: Message) {
    debug!("Received message!");
    debug!("> Is in guild channel?");

    let channel = msg.channel_id.to_channel(&ctx).expect("channel not found");
    if channel.clone().guild().is_some() {
        debug!("| Yes");
    } else {
        debug!("| No");
        return;
    }

    debug!("> Is command?");
    let content = msg.content;
    let command_prefix = env::var("PREFIX").unwrap_or(String::from("B>"));
    if (&content).starts_with(&command_prefix) {
        debug!("| Yes");
    } else {
        debug!("| No");
        return;
    }

    command::execute(&ctx, channel, parse_command(content).expect("invalid command"));
}

fn parse_command(msg: String) -> Result<command::Command, String> {
    Ok(Code(msg))
}