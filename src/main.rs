#[allow(dead_code)]
mod baker;

#[allow(dead_code)]
mod logger;

use logger::Logger;

fn main() {
    match baker::BakFile::new() {
        Ok(_) => (),
        Err(e) => {
            Logger::error(&format!("{e}"));
            std::process::exit(1);
        },
    };

    let bak = match baker::BakFile::content() {
        Ok(o) => o,
        Err(e) => {
            Logger::error(&format!("{e}"));
            std::process::exit(1);
        },
    };

    Logger::print(&bak);
}
