use serenity::prelude::{Context};
use serenity::model::channel::GuildChannel;
use crate::commands::*;
use log::error;
use crate::errors::catch_error;

pub enum Command {
    Code(String),
    Help()
}

pub fn execute(ctx: &Context, chan: GuildChannel, c: Command) {
    let res = match c {
        Command::Code(c) => code::execute(ctx, c),
        Command::Help()  => help::execute(ctx),
        _                => Err(String::from(""))
    };
    match res {
        Err(e)  => error!("{}", e),
        Ok(res) => catch_error(chan.say(&ctx.http, res))
    }
}