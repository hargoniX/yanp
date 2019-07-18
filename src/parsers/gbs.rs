use super::utils::*;
use crate::errors::NmeaSentenceError;
use crate::parse::*;

fn build_gbs<'a>(
    sentence: (
        Option<GpsTime>,
        Option<f32>,
        Option<f32>,
        Option<f32>,
        Option<u8>,
        Option<f32>,
        Option<f32>,
        Option<f32>,
    ),
) -> Result<GbsData, NmeaSentenceError<'a>> {
    Ok(GbsData {
        time: sentence.0,
        lat_error: sentence.1,
        lon_error: sentence.2,
        alt_error: sentence.3,
        most_likely_failed_sat: sentence.4,
        missed_probability: sentence.5,
        bias_estimate: sentence.6,
        bias_standard_deviation: sentence.7,
    })
}

named!(pub (crate) parse_gbs<GbsData>,
    map_res!(
        do_parse!(
            time: opt!(complete!(parse_utc_stamp)) >>
            char!(',') >>
            lat_error: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            lon_error: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            alt_error: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            most_likely_failed_sat: opt!(map_res!(take_until!(","), parse_num::<u8>)) >>
            char!(',') >>
            missed_probability: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            bias_estimate: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            bias_standard_deviation: opt!(map_res!(take_until!("*"), parse_num::<f32>)) >>
            char!('*') >>
            (time, lat_error, lon_error, alt_error, most_likely_failed_sat, missed_probability, bias_estimate, bias_standard_deviation)
        ),
        build_gbs
    )
);
