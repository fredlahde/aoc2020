use std::error::Error;
use std::fmt::{self, Display};
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum RuleParseError {
    Syntax { cause: String },
    ParseInt { inner: ParseIntError },
    InvalidRuleKind,
    NoRuleString,
    NoRuleChar,
}

impl Display for RuleParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Syntax { cause } => write!(f, "invalid syntax: {}", cause),
            Self::ParseInt { inner } => inner.fmt(f),
            Self::InvalidRuleKind => write!(f, "invalid rule kind given"),
            Self::NoRuleString => write!(f, "no defintion of rule give"),
            Self::NoRuleChar => write!(f, "no rule char in direct rule"),
        }
    }
}

impl From<ParseIntError> for RuleParseError {
    fn from(err: ParseIntError) -> Self {
        Self::ParseInt { inner: err }
    }
}

impl Error for RuleParseError {}
