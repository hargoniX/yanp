use crate::sentences::{GeneralSentence, SentenceType};
use crate::errors::NmeaSentenceError;
use crate::parsers::parse_rmc;

/// An enum storing consisting of all NMEA sentence types
/// together with their corresponding data structs
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SentenceData<'a> {
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
    BOD(BodData<'a>),
    /// BWC - Bearing and Distance to Waypoint, Latitude, N/S, Longitude, E/W, UTC, Status
    BWC(BwcData<'a>),
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
    ROO(RooData<'a>),
    /// RMA - Recommended Minimum Navigation Information
    RMA(RmaData),
    /// RMB - Recommended Minimum Navigation Information
    RMB(RmbData<'a>),
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
    RTE(RteData<'a>),
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
    WPL(WplData<'a>),
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
/// Represents a UTC timestamp
pub struct GpsTime {
    pub hour: u8,
    pub minute: u8,
    pub second: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Represents a geographical position
pub struct GpsPosition {
    pub lat: f32,
    pub lat_dir: LatitudeDirection,
    pub lon: f32,
    pub lon_dir: LongitudeDirection,
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Represents the current date
pub struct GpsDate {
    pub day: u8,
    pub month: u8,
    pub year: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Represents a status in one of the RM messages
pub enum RmStatus {
    Autonomous,
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LatitudeDirection {
    North,
    South,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LongitudeDirection {
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GpsQuality {
    FixNotAvailable,
    Fix,
    DifferentialFix,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GllStatus {
    DataValid,
    DataInvalid,
    Precise
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GsaMode {
    FixNotAvailable,
    Fix2D,
    Fix3D,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GsaSelectionMode {
    Manual,
    Automatic,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SteerDirection {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArrivalStatus {
    Arrived,
    NotArrived,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RteMode {
    CompleteRoute,
    WorkingRoute,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataValidity {
    DataValid,
    DataInvalid,
}

impl LongitudeDirection {
    pub (crate) fn from_char(chr: char) -> Self {
        match chr {
            'E' => LongitudeDirection::East,
            _ => LongitudeDirection::West,
        }
    }
}

impl LatitudeDirection {
    pub (crate) fn from_char(chr: char) -> Self {
        match chr {
            'N' => LatitudeDirection::North,
            _ => LatitudeDirection::South,
        }
    }
}

impl RmStatus {
    pub (crate) fn from_char(chr: char) -> Self {
        match chr {
            'A' => RmStatus::Autonomous,
            _ => RmStatus::Invalid, 
        }            
    }
}

impl GpsQuality {
    pub (crate) fn from_number(num: u8) -> Self {
        match num {
            0 => GpsQuality::FixNotAvailable,
            1 => GpsQuality::Fix,
            2 => GpsQuality::DifferentialFix,
        }
    }
}

impl GllStatus {
    pub (crate) fn from_char(chr: char) -> Self {
        match chr {
            'A' => GllStatus::DataValid,
            'V' => GllStatus::DataInvalid,
            'P' => GllStatus::Precise,
        }
    }
}

impl GsaMode {
    pub (crate) fn from_number(num: u8) -> Self {
        match num {
            1 => GsaMode::FixNotAvailable,
            2 => GsaMode::Fix2D,
            3 => GsaMode::Fix3D,
        }
    }
}

impl GsaSelectionMode {
    pub (crate) fn from_char(chr: char) -> Self {
        match chr {
            'M' => GsaSelectionMode::Manual,
            'A' => GsaSelectionMode::Automatic,
        }
    }
}

impl SteerDirection {
    pub (crate) fn from_char(chr: char) -> Self {
        match chr {
            'L' => SteerDirection::Left,
            _ => SteerDirection::Right,
        }
    }
}

impl ArrivalStatus {
    pub (crate) fn from_char(chr: char) -> Self {
        match chr {
            'A' => ArrivalStatus::Arrived,
            _ => ArrivalStatus::NotArrived,
        }
    }
}

impl RteMode {
    pub (crate) fn from_char(chr: char) -> Self {
        match chr {
            'c' => RteMode::CompleteRoute,
            'w' => RteMode::WorkingRoute,
        }
    }
}

impl DataValidity {
    pub (crate) fn from_char(chr: char) -> Self {
        match chr {
            'A' => DataValidity::DataValid,
            _ => DataValidity::DataInvalid,
        }
    }
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
pub struct BodData<'a> {
    pub bearing_true: Option<f32>,
    pub bearing_magnetic: Option<f32>,
    pub to_waypoint: &'a str,
    pub from_waypoint: &'a str,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BwcData<'a> {
    pub time: Option<GpsTime>,
    pub waypoint_position: GpsPosition,
    pub bearing_true: Option<f32>,
    pub bearing_magnetic: Option<f32>,
    pub nautical_miles: Option<f32>,
    pub waypoint: &'a str,
}
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
pub struct GgaData {
    pub time: Option<GpsTime>,
    pub position: GpsPosition,
    pub quality: Option<GpsQuality>,
    pub sats_in_view: Option<u8>,
    pub hdop: Option<f32>,
    pub altitude: Option<f32>,
    pub geoid_altitude: Option<f32>,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlcData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GllData {
    pub position: GpsPosition,
    pub time: Option<GpsTime>,
    pub status: Option<GllStatus>,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GnsData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GrsData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GstData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GsaData {
    pub selection_mode: Option<GsaSelectionMode>,
    pub mode: Option<GsaMode>,
    pub satellites: [Option<u8>; 12],
    pub pdob: Option<f32>,
    pub hdop: Option<f32>,
    pub vdop: Option<f32>,
}
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
pub struct HdtData {
    pub heading_true: Option<f32>,
}
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
pub struct RooData<'a> {
    pub waypoints: &'a [&'a str],
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RmaData {
    pub status: Option<RmStatus>,
    pub position: GpsPosition,
    pub time_diff_a: Option<f32>,
    pub time_diff_b: Option<f32>,
    pub speed: Option<f32>,
    pub heading: Option<f32>,
    pub magnetic_variation: Option<f32>,
    pub magnetic_direction: Option<LongitudeDirection>,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RmbData<'a> {
    pub status: Option<RmStatus>,
    pub cross_error: Option<u8>,
    pub steer_direction: Option<SteerDirection>,
    pub from_waypoint: Option<&'a str>,
    pub to_waypoint: Option<&'a str>,
    pub dest_position: GpsPosition,
    pub range_to_dest: Option<f32>,
    pub bearing: Option<f32>,
    pub closing_velocity: Option<f32>,
    pub arrival_status: Option<ArrivalStatus>,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RmcData {
    pub time: Option<GpsTime>,
    pub status: Option<RmStatus>,
    pub position: GpsPosition,
    pub speed: Option<f32>,
    pub heading: Option<f32>,
    pub date: Option<GpsDate>,
    pub magnetic_variation: Option<f32>,
    pub magnetic_direction: Option<LongitudeDirection>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RotData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RpmData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RsaData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RsdData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RteData<'a> {
    pub amount_messages: Option<u8>,
    pub sentence_number: Option<u8>,
    pub message_mode: Option<RteMode>,
    pub waypoints: &'a [&'a str],
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SfiData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StnData {
    pub talker_id: u8,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TllData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TrfData {
    pub time: Option<GpsTime>,
    pub date: Option<GpsDate>,
    pub position: Option<GpsPosition>,
    pub elevation: Option<f32>,
    pub iterations: Option<u8>,
    pub doppler_intervals: Option<u8>,
    pub update_distance: Option<u8>,
    pub satellite_id: Option<u8>,
    pub validity: Option<DataValidity>,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TtmData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VbwData {
    pub lon_water_speed: Option<f32>,
    pub transverse_water_speed: Option<f32>,
    pub water_validity: Option<DataValidity>,
    pub lon_ground_speed: Option<f32>,
    pub transverse_ground_speed: Option<f32>,
    pub ground_validity: Option<DataValidity>,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VdrData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VhwData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VlwData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VpwData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VtgData {
    pub bearing_true: Option<f32>,
    pub bearing_magnetic: Option<f32>,
    pub speed_knots: Option<f32>,
    pub speed_kmh: Option<f32>
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VwrData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WcvData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WncData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WplData<'a> {
    pub position: Option<GpsPosition>,
    pub waypoint_name: Option<&'a str>,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XdrData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XteData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XtrData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ZdaData {
   pub utc_time: Option<GpsTime>,
   pub utc_date: Option<GpsDate>,
   pub hour_offset: Option<u8>,
   pub minute_offset: Option<u8>,
}
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

macro_rules! sentence_parse_generator {
    ($sentence:ident : [$($TYPE:ident => $function:ident,)+]) => {
        match $sentence.sentence_type {
            $(
                SentenceType::$TYPE => Ok(SentenceData::$TYPE(parse_result_to_data($function($sentence.data))?)),
            )+
            _ => Err(NmeaSentenceError::TypeNotImplementedError($sentence.sentence_type)),
        }
    }
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

fn parse_result_to_data<'a, Data>(parse_result: Result<(&[u8], Data), nom::Err<(&[u8], nom::error::ErrorKind)>>) -> Result<Data, NmeaSentenceError<'a>> {
    
    match parse_result {
        Ok(value) => Ok(value.1),
        Err(_) => {
            Err(NmeaSentenceError::DataParsingError)
        }
    }
}

pub (crate) fn parse_sentence_data<'a>(general_sentence: GeneralSentence<'a>) -> Result<SentenceData, NmeaSentenceError<'a>> {
    sentence_parse_generator!(
        general_sentence: [
            //AAM => parse_aam,
            //ABK => parse_abk,
            //ACK => parse_ack,
            //ALM => parse_alm,
            //APA => parse_apa,
            //APB => parse_apb,
            //BEC => parse_bec,
            BOD => parse_bod,
            BWC => parse_bwc,
            //BWR => parse_bwr,
            //BWW => parse_bww,
            //DBK => parse_dbk,
            //DBS => parse_dbs,
            //DBT => parse_dbt,
            //DCN => parse_dcn,
            //DPT => parse_dpt,
            //DTM => parse_dtm,
            //FSI => parse_fsi,
            //GBS => parse_gbs,
            GGA => parse_gga,
            //GLC => parse_glc,
            GLL => parse_gll,
            //GNS => parse_gns,
            //GRS => parse_grs,
            //GST => parse_gst,
            GSA => parse_gsa,
            GSV => parse_gsv,
            //GTD => parse_gtd,
            //GXA => parse_gxa,
            //HDG => parse_hdg,
            //HDM => parse_hdm,
            HDT => parse_hdt,
            //HSC => parse_hsc,
            //LCD => parse_lcd,
            //MSK => parse_msk,
            //MTW => parse_mtw,
            //MWV => parse_mwv,
            //OLN => parse_oln,
            //OSD => parse_osd,
            ROO => parse_roo,
            RMA => parse_rma,
            RMB => parse_rmb,
            RMC => parse_rmc,
            //ROT => parse_rot,
            //RPM => parse_rpm,
            //RSA => parse_rsa,
            //RSD => parse_rsd,
            RTE => parse_rte,
            //SFI => parse_sfi,
            STN => parse_stn,
            //TLL => parse_tll,
            TRF => parse_trf,
            //TTM => parse_ttm,
            VBW => parse_vbw,
            //VDR => parse_vdr,
            //VHW => parse_vhw,
            //VLW => parse_vlw,
            //VPW => parse_vpw,
            VTG => parse_vtg,
            //VWR => parse_vwr,
            //WCV => parse_wcv,
            //WNC => parse_wnc,
            WPL => parse_wpl,
            //XDR => parse_xdr,
            //XTE => parse_xte,
            //XTR => parse_xtr,
            ZDA => parse_zda,
            //ZFO => parse_zfo,
            //ZTG => parse_ztg,
        ]
    )
}