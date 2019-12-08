use log::{error};
use std::result::{Result};
use std::fmt::{Display};

pub fn catch_error<A, B: Display>(action: Result<A, B>) {
    if let Err(err) = action {
        error!("Error: {}", err);
    }
}