#![deny(warnings)]
#![no_std]
pub mod sentences;
pub mod parse;
pub mod errors;

use errors::NmeaSentenceError;
use parse::{SentenceData, parse_sentence_data};
use sentences::GeneralSentence;

/// The central entrypoint for the library, it verifies and parses a given sentence
/// into a specific data struct for the user
pub fn parse_nmea_sentence(sentence: &[u8]) -> Result<SentenceData, NmeaSentenceError> {
    parse_sentence_data(GeneralSentence::new(sentence)?)
}