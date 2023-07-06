#[allow(dead_code)]
mod baker;

#[allow(dead_code)]
mod logger;

#[allow(dead_code)]
mod ruler;

use baker::BakFile;
use logger::Logger;
use ruler::SetRule;

fn main() {
    let bakfile = match BakFile::new(String::from(".baker")) {
        Ok(o) => o,
        Err(e) => Logger::exit(&format!("{e}")),
    };

    SetRule::read(bakfile).unwrap();
}
