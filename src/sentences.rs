 use crate::errors::NmeaSentenceError;

macro_rules! general_sentences {
    ($($string_type:tt => $STYPE:ident),+) => {
        pub (crate) fn parse_general_sentence(sentence: &[u8]) -> Result<GeneralSentence, NmeaSentenceError> {
            let (prefix, rest) = sentence.split_at(7);
            let (data, checksum) = rest.split_at(rest.len() - 4);
            let checksum = parse_hex(&checksum[0..2])?;

             match &prefix[3..6] {
                 $(
                    $string_type => Ok(GeneralSentence {
                                            sentence_type: SentenceType::$STYPE,
                                            data: data,
                                            checksum: checksum,
                                            prefix: prefix
                                        }
                                     ),
                 )+
                _ => Err(NmeaSentenceError::UnkownTypeError(prefix)),
             }
        }

        /// Enum of all sentence type so the GeneralSentence struct
        /// can store the type without having to parse the data yet
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum SentenceType {
            $(
                $STYPE,
            )+
        }
    }
}

general_sentences!(
    b"AAM" => AAM,
    b"ABK" => ABK,
    b"ACK" => ACK,
    b"ALM" => ALM,
    b"APA" => APA,
    b"APB" => APB,
    b"BEC" => BEC,
    b"BOD" => BOD,
    b"BWC" => BWC,
    b"BWR" => BWR,
    b"BWW" => BWW,
    b"DBK" => DBK,
    b"DBS" => DBS,
    b"DBT" => DBT,
    b"DCN" => DCN,
    b"DPT" => DPT,
    b"DTM" => DTM,
    b"FSI" => FSI,
    b"GBS" => GBS,
    b"GGA" => GGA,
    b"GLC" => GLC,
    b"GLL" => GLL,
    b"GNS" => GNS,
    b"GRS" => GRS,
    b"GST" => GST,
    b"GSA" => GSA,
    b"GSV" => GSV,
    b"GTD" => GTD,
    b"GXA" => GXA,
    b"HDG" => HDG,
    b"HDM" => HDM,
    b"HDT" => HDT,
    b"HSC" => HSC,
    b"LCD" => LCD,
    b"MSK" => MSK,
    b"MTW" => MTW,
    b"MWV" => MWV,
    b"OLN" => OLN,
    b"OSD" => OSD,
    b"ROO" => ROO,
    b"RMA" => RMA,
    b"RMB" => RMB,
    b"RMC" => RMC,
    b"ROT" => ROT,
    b"RPM" => RPM,
    b"RSA" => RSA,
    b"RSD" => RSD,
    b"RTE" => RTE,
    b"SFI" => SFI,
    b"STN" => STN,
    b"TLL" => TLL,
    b"TRF" => TRF,
    b"TTM" => TTM,
    b"VBW" => VBW,
    b"VDR" => VDR,
    b"VHW" => VHW,
    b"VLW" => VLW,
    b"VPW" => VPW,
    b"VTG" => VTG,
    b"VWR" => VWR,
    b"WCV" => WCV,
    b"WNC" => WNC,
    b"WPL" => WPL,
    b"XDR" => XDR,
    b"XTE" => XTE,
    b"XTR" => XTR,
    b"ZDA" => ZDA,
    b"ZFO" => ZFO,
    b"ZTG" => ZTG
);

fn parse_hex(data: &[u8]) ->Result<u8, NmeaSentenceError> {
    u8::from_str_radix(unsafe {core::str::from_utf8_unchecked(data)}, 16)
        .map_err(|_| NmeaSentenceError::HexParsingError(data[0], data[1]))
}

#[derive(Debug, Clone, Copy)]
pub (crate) struct GeneralSentence<'a> {
    pub (crate) sentence_type: SentenceType,
    pub (crate) data: &'a [u8],
    pub (crate) checksum: u8,
    pub (crate) prefix: &'a [u8],
}

impl<'a> GeneralSentence<'a> {
    /// Generates a new GeneralSentence instance with verified checksum
    pub (crate) fn new(sentence: &'a [u8]) -> Result<Self, NmeaSentenceError> {
        if sentence.len() > 102 {
            return Err(NmeaSentenceError::SentenceLengthError(sentence.len()));
        }

        let parsed_sentence = parse_general_sentence(sentence)?;
        let calculated_checksum = parsed_sentence.calc_checksum();

        if calculated_checksum != parsed_sentence.checksum {
            return Err(NmeaSentenceError::ChecksumError(parsed_sentence.checksum, calculated_checksum));
        }

        Ok(parsed_sentence)
    }

    /// Calculates the actual checksum of a sentence
    fn calc_checksum(&self) -> u8 {
        let mut checksum = 0;
        for byte in &self.prefix[1..] {
            checksum = checksum ^ byte;
        }

        for byte in &self.data[..self.data.len() - 1] {
            checksum = checksum ^ byte;
        }

        checksum
    }
}