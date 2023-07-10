#[allow(dead_code)]
mod baker;

#[allow(dead_code)]
mod logger;

#[allow(dead_code)]
mod rules;

use baker::BakFile;
use logger::Logger;
use regex::Regex;
use rules::define_rule;

fn main() {
    let bakfile = match BakFile::new(".baker") {
        Ok(o) => o,
        Err(e) => Logger::exit(&format!("{e}")),
    };

    for capture in Regex::new(r"(?m)^\$run.*$").unwrap().captures_iter(&content) {}

    if let Ok(rules) = define_rule::Rule::gather(bakfile.read().unwrap_or(String::new())) {
        for rule in rules { rule.run(); }
    }

    Logger::info("Program ended");
}
