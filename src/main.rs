#[allow(dead_code)]
mod baker;

#[allow(dead_code)]
mod logger;

#[allow(dead_code)]
mod rules;

use std::io;

use baker::BakFile;
use colored::Colorize;
use logger::Logger;
use regex::Regex;
use rules::define_rule::{self, Rule};

fn main() {
    if let Err(e) = run_program() { Logger::exit(&format!("An error occurred: {}", e)); }
    Logger::info("Program ended");
}

fn find_rule<T>(rules: &[Rule], cond: T, exitstr: &str) -> ()
where
    T: Fn(&&Rule) -> bool
{
    rules.iter().find(cond).unwrap_or_else(|| Logger::exit(exitstr)).run();
}

fn run_program() -> io::Result<()> {
    let content: String   = BakFile::new(".baker")?.read()?;
    let rules:   Vec<Rule> = define_rule::Rule::gather(&content)?;

    for capture in Regex::new(r"(?m)^\$run.*$").unwrap().captures_iter(&content) {
        let mut names: Vec<&str> = capture[0].trim().split_whitespace().collect();
        names.remove(0);

        if names.is_empty() {
            find_rule(&rules, |x| x.is_default, "No default rule found");
        }

        for name in names {
            find_rule(&rules, |x| x.name == name, &format!("No rule {} found at line {}", name.purple().bold(), (content.lines().position(|x| x == &capture[0]).unwrap() + 1).to_string().purple().bold()));
        }
    };

    return Ok(());
}
