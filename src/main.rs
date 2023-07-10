#[allow(dead_code)]
mod baker;

#[allow(dead_code)]
mod logger;

#[allow(dead_code)]
mod rules;

use baker::BakFile;
use colored::Colorize;
use logger::Logger;
use regex::Regex;
use rules::define_rule::{self, Rule};

fn main() {
    let bakfile = match BakFile::new(".baker") {
        Ok(o) => o,
        Err(e) => Logger::exit(&format!("{e}")),
    };

    let content = match bakfile.read() {
        Ok(o) => o,
        Err(e) => Logger::exit(&format!("{e}")),
    };

    let rules: Vec<Rule> = match define_rule::Rule::gather(&content) {
        Ok(o) => o,
        Err(e) => Logger::exit(&format!("{e}")),
    };

    for capture in Regex::new(r"(?m)^\$run.*$").unwrap().captures_iter(&content) {
        let mut names: Vec<&str> = capture[0].trim().split_whitespace().collect();
        names.remove(0); // Remove '$run'

        if names.is_empty() {
            rules.iter().find(|x| x.is_default).unwrap_or_else(|| Logger::exit("No default rule found")).run();
        }

        for name in names {
            rules
                .iter()
                .find(|x| x.name == name)
                .unwrap_or_else(|| Logger::exit(&format!("No rule {} found at line {}",
                     name.purple().bold(),
                     (content.lines().position(|x| x == &capture[0]).unwrap() + 1).to_string().purple().bold()
                 )))
                .run();
        }
    }

    Logger::info("Program ended");
}
