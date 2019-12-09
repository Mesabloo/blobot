use serenity::client::Client;
use std::env;
use crate::errors::{catch_error};
use crate::event_handler::{Handler};
use env_logger::{Builder};
use std::{path::Path};
use log::{debug, info, error};
use cmd_lib::{run_cmd};
use std::fs;

mod events;
mod event_handler;
mod command_handler;
mod errors;
mod commands;

fn main() {
    Builder::from_env("LOGS")
        .format_timestamp(None)
        .format_module_path(false)
        .format_indent(Some(4))
        .init();

    let token = &env::var("TOKEN").expect("token");
    let mut client = Client::new(token, Handler)
        .expect("Error creating client");

    if install_blob() {
        catch_error(client.start());
    }
}

fn install_blob() -> bool {
    if !Path::new("./blob").exists() {
        debug!("Blob not found in `./blob`. Cloning...");
        let res = run_cmd!("git clone https://github.com/mesabloo/blob blob");
        if let Err(why) = res {
            error!("{}", why);
            return false;
        } else {
            if let Err(why) = run_cmd!("cd blob") {
                error!("{}", why);
                return false;
            } else {
                debug!("Cloned blob in `./blob`.");
            }
        }
    } else {
        let res = run_cmd!("cd blob; git pull");
        if let Err(why) = res {
            error!("{}", why);
            return false;
        }
    }

    let res = run_cmd!("stack install --local-bin-path '../bin'");
    if let Err(why) = res {
        error!("{}", why);
        false
    } else {
        info!("Successfully installed blob to `./bin`.");
        run_cmd!("cd ..").unwrap();

        fs::create_dir_all("tmp").expect("unable to create directory `./tmp`");

        true
    }
}