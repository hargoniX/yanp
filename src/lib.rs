#![deny(warnings)]
#![no_std]

#[macro_use]
extern crate nom;
use errors::NmeaSentenceError;
use parse::{parse_sentence_data, SentenceData};
use sentences::GeneralSentence;
#[cfg(feature = "alloc")]
extern crate alloc;

pub mod errors;
pub mod parse;
mod parsers;
pub mod sentences;

/// The central entrypoint for the library, it verifies and parses a given sentence
/// into a specific data struct for the user
pub fn parse_nmea_sentence(sentence: &[u8]) -> Result<SentenceData, NmeaSentenceError> {
    parse_sentence_data(GeneralSentence::new(sentence)?)
}
