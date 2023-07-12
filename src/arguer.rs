use std::env::args;

#[allow(dead_code)]
pub struct Arguer {
    commands: Vec<String>,
    flags:    Vec<String>,
}

impl Arguer {
    pub fn new() -> Self {
        let commands: Vec<String> = args().filter(|x| !x.starts_with('-')).collect();
        let flags:    Vec<String> = args().filter(|x|  x.starts_with('-')).collect();

        return Self { commands, flags };
    }

    pub fn get_flag(&self, name: &str) -> Option<(&str, &str)> {
        let flag = if let Some(o) = self.flags.iter().find(|&x| &x.split_once('=').unwrap_or((x, "")).0[1..] == name) {
            let (key, value) = o.split_once('=').unwrap_or((o, ""));
            (&key[1..], value)
        } else {
            return None;
        };

        return Some((flag.0, flag.1));
    }

    pub fn has_flag(&self, name: &str) -> bool {
        return self.get_flag(name).is_some();
    }
}
