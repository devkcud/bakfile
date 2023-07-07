pub mod define_rule;

use std::io;

pub struct Ruler {
    define_rules: Vec<define_rule::Rule>,
}

impl Ruler {
    pub fn new(_content: String) -> io::Result<()> {
        Ok(())
    }
}
