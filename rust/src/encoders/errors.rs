use std::fmt;

#[derive(Debug)]
pub enum Error {
    CharacterNotInAlphabet(char),
    NoEncoderFitAlphabet(String),
    CouldNotEncode(String),
    CouldNotEncodeSubstring(String),
    EncoderNotFound(String),
    DecoderNotFound(char),
    CouldNotDecodeEmptyString,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CharacterNotInAlphabet(ch) => {
                write!(f, "character {:?} is not in the alphabet", ch)
            }
            Error::NoEncoderFitAlphabet(payload) => {
                write!(f, "no encoder fits the alphabet of the payload: {:?}", payload)
            }
            Error::CouldNotEncode(payload) => {
                write!(f, "could not encode payload: {:?}", payload)
            }
            Error::CouldNotEncodeSubstring(substring) => {
                write!(f, "could not encode substring: {:?}", substring)
            }
            Error::EncoderNotFound(encoder) => {
                write!(f, "encoder not found: {:?}", encoder)
            }
            Error::DecoderNotFound(encoder_id) => {
                write!(f, "decoder not found: {:?}", encoder_id)
            }
            Error::CouldNotDecodeEmptyString => {
                write!(f, "could not decode empty payload")
            }
        }
    }
}
