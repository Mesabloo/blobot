use serenity::prelude::{Context};
use serenity::model::channel::Message;
use log::{debug};
use std::env;

pub fn handle(ctx: Context, msg: Message) {
    debug!("Received message!");
    debug!("> Is in guild channel?");

    let channel = msg.channel_id.to_channel(&ctx).expect("channel not found");
    if let Some(lock) = channel.guild() {
        debug!("| Yes");
    } else {
        debug!("| No");
        return;
    }

    debug!("> Is command?");
    let mut content = msg.content;
    let command_prefix = env::var("PREFIX").expect("command prefix");
    if content.starts_with(&command_prefix) {
        debug!("| Yes");
    } else {
        debug!("| No");
        return;
    }
}