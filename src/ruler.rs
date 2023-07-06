use std::io;
use colored::Colorize;
use itertools::Itertools;
use regex::Regex;

use crate::{baker::BakFile, logger::Logger};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct SetRule {
    pub name: String,
    pub commands: Vec<String>,
    pub is_default: bool,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct RunRule {
    pub to_run: Vec<SetRule>,
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
            let capture = capture[0].trim();

            let commands = content
                .lines()
                .skip_while(|&x| x != capture)
                .skip(1)
                .take_while(|&x| !x.starts_with('$'))
                .filter_map(|x| if x.is_empty() { None } else { Some(x.trim().to_string()) })
                .collect::<Vec<String>>();

            let curline = content.lines().position(|x| x == capture).unwrap() + 1;

            let mut arguments = capture.split_whitespace().collect::<Vec<&str>>();
            arguments.remove(0);

            if let Some(arg) = arguments.get(0) {
                let name = name_regex.replace_all(arg, "").to_string();
                let is_default = arguments.len() > 1 && arguments[1] == "*";

                if name.is_empty() {
                    Logger::exit(&format!("Rule {} at line {} | Proper define: {}", capture.red(), curline.to_string().red(), "$set <name> [*]".green()));
                }

                Logger::info(&format!("Loaded rule {} with {} commands (default: {})",
                    name.purple().bold(),
                    commands.len().to_string().purple().bold(),
                    is_default.to_string().purple().bold()
                ));

                rules.push(SetRule { name, commands, is_default });
            }
        }

        return Ok(rules.into_iter().unique().collect_vec());
    }

    pub fn lookup_run_rules(bakfile: BakFile) -> io::Result<()> {
        let content = bakfile.read()?;

        let regex = Regex::new(r"(?m)^\$run.*$").unwrap();
        let captures = regex.captures_iter(&content);

        for capture in captures {
            let mut rules_to_run = capture[0].split_whitespace().collect::<Vec<&str>>();
            rules_to_run.remove(0);

            if rules_to_run.len() == 0 {
                // TODO: Run the default set rules
                continue;
            }

            for set_rule in rules_to_run {
                // TODO: Run the set rules (run)
                Logger::info(set_rule);
                continue;
            }
        }

        return Ok(());
    }
}
