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
use chrono::{Utc, Duration};
use regex::Regex;
use std::process::{Command, Stdio};
use wait_timeout::ChildExt;
use std::io::Read;
use reqwest::Client;
use serde::Deserialize;

pub fn execute(_ctx: &Context, code: String) -> Result<String, String> {
    debug!("code: ```{}```", &code);

    let time = Utc::now().timestamp_millis();
    let filename = format!("tmp/tmp{}.blob", &time);

    let file = File::create(&filename);
    match file {
        Err(e)    => return Err(e.to_string()),
        Ok(mut f) => f.write_all(code.as_bytes()).unwrap()
    };

    let time = Duration::seconds(30);
    let mut command = Command::new("./bin/iBlob")
        .arg("repl")
        .arg(&filename)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process `iBlob`");

    command.stdin.as_mut()
        .expect("cannot write to stdin")
        .write(b"main").expect("cannot write to stdin");

    let res = match command.wait_timeout(time.to_std().unwrap()).unwrap() {
        Some(_) => {
            let mut output = String::new();
            command.stdout.unwrap()
                .read_to_string(&mut output).expect("cannot read from stdout");

            let re = Regex::new("\\[.+?m").unwrap();
            let res = re.replace_all(output.as_str(), "");

            if res.len() >= 1900 {
                let link = request_hastebin(&res);
                Ok(format!("> Output was too long so I created a link for you: {}", link))
            } else {
                Ok(format!(">>> ```hs\n{}```", res))
            }
        },
        None    => {
            command.kill().unwrap();
            Err(String::from("Process timed out after 30s."))
        }
    };

    fs::remove_file(&filename).expect("unable to delete temporary file");
    res
}

#[derive(Deserialize)]
struct HastebinResponse {
    key: String,
}

fn request_hastebin(s: &str) -> String {
    let cli = Client::new();
    let json: HastebinResponse =
        cli.post("https://hasteb.in/documents")
            .body(String::from(s))
            .send().expect("cannot send request")
            .json().expect("cannot retrieve JSON");
    format!("https://hasteb.in/{}", json.key)
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