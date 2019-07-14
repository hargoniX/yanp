use crate::sentences::SentenceType;

/// A list of errors that can occur during the creation of
/// a GeneralSentence
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NmeaSentenceError<'a> {
    /// Is thrown if the sentence was longer than 102 characters.
    /// The integer associated with this is the length of the sentence
    SentenceLengthError(usize),
    /// Is thrown if the chekcsum of the parsed and the calculated one do not match up.
    /// The first u8 is the parsed one, the second one the calculated one
    ChecksumError(u8, u8),
    /// Is thrown if parsing a raw sentence into a GeneralSentence fails
    GeneralParsingError,
    /// Is thrown if no parser exists for a given NMEA sentence type yet,
    /// contains the sentence type for which no parser was found
    TypeNotImplementedError(SentenceType),
    /// Is thrown if the checksum couldnt be parsed as a valid hex number,
    /// contains both supposedly hex digits as their utf8 representation
    HexParsingError(u8, u8),
    /// Is thrown if the type of a sentence could not be matched against
    /// a list of already known types
    UnkownTypeError(&'a [u8]),
    /// Is thrown if parsing the data of a sentence into a data struct fails
    DataParsingError,
}