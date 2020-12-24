pub mod rule_error;

use crate::rule::rule_error::RuleParseError;

#[derive(PartialEq, Debug)]
pub enum RuleType {
    Direct(char),
    Ref(Vec<usize>),
    RefOr(Vec<usize>, Vec<usize>),
}

#[derive(PartialEq, Debug)]
pub struct Rule {
    pub idx: usize,
    pub rule_type: RuleType,
}

impl Rule {
    pub fn parse(in_str: &str) -> Result<Self, RuleParseError> {
        let mut split = in_str.split(":");
        let idx_str = split.next().ok_or_else(|| RuleParseError::Syntax {
            cause: "no index given in rule".to_string(),
        })?;
        let idx: usize = idx_str.parse()?;

        let rule_str = split.next().ok_or(RuleParseError::NoRuleString)?;

        let mut char_iter = rule_str.chars();
        let mut digits_side_a: Vec<usize> = vec![];
        let mut digits_side_b: Vec<usize> = vec![];

        let mut digits_in_use = &mut digits_side_a;
        let mut or_rule = false;
        loop {
            let curr_char = match char_iter.next() {
                Some(cc) => cc,
                None => break,
            };

            if curr_char.is_whitespace() {
                continue;
            }

            if curr_char == '"' {
                let rule_char = match char_iter.next() {
                    Some(cc) => cc,
                    None => return Err(RuleParseError::NoRuleChar),
                };
                return Ok(Rule {
                    idx,
                    rule_type: RuleType::Direct(rule_char),
                });
            }

            if curr_char.is_digit(10) {
                let mut digit_chars = vec![curr_char];
                while let Some(cc) = char_iter.next() {
                    if !cc.is_digit(10) {
                        break;
                    }
                    digit_chars.push(cc);
                }

                let digit_str = digit_chars.iter().collect::<String>();
                let digit: usize = digit_str.parse()?;
                digits_in_use.push(digit);
            }

            if curr_char == '|' {
                digits_in_use = &mut digits_side_b;
                or_rule = true;
            }
        }

        if or_rule {
            return Ok(Rule {
                idx,
                rule_type: RuleType::RefOr(digits_side_a, digits_side_b),
            });
        }

        if digits_side_a.len() > 0 {
            return Ok(Rule {
                idx,
                rule_type: RuleType::Ref(digits_side_a),
            });
        }
        Err(RuleParseError::InvalidRuleKind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_parse() {
        let rule = r#"0: "b""#;
        assert_eq!(
            Ok(Rule {
                idx: 0,
                rule_type: RuleType::Direct('b')
            }),
            Rule::parse(rule)
        );
        let rule = r#"115: "z""#;
        assert_eq!(
            Ok(Rule {
                idx: 115,
                rule_type: RuleType::Direct('z')
            }),
            Rule::parse(rule)
        );

        let rule = r#"115: 1 2 3"#;
        assert_eq!(
            Ok(Rule {
                idx: 115,
                rule_type: RuleType::Ref(vec![1, 2, 3])
            }),
            Rule::parse(rule)
        );

        let rule = r#"0: 1 2 | 3 245"#;
        assert_eq!(
            Ok(Rule {
                idx: 0,
                rule_type: RuleType::RefOr(vec![1, 2], vec![3, 245])
            }),
            Rule::parse(rule)
        );
    }
}
