#[allow(dead_code)]
mod baker;

#[allow(dead_code)]
mod logger;

use logger::Logger;

fn main() {
    baker::BakFile::new();
    let bak = baker::BakFile::content().unwrap();
}
