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

    let ruler = match Ruler::new(bakfile) {
        Ok(o) => o,
        Err(e) => Logger::exit(&format!("{e}")),
    };

    match ruler.setup_run() {
        Ok(_) => (),
        Err(e) => Logger::exit(&format!("{e}")),
    };
}
