use std::fmt;
use std::fmt::Display;

use clap::ValueEnum;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum CompressionLevel {
    // TODO: rename
    Fast,
    Balanced,
}

impl Display for CompressionLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            CompressionLevel::Fast => "fast",
            CompressionLevel::Balanced => "balanced",
        })
    }
}

pub struct EncodeOptions {
    pub level: CompressionLevel,
}
