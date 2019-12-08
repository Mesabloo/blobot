use nom::*;
use nom::character::complete::*;
use nom::combinator::{rest, complete, opt, not};
use nom::bytes::complete::{tag};
use nom::sequence::{preceded};
use nom::multi::{many1, many_till, many0};
use serenity::prelude::{Context};
use crate::commands::command;
use std::iter::FromIterator;
use log::{debug};

pub fn execute(ctx: &Context, code: String) -> Result<&str, &str> {
    debug!("code: ```{}```", code);
    Ok("")
}

pub fn parse(i: &str) -> IResult<&str, command::Command> {
    let (i, _)    = multispace0(i)?;
    let (i, code) = complete(rest)(i)?;
    let (_, code) = complete(parse_codeblock)(code)?;
    Ok((i, command::Command::Code(code)))
}

fn parse_codeblock(i: &str) -> IResult<&str, String> {
    let (i, _)         = tag("```")(i)?;
    let (i, _)         = opt(preceded(alpha1, preceded(many1(line_ending), not(tag("```")))))(i)?;
    let (i, (code, _)) = preceded(many0(line_ending), many_till(anychar, tag("```")))(i)?;
    Ok((i, String::from_iter(code)))
}