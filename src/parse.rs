use crate::sentences:: {GeneralSentence, SentenceType};
use crate::errors::NmeaSentenceError;
use crate::parsers;

macro_rules! status {
    ($($name:ident, $type:ty : [$($input:tt => $status:ident),+ error: $error:ident]),+) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum StatusParsingError {
            $(
                $error,
            )+
        }
        $(  
            #[derive(Debug, Clone, Copy, PartialEq)]
            pub enum $name {
                $(
                    $status,
                )+
            } 

            impl $name {
                #[allow(unused)]
                pub (crate) fn try_from<'a>(num: $type) -> Result<Self, NmeaSentenceError<'a>> {
                    match num {
                        $(
                            $input => Ok($name::$status),
                        )+
                        _ => Err(NmeaSentenceError::StatusParsingError(StatusParsingError::$error)),
                    }
                }
            }
        )+
    }
}

status! {
    RmStatus, char: [
        'A' => Active,
        'V' => Warning,
        'P' => Precise
        error: RmStatusError
    ],
    LatitudeDirection, char: [
        'N' => North,
        'S' => South
        error: LatitudeDirectionError
    ],
    LongitudeDirection, char: [
        'E' => East,
        'W' => West
        error: LongitudeDirectionError
    ],
    GpsQuality, u8: [
        0 => FixNotAvailable,
        1 => Fix,
        2 => DifferentialFix
        error: GpsQualityError
    ],
    GllStatus, char: [
        'A' => DataValid,
        'V' => DataInvalid,
        'P' => Precise
        error: GllStatusError
    ],
    GsaMode, u8: [
        1 => FixNotAvailable,
        2 => Fix2D,
        3 => Fix3D
        error: GsaModeError
    ],
    GsaSelectionMode, char: [
        'M' => Manual,
        'A' => Automatic
        error: GsaSelectionModeError
    ],
    SteerDirection, char: [
        'L' => Left,
        'R' => Right
        error: SteerDirectionError
    ],
    ArrivalStatus, char: [
        'A' => Arrived,
        'V' => NotArrived
        error: ArrivalStatusError
    ],
    RteMode, char: [
        'c' => CompleteRoute,
        'w' => WorkingRoute
        error: RteModeError
    ],
    DataValidity, char: [
        'A' => DataValid
        error: DataValidityError
    ]
}

