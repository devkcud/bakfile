#[allow(dead_code)]
mod baker;

fn main() {
    baker::BakFile::new();
    let bak = baker::BakFile::content().unwrap();

    println!("{bak}");
}
