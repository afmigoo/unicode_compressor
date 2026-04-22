use std::fmt;

#[derive(Debug)]
pub enum Error {
    CharacterNotInAlphabet(String),
    CouldNotEncodeSubstring(String),
    EncoderNotFound(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CharacterNotInAlphabet(ch) => {
                write!(f, "character {:?} is not in the alphabet", ch)
            }
            Error::CouldNotEncodeSubstring(substring) => {
                write!(f, "could not encode substring: {:?}", substring)
            }
            Error::EncoderNotFound(encoder) => {
                write!(f, "encoder not found: {:?}", encoder)
            }
        }
    }
}
