use std::env::args;

lazy_static::lazy_static! {
    static ref ARGUER_ARGS: Vec<String> = args().collect();
}

pub struct Arguer;

impl Arguer {
    pub fn get(name: &str) -> Option<(&str, &str)> {
        ARGUER_ARGS
            .iter()
            .find_map(|command| {
                let (key, value) = command.split_once('=').unwrap_or((command, ""));
                if key == name { Some((key, value)) } else { None }
            })
    }

    pub fn has(name: &str) -> bool {
        Arguer::get(name).is_some()
    }
}
