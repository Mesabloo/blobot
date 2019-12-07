use serenity::client::Client;
use std::env;
use crate::errors::{catch_error};
use crate::handler::{Handler};
use env_logger::{Builder};

mod events;
mod handler;
mod errors;

fn main() {
    Builder::from_env("LOGS")
        .format_timestamp(None)
        .format_module_path(false)
        .init();

    let token = &env::var("TOKEN").expect("token");
    let mut client = Client::new(token, Handler)
        .expect("Error creating client");

    catch_error(client.start());
}
