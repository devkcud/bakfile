mod baker;
mod logger;
mod rules;
mod config;
mod arguer;

use std::io;

use arguer::Arguer;
use baker::BakFile;
use colored::{Colorize, control};
use config::Config;
use logger::Logger;
use regex::Regex;
use rules::{define_rule, RuleManager};

fn main() {
    if let Err(e) = Config::setup(Arguer::has("--local") || Arguer::has("-L")) { Logger::exit(&format!("An error occurred: {}", e)); }

    let config = Config::get_config();
    control::set_override(config.colors);
    Logger::set_level(config.log);

    if let Err(e) = run_program() { Logger::exit(&format!("An error occurred: {}", e)); }
}

fn find_rule<T>(rules: &[define_rule::Rule], cond: T, exitstr: &str) -> ()
where
    T: Fn(&&define_rule::Rule) -> bool
{
    rules.iter().find(cond).unwrap_or_else(|| Logger::exit(exitstr)).run();
}

fn run_program() -> io::Result<()> {
    let filearg = Arguer::get("file");

    let content = BakFile::new(if filearg.is_none() || filearg.unwrap().1 == "" { Config::get_config().rulefilename } else { filearg.unwrap().1 })?.read()?;
    let rules: Vec<define_rule::Rule> = define_rule::Rule::gather(&content)?;

    for capture in Regex::new(RuleManager::get_rule_regex("run")).unwrap().captures_iter(&content) {
        let mut names: Vec<&str> = capture[0].trim().split_whitespace().collect();
        names.remove(0);

        if names.is_empty() {
            find_rule(&rules, |x| x.is_default, "No default rule found");
        }

        for name in names {
            find_rule(&rules, |x| if name == define_rule::DEFAULT_DEFINER { x.is_default } else { x.name == name }, &format!("No rule {} found at line {}", name.purple().bold(), (content.lines().position(|x| x == &capture[0]).unwrap() + 1).to_string().purple().bold()));
        }
    };

    return Ok(());
}
