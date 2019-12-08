use nom::character::complete::*;
use nom::bytes::complete::{tag};
use nom::branch::{alt};
use nom::*;
use crate::commands::command;
use serenity::prelude::{Context};

pub fn execute(_: &Context) -> Result<&str, &str> {
    Ok(">>> __Help:__\n\n\
        **`:?` `:h` `:help`**: Show this help menu\n\
        \n\
        You can also directly write code in blocks, using \\`\\`\\`.
    ")
}

pub fn parse(i: &str) -> IResult<&str, command::Command> {
    let (i, _) = multispace0(i)?;
    let (i, _) = alt((tag(":help"), tag(":h"), tag(":?")))(i)?;
    let (i, _) = combinator::complete(multispace0)(i)?;
    Ok((i, command::Command::Help()))
}