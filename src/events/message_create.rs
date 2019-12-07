use serenity::prelude::{Context};
use serenity::model::channel::{Message};
use crate::errors::{catch_error};

pub fn on_message(ctx: Context, msg: Message) {
    if !msg.author.bot {
        let res = msg.channel_id.say(&ctx.http, msg.content);
        catch_error(res);
    }
}