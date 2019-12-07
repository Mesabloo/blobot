use serenity::prelude::{EventHandler, Context};
use serenity::model::{gateway::Ready, channel::Message};
use crate::events::*;

pub struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, r: Ready) {
        ready::handle(r);
    }

    fn message(&self, ctx: Context, m: Message) {
        message_create::on_message(ctx, m);
    }
}