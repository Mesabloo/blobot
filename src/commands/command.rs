use serenity::prelude::{Context};
use serenity::model::channel::Channel;
use crate::commands::*;
use log::error;
use crate::errors::catch_error;

pub enum Command {
    Code(String)
}

pub fn execute(ctx: &Context, chan: Channel, c: Command) {
    let res = match c {
        Command::Code(c) => code::execute(ctx, c)
    };
    match res {
        Err(e) => error!("{}", e),
        Ok(_)  => {
            let guild_chan = chan.guild().unwrap();
            catch_error(guild_chan.read().say(&ctx.http, "> Succesfully compiled code!"));
        }
    }
}