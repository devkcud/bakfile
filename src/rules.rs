pub mod define_rule;

use std::{io, process::Command};

use colored::Colorize;

use crate::logger::Logger;

trait Runner {
    fn run(&self);
}

impl Runner for define_rule::Rule {
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

pub struct Ruler {
    define_rules: Vec<define_rule::Rule>,
}

impl Ruler {
    pub fn new(content: String) -> io::Result<()> {
        Ok(())
    }
}
