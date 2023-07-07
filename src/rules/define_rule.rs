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
    pub fn gather(content: String) -> io::Result<Vec<Self>> {
        let mut rules: Vec<Self> = Vec::new();

        for capture in Regex::new(r"(?m)^\$define.* $").unwrap().captures_iter(&content) {
            let capture = capture[0].trim();

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
                let name = Regex::new(r"[^a-zA-Z0-9]").unwrap().replace_all(arg, "").to_string();
                let is_default = arguments.len() > 1 && arguments[1] == "*";

                if name.is_empty() {
                    Logger::exit(&format!("Rule {} at line {} | Proper define: {}", capture.red(), (content.lines().position(|x| x == capture).unwrap() + 1).to_string().red(), "$define <name> [*]".green()));
                }

                Logger::info(&format!("Loaded rule {} with {} commands (default: {})",
                    name.purple().bold(),
                    commands.len().to_string().purple().bold(),
                    is_default.to_string().purple().bold()
                ));

                rules.push(Self { name, commands, is_default });
            }
        }

        return Ok(rules.into_iter().unique().collect_vec());
    }

    fn run(&self) {
        for command in self.commands.iter() {
            Logger::info(&format!("Running {}", command.purple().bold()));

            match Command::new("sh").arg("-c").arg(command).output() {
                Ok(o) => Logger::print(&String::from_utf8_lossy(if !o.stdout.is_empty() { &o.stdout } else { &o.stderr })),
                Err(e) => Logger::error(&e.to_string()),
            }
        }
    }
}
