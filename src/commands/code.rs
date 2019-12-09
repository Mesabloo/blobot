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
use std::fs::{File};
use std::fs;
use std::io::Write;
use chrono::Utc;
use cmd_lib::{run_fun};
use regex::Regex;

pub fn execute(_ctx: &Context, code: String) -> Result<String, String> {
    debug!("code: ```{}```", &code);

    let time = Utc::now().timestamp_millis();
    let filename = format!("tmp/tmp{}.blob", &time);

    let file = File::create(&filename);
    match file {
        Err(e)    => return Err(e.to_string()),
        Ok(mut f) => f.write_all(code.as_bytes()).unwrap()
    };
    let input = "main";

    let command = format!("echo \"{}\" | ./bin/iBlob repl \"{}\"", input, filename);
    let output = run_fun(&command).expect("failed to execute command");

    fs::remove_file(filename).expect("unable to delete temporary file");

    let re = Regex::new("\\[.+?m").unwrap();
    let res = re.replace_all(output.as_str(), "");

    Ok(format!(">>> ```hs\n\
                {}```", res))
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