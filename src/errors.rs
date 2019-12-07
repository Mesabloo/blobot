use log::{error};
use std::result::{Result};
use std::fmt::{Debug};

pub fn catch_error<A, B: Debug>(action: Result<A, B>) {
    if let Err(err) = action {
        error!("Error: {:?}", err);
    }
}