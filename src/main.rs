#[allow(dead_code)]
mod baker;

#[allow(dead_code)]
mod logger;

#[allow(dead_code)]
mod ruler;

use baker::BakFile;
use logger::Logger;
use ruler::Ruler;

fn main() {
    let bakfile = match BakFile::new(".baker") {
        Ok(o) => o,
        Err(e) => Logger::exit(&format!("{e}")),
    };

    Ruler::lookup_set_rules(bakfile).unwrap();
    Ruler::lookup_run_rules(bakfile).unwrap();
}
