mod rule;

use crate::rule::{Rule, RuleType};

fn main() {
    println!("Hello, world!");
}

struct RuleValidator {
    rules: Vec<Rule>,
}

impl RuleValidator {
    fn new(rules: Vec<Rule>) -> Self {
        Self { rules }
    }

    fn get_rule_at_idx(&self, idx: usize) -> Option<&Rule> {
        self.rules.iter().find(|rr| rr.idx == idx)
    }

    fn validate(&self, in_str: &str, idx: usize) -> bool {
        let rule = match self.get_rule_at_idx(idx) {
            Some(rr) => rr,
            None => return false,
        };

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        let rules = vec!["0: 1 2", r#"1: "a""#, r#"2: "b""#]
            .into_iter()
            .map(|ss| match Rule::parse(ss) {
                Ok(rr) => rr,
                Err(e) => panic!("failed to parse rule: {}", e),
            })
            .collect::<Vec<_>>();

        //println!("{:?}", rules);
        let validator = RuleValidator::new(rules);
        assert_eq!(true, validator.validate("ab", 0));
    }
}
