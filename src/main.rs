#[allow(dead_code)]
mod baker;

#[allow(dead_code)]
mod logger;

use baker::BakFile;
use logger::Logger;

fn main() {
    let bakfile = match BakFile::new(String::from(".baker")) {
        Ok(o) => o,
        Err(e) => Logger::exit(&format!("{e}")),
    };

    let content: &str = &match bakfile.read() {
        Ok(o) => o,
        Err(e) => Logger::exit(&format!("{e}")),
    };

    Logger::print(content);
}
