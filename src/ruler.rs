use std::io;
use colored::Colorize;
use itertools::Itertools;
use regex::Regex;

use crate::{baker::BakFile, logger::Logger};

#[derive(Clone)]
#[derive(Hash)]
#[derive(PartialEq, Eq)]
pub struct SetRule {
    pub name: String,
    pub commands: Vec<String>,
    // TODO: Implement 'is_default' (is_default: bool)
}

pub struct Ruler;

impl Ruler {
    pub fn lookup_set_rules(bakfile: BakFile) -> io::Result<Vec<SetRule>> {
        let content = bakfile.read()?;

        let regex = Regex::new(r"(?m)^\$set.*$").unwrap();
        let captures = regex.captures_iter(&content);

        let mut rules: Vec<SetRule> = Vec::new();

        let name_regex = Regex::new(r"[^a-zA-Z0-9]").unwrap();

        for capture in captures {
            let capture = capture.get(0).unwrap().as_str().trim();

            let commands = content
                .lines()
                .skip_while(|&x| {
                    x != capture
                })
                .skip(1)
                .take_while(|&x| {
                    !x.starts_with('$')
                })
                .filter(|x| !x.is_empty())
                .map(|x| String::from(x.trim()))
                .collect::<Vec<String>>();

            let curline = content.lines().position(|x| x == capture).unwrap() + 1;

            let mut arguments = capture.split_whitespace().collect::<Vec<&str>>();
            arguments.remove(0);

            let name = name_regex.replace_all(arguments[0], "").to_string();

            if arguments.len() == 0 || name == "" {
                Logger::exit(&format!("Rule {} at line {} | Proper define: {}", capture.red(), curline.to_string().red(), "$set <name> [*]".green()));
            }

            Logger::info(&format!("Loaded rule {} with {} commands", name.purple().bold(), commands.len().to_string().purple().bold()));

            rules.push(SetRule { name, commands });
        }

        let rules = rules.into_iter().unique().collect_vec();

        return Ok(rules);
    }
}
