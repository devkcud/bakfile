use std::{io, process::Command};
use colored::Colorize;
use itertools::Itertools;
use regex::Regex;

use crate::{baker::BakFile, logger::Logger};

fn run_commands(commands: Vec<String>) -> () {
    for command in commands.iter() {
        match Command::new("sh").arg("-c").arg(command).output() {
            Ok(o) => {
                if !o.stdout.is_empty() {
                    Logger::print(&String::from_utf8_lossy(&o.stdout));
                }

                if !o.stderr.is_empty() {
                    Logger::print(&String::from_utf8_lossy(&o.stderr));
                }
            },

            Err(e) => Logger::error(&e.to_string()),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct DefineRule {
    pub name: String,
    pub commands: Vec<String>,
    pub is_default: bool,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct RunRule {
    pub to_run: Vec<DefineRule>,
}

pub struct Ruler {
    file_content: String,
    define_rules: Vec<DefineRule>,
}

impl Ruler {
    pub fn new(bakfile: BakFile) -> io::Result<Self> {
        let file_content = bakfile.read()?;

        let regex = Regex::new(r"(?m)^\$define.*$").unwrap();
        let name_regex = Regex::new(r"[^a-zA-Z0-9]").unwrap();
        let captures = regex.captures_iter(&file_content);

        let mut rules: Vec<DefineRule> = Vec::new();

        for capture in captures {
            let capture = capture[0].trim();

            let commands = file_content
                .lines()
                .skip_while(|&x| x != capture)
                .skip(1)
                .take_while(|&x| !x.starts_with('$'))
                .filter_map(|x| if x.is_empty() { None } else { Some(x.trim().to_string()) })
                .collect::<Vec<String>>();

            let curline = file_content.lines().position(|x| x == capture).unwrap() + 1;

            let mut arguments = capture.split_whitespace().collect::<Vec<&str>>();
            arguments.remove(0);

            if let Some(arg) = arguments.get(0) {
                let name = name_regex.replace_all(arg, "").to_string();
                let is_default = arguments.len() > 1 && arguments[1] == "*";

                if name.is_empty() {
                    Logger::exit(&format!("Rule {} at line {} | Proper define: {}", capture.red(), curline.to_string().red(), "$define <name> [*]".green()));
                }

                Logger::info(&format!("Loaded rule {} with {} commands (default: {})",
                    name.purple().bold(),
                    commands.len().to_string().purple().bold(),
                    is_default.to_string().purple().bold()
                ));

                rules.push(DefineRule { name, commands, is_default });
            }
        }

        return Ok(Self {
            file_content,
            define_rules: rules.into_iter().unique().collect_vec(),
        });
    }

    pub fn setup_run(&self) -> io::Result<()> {
        let regex = Regex::new(r"(?m)^\$run.*$").unwrap();
        let captures = regex.captures_iter(&self.file_content);

        for capture in captures {
            let mut rules_to_run = capture[0].split_whitespace().collect::<Vec<&str>>();
            rules_to_run.remove(0);

            if rules_to_run.len() == 0 {
                for rule in self.define_rules.iter().filter(|x| x.is_default) {
                    run_commands(rule.commands.clone());
                }
                continue;
            }

            for define_rule in rules_to_run {
                if let Some(rule) = self.define_rules.iter().find(|x| x.name == define_rule) {
                    run_commands(rule.commands.clone());
                }
            }
        }

        return Ok(());
    }
}
