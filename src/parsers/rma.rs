use crate::parse::*;
use crate::errors::NmeaSentenceError;
use super::utils::*;

fn build_rma<'a>(sentence: (Option<char>, GpsPosition, Option<f32>, Option<f32>, Option<f32>, Option<f32>, Option<f32>, Option<char>)) -> Result<RmaData, NmeaSentenceError<'a>> {
    Ok(RmaData {
        status: translate_option!(sentence.0, RmStatus),
        position: sentence.1,
        time_diff_a: sentence.2,
        time_diff_b: sentence.3,
        speed: sentence.4,
        heading: sentence.5,
        magnetic_variation: sentence.6,
        magnetic_direction: translate_option!(sentence.7, LongitudeDirection)
    })
}

named!(pub (crate) parse_rma<RmaData>,
    map_res!(
        do_parse!(
            status: opt!(one_of!("AVP")) >>
            char!(',') >>
            position: complete!(parse_gps_position) >>
            char!(',') >>
            time_diff_a: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            time_diff_b: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            speed: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            heading: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            magnetic_variation: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            magnetic_direction: opt!(one_of!("EW")) >>
            char!('*') >>
            (status, position, time_diff_a, time_diff_b, speed, heading, magnetic_variation, magnetic_direction)
        ),
        build_rma
    )
);