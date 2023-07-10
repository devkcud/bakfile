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

    // TODO: Write better code :(
    // TODO: Remove repeated code
    for capture in Regex::new(r"(?m)^\$run.*$").unwrap().captures_iter(&bakfile.read().unwrap_or(String::new())) {
        let mut args = capture[0].trim().split_whitespace().collect::<Vec<&str>>();
        args.remove(0);

        if args.len() == 0 {
            if let Ok(rules) = define_rule::Rule::gather(bakfile.read().unwrap_or(String::new())) {
                if let Some(rule) = rules.iter().find(|x| x.is_default) {
                    rule.run();
                }
            }

            break;
        }

        if let Ok(rules) = define_rule::Rule::gather(bakfile.read().unwrap_or(String::new())) {
            for arg in args {
                if let Some(rule) = rules.iter().find(|x| x.name == arg) {
                    // why much nesting wtf
                    rule.run();
                }
            }
        }
    }


    Logger::info("Program ended");
}