/// An enum storing consisting of all NMEA sentence types
/// together with their corresponding data structs
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SentenceData<'a> {
    AAM(AamData),
    ABK(AbkData),
    ACK(AckData),
    ALM(AlmData),
    APA(ApaData),
    APB(ApbData),
    BEC(BecData),
    BOD(BodData<'a>),
    BWC(BwcData<'a>),
    BWR(BwrData),
    BWW(BwwData),
    DBK(DbkData),
    DBS(DbsData),
    DBT(DbtData),
    DCN(DcnData),
    DPT(DptData),
    DTM(DtmData),
    FSI(FsiData),
    GBS(GbsData),
    GGA(GgaData),
    GLC(GlcData),
    GLL(GllData),
    GNS(GnsData),
    GRS(GrsData),
    GST(GstData),
    GSA(GsaData),
    GSV(GsvData),
    GTD(GtdData),
    GXA(GxaData),
    HDG(HdgData),
    HDM(HdmData),
    HDT(HdtData),
    HSC(HscData),
    LCD(LcdData),
    MSK(MskData),
    MTW(MtwData),
    MWV(MwvData),
    OLN(OlnData),
    OSD(OsdData),
    ROO(RooData),
    RMA(RmaData),
    RMB(RmbData<'a>),
    RMC(RmcData),
    ROT(RotData),
    RPM(RpmData),
    RSA(RsaData),
    RSD(RsdData),
    RTE(RteData),
    SFI(SfiData),
    STN(StnData),
    TLL(TllData),
    TTM(TtmData),
    VBW(VbwData),
    VDR(VdrData),
    VHW(VhwData),
    VLW(VlwData),
    VPW(VpwData),
    VTG(VtgData),
    VWR(VwrData),
    WCV(WcvData),
    WNC(WncData),
    WPL(WplData<'a>),
    XDR(XdrData),
    XTE(XteData),
    XTR(XtrData),
    ZDA(ZdaData),
    ZFO(ZfoData),
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
    pub to_waypoint: Option<&'a [u8]>,
    pub from_waypoint: Option<&'a [u8]>,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BwcData<'a> {
    pub time: Option<GpsTime>,
    pub waypoint_position: GpsPosition,
    pub bearing_true: Option<f32>,
    pub bearing_magnetic: Option<f32>,
    pub nautical_miles: Option<f32>,
    pub waypoint: Option<&'a [u8]>,
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
    pub age_of_differential: Option<f32>,
    pub differential_station_id: Option<u16>,
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
pub struct GtdData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GsvData {}
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
pub struct RooData {}
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
    pub cross_error: Option<f32>,
    pub steer_direction: Option<SteerDirection>,
    pub to_waypoint: Option<&'a [u8]>,
    pub from_waypoint: Option<&'a [u8]>,
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
pub struct RteData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SfiData {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StnData {
    pub talker_id: u8,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TllData {}
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
    pub waypoint_name: Option<&'a [u8]>,
}
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

macro_rules! sentence_parse_generator {
    ($sentence:ident : [$($TYPE:ident => $function:path,)+]) => {
        match $sentence.sentence_type {
            $(
                SentenceType::$TYPE => Ok(SentenceData::$TYPE(parse_result_to_data($function($sentence.data))?)),
            )+
            _ => Err(NmeaSentenceError::TypeNotImplementedError($sentence.sentence_type)),
        }
    }
}


fn parse_result_to_data<'a, Data>(parse_result: Result<(&'a [u8], Data), nom::Err<(&'a [u8], nom::error::ErrorKind)>>) -> Result<Data, NmeaSentenceError<'a>> {
    
    match parse_result {
        Ok(value) => Ok(value.1),
        Err(val) => {
            Err(NmeaSentenceError::DataParsingError(val))
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
            BOD => parsers::bod::parse_bod,
            BWC => parsers::bwc::parse_bwc,
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
            GGA => parsers::gga::parse_gga,
            //GLC => parse_glc,
            GLL => parsers::gll::parse_gll,
            //GNS => parse_gns,
            //GRS => parse_grs,
            //GST => parse_gst,
            GSA => parsers::gsa::parse_gsa,
            //GTD => parse_gtd,
            //GXA => parse_gxa,
            //HDG => parse_hdg,
            //HDM => parse_hdm,
            HDT => parsers::hdt::parse_hdt,
            //HSC => parse_hsc,
            //LCD => parse_lcd,
            //MSK => parse_msk,
            //MTW => parse_mtw,
            //MWV => parse_mwv,
            //OLN => parse_oln,
            //OSD => parse_osd,
            //ROO => parse_roo,
            RMA => parsers::rma::parse_rma,
            RMB => parsers::rmb::parse_rmb,
            RMC => parsers::rmc::parse_rmc,
            //ROT => parse_rot,
            //RPM => parse_rpm,
            //RSA => parse_rsa,
            //RSD => parse_rsd,
            //RTE => parse_rte,
            //SFI => parse_sfi,
            STN => parsers::stn::parse_stn,
            //TLL => parse_tll,
            //TTM => parse_ttm,
            VBW => parsers::vbw::parse_vbw,
            //VDR => parse_vdr,
            //VHW => parse_vhw,
            //VLW => parse_vlw,
            //VPW => parse_vpw,
            VTG => parsers::vtg::parse_vtg,
            //VWR => parse_vwr,
            //WCV => parse_wcv,
            //WNC => parse_wnc,
            WPL => parsers::wpl::parse_wpl,
            //XDR => parse_xdr,
            //XTE => parse_xte,
            //XTR => parse_xtr,
            //ZDA => parse_zda,
            //ZFO => parse_zfo,
            //ZTG => parse_ztg,
        ]
    )
}