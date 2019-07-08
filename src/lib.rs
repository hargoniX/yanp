#![no_std]

use combine::error::ParseError;
use combine::parser::char::{char, digit};
use combine::stream::state::State;
use combine::{choice, many, optional, Parser, Stream};

pub enum SentenceType {
    BOD,
    PBWC,
    PGGA,
    PGLL,
    PGSA,
    PGSV,
    PHDT,
    PR00,
    PRMA,
    PRMB,
    PRMC,
    PRTE,
    PTRF,
    PSTN,
    PVBW,
    PVTG,
    PWPL,
    PXTE,
    PZDA,
}

pub enum ParseResult<'a> {
    BOD(BodData<'a>),
    Unsupported(SentenceType)
}

#[derive(Debug)]
/// Bearing Origin to Destination
pub struct BodData<'a> {
    /// True bearing degrees
    pub true_bearing: Option<f32>,
    /// Magnet bearing degrees
    pub magnetic_bearing: Option<f32>,
    /// target waypoint
    pub target: Option<&'a str>,
    /// origin waypoint
    pub origin: Option<&'a str>,
}

#[derive(Debug)]
/// Bearing and Distance to Waypoint
pub struct BWC {}

#[derive(Debug)]
/// Global Positioning System Fix Data
pub struct GGA {}

#[derive(Debug)]
/// Geographic Position - Latitude/Longitude
pub struct GLL {}

#[derive(Debug)]
/// GPS DOP and active satellites
pub struct GSA {}

#[derive(Debug)]
/// Satellites in view
pub struct GSV {}

#[derive(Debug)]
/// Heading - True
pub struct HDT {}

#[derive(Debug)]
/// Waypoints in active route
pub struct R00 {}

#[derive(Debug)]
/// Recommended minimum specific Loran-C data
pub struct RMA {}

#[derive(Debug)]
/// Recommended minimum navigation info
pub struct RMB {}

#[derive(Debug)]
/// Recommended minimum specific GPS/Transit data
pub struct RMC {}

#[derive(Debug)]
/// Routes
pub struct RTE {}

#[derive(Debug)]
/// TRANSIT Fix Data
pub struct TRF {}

#[derive(Debug)]
/// Multiple Data ID
pub struct STN {}

#[derive(Debug)]
/// Dual Ground/Water Speed
pub struct VBW {}

#[derive(Debug)]
/// Track made good and Ground speed
pub struct VTG {}

#[derive(Debug)]
/// Waypoint Location
pub struct WPL {}

#[derive(Debug)]
/// Cross-Track Error, Measured
pub struct XTE {}

#[derive(Debug)]
/// Time & Date
pub struct ZDA {}

fn parse_sentence(sentence: &str) -> ParseResult {
    
}

pub fn parse(sentence: &str) -> Result<ParseResult, &str> {
    if sentence.len() > 102 {
        return Err("Sentence too long");
    }

    let res = parse_sentence(sentence);

    Ok(res)
}