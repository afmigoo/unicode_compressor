use std::fmt;
use std::fmt::Display;

use clap::ValueEnum;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum TokenizationStrategy {
    FirstMatch,
    LongestMatch,
}

impl Display for TokenizationStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            TokenizationStrategy::FirstMatch => "first-match",
            TokenizationStrategy::LongestMatch => "longest-match",
        })
    }
}

pub struct EncodeOptions {
    pub tokenization_strategy: TokenizationStrategy,
}
