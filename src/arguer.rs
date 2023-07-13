use std::env::args;

pub struct Arguer {
    commands: Vec<String>,
}

impl Arguer {
    pub fn new() -> Self {
        return Self { commands: args().collect() };
    }

    pub fn get(&self, name: &str) -> Option<(&str, &str)> {
        return self.commands
            .iter()
            .find_map(|command| {
                let (key, value) = command.split_once('=').unwrap_or((command, ""));
                if key == name {
                    Some((key, value))
                } else {
                    None
                }
            });
    }

    pub fn has(&self, name: &str) -> bool {
        return self.get(name).is_some();
    }
}
