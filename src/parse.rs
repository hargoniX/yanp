use crate::sentences::{GeneralSentence, SentenceType};
use crate::errors::NmeaSentenceError;

/// An enum storing consisting of all NMEA sentence types
/// together with their corresponding data structs
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SentenceData {
    /// AAM - Waypoint Arrival Alarm
    AAM(AamData),
    /// ABK - UAIS Addressed and binary broadcast acknowledgement
    ABK(AbkData),
    /// ACK - Acknowledge Alarm
    ACK(AckData),
    /// ALM - GPS Almanac Data
    ALM(AlmData),
    /// APA - Autopilot Sentence "A"
    APA(ApaData),
    /// APB - Autopilot Sentence "B"
    APB(ApbData),
    /// BEC - Bearing & Distance to Waypoint - Dead Reckoning
    BEC(BecData),
    /// BOD - Bearing Origin to Destination
    BOD(BodData),
    /// BWC - Bearing and Distance to Waypoint, Latitude, N/S, Longitude, E/W, UTC, Status
    BWC(BwcData),
    /// BWR - Bearing and Distance to Waypoint - Rhumb Line, Latitude, N/S, Longitude, E/W, UTC, Status
    BWR(BwrData),
    /// BWW - Bearing - Waypoint to Waypoint
    BWW(BwwData),
    /// DBK - Depth Below Keel
    DBK(DbkData),
    /// DBS - Depth Below Surface
    DBS(DbsData),
    /// DBT - Depth below transducer
    DBT(DbtData),
    /// DCN - Decca Position (obsolete)
    DCN(DcnData),
    /// DPT - Depth of water
    DPT(DptData),
    /// DTM - Datum Reference
    DTM(DtmData),
    /// FSI - Frequency Set Information
    FSI(FsiData),
    /// GBS - GPS Satellite Fault Detection
    GBS(GbsData),
    /// GGA - Global Positioning System Fix Data, Time, Position and fix related data fora GPS receiver.
    GGA(GgaData),
    /// GLC - Geographic Position, Loran-C
    GLC(GlcData),
    /// GLL - Geographic Position - Latitude/Longitude
    GLL(GllData),
    /// GNS - GNSS fixed data
    GNS(GnsData),
    /// GRS - GNSS Range Residual
    GRS(GrsData),
    /// GST - GNSS Pseudorange Error Statistics
    GST(GstData),
    /// GSA - GPS DOP and active satellites
    GSA(GsaData),
    /// GSV - Satellites in view
    GSV(GsvData),
    /// GTD - Geographic Location in Time Differences
    GTD(GtdData),
    /// GXA - TRANSIT Position - Latitude/Longitude - Location and time of TRANSIT fix at waypoint (obsolete)
    GXA(GxaData),
    /// HDG - Heading - Deviation & Variation
    HDG(HdgData),
    /// HDM - Heading - Magnetic
    HDM(HdmData),
    /// HDT - Heading - True
    HDT(HdtData),
    /// HSC - Heading Steering Command
    HSC(HscData),
    /// LCD - Loran-C Signal Data
    LCD(LcdData),
    /// MSK - MSK Receiver Interface (for DGPS Beacon Receivers)
    MSK(MskData),
    /// MTW - Water Temperature
    MTW(MtwData),
    /// MWV - Wind Speed and Angle
    MWV(MwvData),
    /// OLN - Omega Lane Numbers (obsolete)
    OLN(OlnData),
    /// OSD - Own Ship Data
    OSD(OsdData),
    /// R00 - Waypoints in active route
    ROO(RooData),
    /// RMA - Recommended Minimum Navigation Information
    RMA(RmaData),
    /// RMB - Recommended Minimum Navigation Information
    RMB(RmbData),
    /// RMC - Recommended Minimum Navigation Information
    RMC(RmcData),
    /// ROT - Rate Of Turn
    ROT(RotData),
    /// RPM - Revolutions
    RPM(RpmData),
    /// RSA - Rudder Sensor Angle
    RSA(RsaData),
    /// RSD - RADAR System Data
    RSD(RsdData),
    /// RTE - Routes
    RTE(RteData),
    /// SFI - Scanning Frequency Information
    SFI(SfiData),
    /// STN - Multiple Data ID
    STN(StnData),
    /// TLL - Target latitude and longitude
    TLL(TllData),
    /// TRF - TRANSIT Fix Data (obsolete)
    TRF(TrfData),
    /// TTM - Tracked Target Message
    TTM(TtmData),
    /// VBW - Dual Ground/Water Speed
    VBW(VbwData),
    /// VDR - Set and Drift
    VDR(VdrData),
    /// VHW - Water speed and heading
    VHW(VhwData),
    /// VLW - Distance Traveled through Water
    VLW(VlwData),
    /// VPW - Speed - Measured Parallel to Wind
    VPW(VpwData),
    /// VTG - Track made good and Ground speed
    VTG(VtgData),
    /// VWR - Relative Wind Speed and Angle
    VWR(VwrData),
    /// WCV - Waypoint Closure Velocity
    WCV(WcvData),
    /// WNC - Distance - Waypoint to Waypoint
    WNC(WncData),
    /// WPL - Waypoint Location
    WPL(WplData),
    /// XDR - Transducer Measurement
    XDR(XdrData),
    /// XTE - Cross-Track Error, Measured
    XTE(XteData),
    /// XTR - Cross Track Error - Dead Reckoning
    XTR(XtrData),
    /// ZDA - Time & Date - UTC, day, month, year and local time zone
    ZDA(ZdaData),
    /// ZFO - UTC & Time from origin Waypoint
    ZFO(ZfoData),
    /// ZTG - UTC & Time to Destination Waypoint
    ZTG(ZtgData),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AamData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AbkData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AckData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AlmData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ApaData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ApbData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BecData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BodData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BwcData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BwrData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BwwData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DbkData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DbsData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DbtData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DcnData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DptData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DtmData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FsiData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GbsData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GgaData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlcData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GllData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GnsData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GrsData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GstData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GsaData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GsvData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GtdData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GxaData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HdgData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HdmData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HdtData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HscData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LcdData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MskData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MtwData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MwvData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OlnData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OsdData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RooData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RmaData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RmbData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RmcData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RotData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RpmData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RsaData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RsdData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RteData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SfiData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StnData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TllData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TrfData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TtmData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VbwData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VdrData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VhwData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VlwData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VpwData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VtgData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VwrData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WcvData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WncData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WplData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XdrData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XteData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XtrData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ZdaData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ZfoData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ZtgData {}

macro_rules! prefix_match_generator {
    ($prefix:ident, $data:ident, $checksum:ident: [$($type_literal:tt => $STYPE:ident,)+]) => {
        match &$prefix[3..6] {
            $(
                $type_literal => Ok(GeneralSentence {
                                        sentence_type: SentenceType::$STYPE,
                                        data: $data,
                                        checksum: $checksum,
                                        prefix: $prefix
                                    }
                                 ),
            )+
            _ => Err(NmeaSentenceError::UnkownTypeError($prefix)),
        }
    };
}

fn parse_hex(data: &[u8]) ->Result<u8, NmeaSentenceError> {
    u8::from_str_radix(unsafe {core::str::from_utf8_unchecked(data)}, 16)
        .map_err(|_| NmeaSentenceError::HexParsingError(data[0], data[1]))
}

pub (crate) fn parse_general_sentence(sentence: &[u8]) -> Result<GeneralSentence, NmeaSentenceError> {
    
    let (prefix, rest) = sentence.split_at(7);
    let (data, checksum) = rest.split_at(rest.len() - 5);
    let checksum = parse_hex(&checksum[1..3])?;

    prefix_match_generator!(prefix, data, checksum: [
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
        b"ZTG" => ZTG,
    ])
}

pub (crate) fn parse_sentence_data<'a>(general_sentence: GeneralSentence) -> Result<SentenceData, NmeaSentenceError<'a>> {
    match general_sentence.sentence_type {
        _ => Err(NmeaSentenceError::TypeNotImplementedError(general_sentence.sentence_type)),
    }
}