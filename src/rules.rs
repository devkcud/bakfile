use std::collections::HashMap;

pub mod define_rule;

lazy_static::lazy_static! {
    static ref RULE_TOKENS: HashMap<&'static str, &'static str> = {
        let mut obj = HashMap::new();
        obj.insert("run",    r"(?m)^\$run\s+.*");
        obj.insert("define", r"(?m)^\$define\s+.*");
        obj
    };
}

pub struct RuleManager;

impl RuleManager {
    pub fn get_rule_regex(rule: &str) -> &str {
        return RULE_TOKENS.get(rule).unwrap();
    }
}
