use crate::parse::{parse_general_sentence};
use crate::errors::NmeaSentenceError;
/// Enum of all sentence type so the GeneralSentence struct
/// can store the type without having to parse the data yet
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SentenceType {
    /// AAM - Waypoint Arrival Alarm
    AAM,
    /// ABK - UAIS Addressed and binary broadcast acknowledgement
    ABK,
    /// ACK - Acknowledge Alarm
    ACK,
    /// ALM - GPS Almanac Data
    ALM,
    /// APA - Autopilot Sentence "A"
    APA,
    /// APB - Autopilot Sentence "B"
    APB,
    /// BEC - Bearing & Distance to Waypoint - Dead Reckoning
    BEC,
    /// BOD - Bearing Origin to Destination
    BOD,
    /// BWC - Bearing and Distance to Waypoint, Latitude, N/S, Longitude, E/W, UTC, Status
    BWC,
    /// BWR - Bearing and Distance to Waypoint - Rhumb Line, Latitude, N/S, Longitude, E/W, UTC, Status
    BWR,
    /// BWW - Bearing - Waypoint to Waypoint
    BWW,
    /// DBK - Depth Below Keel
    DBK,
    /// DBS - Depth Below Surface
    DBS,
    /// DBT - Depth below transducer
    DBT,
    /// DCN - Decca Position (obsolete)
    DCN,
    /// DPT - Depth of water
    DPT,
    /// DTM - Datum Reference
    DTM,
    /// FSI - Frequency Set Information
    FSI,
    /// GBS - GPS Satellite Fault Detection
    GBS,
    /// GGA - Global Positioning System Fix Data, Time, Position and fix related data fora GPS receiver.
    GGA,
    /// GLC - Geographic Position, Loran-C
    GLC,
    /// GLL - Geographic Position - Latitude/Longitude
    GLL,
    /// GNS - GNSS fixed data
    GNS,
    /// GRS - GNSS Range Residual
    GRS,
    /// GST - GNSS Pseudorange Error Statistics
    GST,
    /// GSA - GPS DOP and active satellites
    GSA,
    /// GSV - Satellites in view
    GSV,
    /// GTD - Geographic Location in Time Differences
    GTD,
    /// GXA - TRANSIT Position - Latitude/Longitude - Location and time of TRANSIT fix at waypoint (obsolete)
    GXA,
    /// HDG - Heading - Deviation & Variation
    HDG,
    /// HDM - Heading - Magnetic
    HDM,
    /// HDT - Heading - True
    HDT,
    /// HSC - Heading Steering Command
    HSC,
    /// LCD - Loran-C Signal Data
    LCD,
    /// MSK - MSK Receiver Interface (for DGPS Beacon Receivers)
    MSK,
    /// MTW - Water Temperature
    MTW,
    /// MWV - Wind Speed and Angle
    MWV,
    /// OLN - Omega Lane Numbers (obsolete)
    OLN,
    /// OSD - Own Ship Data
    OSD,
    /// R00 - Waypoints in active route
    ROO,
    /// RMA - Recommended Minimum Navigation Information
    RMA,
    /// RMB - Recommended Minimum Navigation Information
    RMB,
    /// RMC - Recommended Minimum Navigation Information
    RMC,
    /// ROT - Rate Of Turn
    ROT,
    /// RPM - Revolutions
    RPM,
    /// RSA - Rudder Sensor Angle
    RSA,
    /// RSD - RADAR System Data
    RSD,
    /// RTE - Routes
    RTE,
    /// SFI - Scanning Frequency Information
    SFI,
    /// STN - Multiple Data ID
    STN,
    /// TLL - Target latitude and longitude
    TLL,
    /// TRF - TRANSIT Fix Data (obsolete)
    TRF,
    /// TTM - Tracked Target Message
    TTM,
    /// VBW - Dual Ground/Water Speed
    VBW,
    /// VDR - Set and Drift
    VDR,
    /// VHW - Water speed and heading
    VHW,
    /// VLW - Distance Traveled through Water
    VLW,
    /// VPW - Speed - Measured Parallel to Wind
    VPW,
    /// VTG - Track made good and Ground speed
    VTG,
    /// VWR - Relative Wind Speed and Angle
    VWR,
    /// WCV - Waypoint Closure Velocity
    WCV,
    /// WNC - Distance - Waypoint to Waypoint
    WNC,
    /// WPL - Waypoint Location
    WPL,
    /// XDR - Transducer Measurement
    XDR,
    /// XTE - Cross-Track Error, Measured
    XTE,
    /// XTR - Cross Track Error - Dead Reckoning
    XTR,
    /// ZDA - Time & Date - UTC, day, month, year and local time zone
    ZDA,
    /// ZFO - UTC & Time from origin Waypoint
    ZFO,
    /// ZTG - UTC & Time to Destination Waypoint
    ZTG,
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

        for byte in self.data {
            checksum = checksum ^ byte;
        }

        checksum
    }
}