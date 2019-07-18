use super::utils::*;
use crate::errors::NmeaSentenceError;
use crate::parse::*;

#[cfg(feature = "alloc")]
fn build_gns<'a>(
    sentence: (
        Option<GpsTime>,
        GpsPosition,
        Option<&'a [u8]>,
        Option<u8>,
        Option<f32>,
        Option<f32>,
        Option<&'a [u8]>,
        Option<f32>,
        Option<u16>,
    ),
) -> Result<GnsData, NmeaSentenceError<'a>> {
    let mode = match sentence.2 {
        Some(value) => {
            let mut modes = alloc::vec::Vec::new();
            for byte in value {
                modes.push(GnsMode::try_from(*byte as char)?);
            }
            Some(modes)
        }
        None => None,
    };

    Ok(GnsData {
        time: sentence.0,
        position: sentence.1,
        mode: mode,
        sats_in_use: sentence.3,
        hdop: sentence.4,
        orthometric_height: sentence.5,
        geoid_seperation: invalid_height_check(sentence.6)?,
        age_of_differential: sentence.7,
        differential_station_id: sentence.8,
    })
}

#[cfg(not(feature = "alloc"))]
#[allow(unused)]
fn build_gns<'a>(
    _sentence: (
        Option<GpsTime>,
        GpsPosition,
        Option<&'a [u8]>,
        Option<u8>,
        Option<f32>,
        Option<f32>,
        Option<&'a [u8]>,
        Option<f32>,
        Option<u16>,
    ),
) -> Result<GnsData, NmeaSentenceError<'a>> {
    use crate::sentences::SentenceType;
    Err(NmeaSentenceError::TypeNotImplementedError(
        SentenceType::GNS,
    ))
}

named!(pub (crate) parse_gns<GnsData>,
    map_res!(
        do_parse!(
            time: opt!(complete!(parse_utc_stamp)) >>
            char!(',') >>
            position: complete!(parse_gps_position) >>
            char!(',') >>
            mode: opt!(take_until!(",")) >>
            char!(',') >>
            sats_in_use: opt!(map_res!(take_until!(","), parse_num::<u8>)) >>
            char!(',') >>
            hdop: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            orthometric_height: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            geoid_altitude: opt!(take_until!(",")) >>
            char!(',') >>
            age_of_differential: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            reference_station_id: opt!(map_res!(take_until!("*"), parse_num::<u16>)) >>
            char!('*') >>
            (time, position, mode, sats_in_use, hdop, orthometric_height, geoid_altitude, age_of_differential, reference_station_id)
        ),
        build_gns
    )
);
