use std::{io, process::Command};

use colored::Colorize;
use itertools::Itertools;
use regex::Regex;

use crate::logger::Logger;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Rule {
    pub name: String,
    pub commands: Vec<String>,
    pub is_default: bool,
}

impl Rule {
    pub fn gather(content: &str) -> io::Result<Vec<Self>> {
        Logger::info("Started gathering define rules");
        let mut rules: Vec<Self> = Vec::new();

        let name_regex = Regex::new(r"[^a-zA-Z0-9]").unwrap();

        for capture in Regex::new(r"(?m)^\$define.*$").unwrap().captures_iter(content) {
            let capture = capture[0].trim();
            let line_id = content.lines().position(|x| x == capture).unwrap() + 1;

            let commands = content
                .lines()
                .skip_while(|&x| x != capture)
                .skip(1)
                .take_while(|&x| !x.starts_with('$'))
                .filter_map(|x| if !x.is_empty() { Some(x.trim().to_string()) } else { None })
                .collect::<Vec<String>>();

            let mut arguments = capture.split_whitespace().collect::<Vec<&str>>();
            arguments.remove(0);

            if let Some(arg) = arguments.get(0) {
                let name = name_regex.replace_all(arg, "").to_string();

                if name.is_empty() {
                    Logger::exit(&format!("Rule {} at line {} | Proper define: {}", capture.red(), line_id.to_string().red(), "$define <name> [*]".green()));
                }

                if rules.iter().find(|x| x.name == name).is_none() {
                    let default = rules.iter().find(|x| x.is_default);
                    let is_default = arguments.len() > 1 && arguments[1] == "*" && default.is_none();

                    if default.is_some() {
                        Logger::warn(&format!("{} can't be defaulted. Rule {} is already the default", name.purple().bold(), default.unwrap().name.purple().bold()));
                    }

                    Logger::log(&format!("Loaded rule {} with {} commands (default: {})",
                        name.purple().bold(),
                        commands.len().to_string().purple().bold(),
                        is_default.to_string().purple().bold()
                    ));

                    rules.push(Self { name, commands, is_default });
                } else {
                    Logger::warn(&format!("Rule {} already defined at {}", name.purple().bold(), format!("line {}", line_id).red()));
                }
            }
        }

        return Ok(rules.into_iter().unique().collect_vec());
    }

    pub fn run(&self) {
        Logger::info(&format!("Running rule {} ({} commands)", self.name.purple().bold(), self.commands.len().to_string().purple().bold()));

        for command in self.commands.iter() {
            match Command::new("sh").arg("-c").arg(command).output() {
                Ok(o) => Logger::print(&String::from_utf8_lossy(if !o.stdout.is_empty() { &o.stdout } else { &o.stderr })),
                Err(e) => Logger::error(&e.to_string()),
            }
        }

        Logger::info(&format!("Done running rule {} ({} commands)", self.name.purple().bold(), self.commands.len().to_string().purple().bold()));
    }
}
