#[allow(dead_code)]
mod baker;

#[allow(dead_code)]
mod logger;

#[allow(dead_code)]
mod rules;

use baker::BakFile;
use logger::Logger;
use rules::define_rule;

fn main() {
    let bakfile = match BakFile::new(".baker") {
        Ok(o) => o,
        Err(e) => Logger::exit(&format!("{e}")),
    };

    define_rule::Rule::gather(bakfile.read().unwrap_or(String::new())).unwrap();
}
