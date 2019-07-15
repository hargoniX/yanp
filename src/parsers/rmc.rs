use crate::parse::*;
use nom::{one_of, named, map_res};
use crate::errors::NmeaSentenceError;
use super::utils::*;

fn build_rmc<'a>(sentence: (Option<GpsTime>, Option<char>, GpsPosition, Option<f32>, Option<f32>, Option<GpsDate>, Option<f32>, Option<char>)) -> Result<RmcData, NmeaSentenceError<'a>> {
    Ok(RmcData{
        time: sentence.0,
        status: translate_option!(sentence.1, RmStatus),
        position: sentence.2,
        speed: sentence.3,
        heading: sentence.4,
        date: sentence.5,
        magnetic_variation: sentence.6,
        magnetic_direction: match sentence.7 {
            Some(direction) => Some(LongitudeDirection::try_from(direction)?),
            None => None
        }
    })
}

named!(pub (crate) parse_rmc<RmcData>, 
    map_res!(
        do_parse!(
            time: opt!(complete!(parse_utc_stamp)) >>
            char!(',') >>
            status: opt!(one_of!("AVP")) >>
            char!(',') >>
            position: complete!(parse_gps_position) >>
            char!(',') >>
            speed: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            heading: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            date: opt!(complete!(parse_date)) >>
            char!(',') >>
            magnetic_variation: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            magnetic_direction: opt!(one_of!("EW")) >>
            char!('*') >>
            (time, status, position, speed, heading, date, magnetic_variation, magnetic_direction)
        ),
        build_rmc
    )
);