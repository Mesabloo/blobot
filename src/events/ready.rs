use serenity::model::gateway::{Ready};
use log::{info};

pub fn handle(r: Ready) {
    info!("Logged in as {}#{}", r.user.name, r.user.id);
}