use anyhow::{anyhow, bail, Context as _, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

#[derive(Parser)]
struct CliOptions {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    NewQuestion(NewQuestionOptions),
}

#[derive(Parser)]
struct NewQuestionOptions {
    #[arg(long)]
    name: String,
    #[arg(long)]
    day: Option<u32>,
}

fn word_to_num(word: &str) -> Option<u32> {
    const WORDS: [&str; 25] = [
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
        "twenty",
        "twenty-one",
        "twenty-two",
        "twenty-three",
        "twenty-four",
        "twenty-five",
    ];
    for (i, w) in WORDS.iter().enumerate() {
        if w == &word {
            return Some(i as u32 + 1);
        }
    }
    None
}

const CARGO_TOML: &str = r#"
[package]
name = "<name>"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
advent = { path = "../advent" }
parse = { path = "../parse" }
combine = "*"
"#;

const MAIN_RS: &str = r#"
use advent::prelude::*;

#[part_one]
fn part_one(_: String) -> &'static str {
    "incomplete"
}

#[part_two]
fn part_two(_: String) -> &'static str {
    "incomplete"
}

harness!();

"#;

const YEAR: u32 = 2023;

fn download_input(name: &str, day: u32) -> Result<()> {
    let session_key = fs::read_to_string(
        homedir::my_home()?
            .ok_or(anyhow!("home directory set"))?
            .join(".config/aocd/token"),
    )
    .with_context(|| "session key missing: ~/.config/aocd/token")?;

    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(format!("https://adventofcode.com/{YEAR}/day/{day}/input"))
        .header("cookie", format!("session={}", session_key.trim()))
        .send()?;

    fs::write(Path::new(name).join("input.txt"), resp.text()?)?;
    Ok(())
}

fn add_to_workspace(name: &str) -> Result<()> {
    let toml = fs::read_to_string("Cargo.toml")?;
    let mut cargo_toml: toml::Value = toml::from_str(&toml)?;
    let members = cargo_toml["workspace"]["members"]
        .as_array_mut()
        .ok_or(anyhow!("workspace.members present"))?;
    members.push(name.into());
    members.sort_by_key(|v| v.as_str().map(|s| s.to_string()).unwrap_or_default());

    fs::write("Cargo.toml", toml::to_string_pretty(&cargo_toml)?)?;

    Ok(())
}

fn add_new_question(name: &str, day: u32) -> Result<()> {
    let path = Path::new(name);
    if path.exists() {
        bail!("ERROR: {name} already exists");
    }
    fs::create_dir(path)?;
    fs::create_dir(path.join("src"))?;

    fs::write(
        path.join("Cargo.toml"),
        CARGO_TOML.replace("<name>", name).trim(),
    )?;
    fs::write(path.join("src/main.rs"), MAIN_RS.trim())?;

    add_to_workspace(name)?;
    download_input(name, day)?;

    Ok(())
}

fn main() -> Result<()> {
    let opt = CliOptions::parse();
    match opt.command {
        Command::NewQuestion(opt) => {
            let day = match opt.day {
                Some(day) => day,
                None => {
                    word_to_num(&opt.name).ok_or(anyhow!("day number could not be determined"))?
                }
            };
            add_new_question(&opt.name, day)?;
        }
    }

    Ok(())
}
