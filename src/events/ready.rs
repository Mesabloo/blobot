use serenity::prelude::{Context};
use serenity::model::gateway::{Ready, Activity};
use serenity::model::user::OnlineStatus;
use log::{info};

pub fn handle(ctx: Context, r: Ready) {
    info!("Logged in as {}#{}", r.user.name, r.user.id);
    let activity = Activity::playing("with some blob code");
    ctx.set_presence(Some(activity), OnlineStatus::Online);
}