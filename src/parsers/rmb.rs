use crate::parse::*;
use nom::{one_of, named, map_res};
use crate::errors::NmeaSentenceError;
use super::utils::*;

fn build_rmb<'a>(sentence: (Option<char>, Option<f32>, Option<char>, Option<&'a [u8]>, Option<&'a [u8]>, GpsPosition, Option<f32>, Option<f32>, Option<f32>, Option<char>)) -> Result<RmbData<'a>, NmeaSentenceError<'a>> {
    Ok(RmbData {
        status: translate_option!(sentence.0, RmStatus),
        cross_error: sentence.1,
        steer_direction: translate_option!(sentence.2, SteerDirection),
        to_waypoint: sentence.3,
        from_waypoint: sentence.4,
        dest_position: sentence.5,
        range_to_dest: sentence.6,
        bearing: sentence.7,
        closing_velocity: sentence.8,
        arrival_status: translate_option!(sentence.9, ArrivalStatus),
    })
}

named!(pub (crate) parse_rmb<RmbData>,
    map_res!(
        do_parse!(
            status: opt!(one_of!("AVP")) >>
            char!(',') >>
            cross_error: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            steer_direction: opt!(one_of!("LR")) >>
            char!(',') >>
            to_waypoint: opt!(take_until!(",")) >>
            char!(',') >>
            from_waypoint: opt!(take_until!(",")) >>
            char!(',') >>
            dest_position: complete!(parse_gps_position) >>
            char!(',') >>
            range_to_dest: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            bearing: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            closing_velocity: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            arrival_status: opt!(one_of!("AV")) >>
            char!('*') >>
            (status, cross_error, steer_direction, to_waypoint, from_waypoint, dest_position, range_to_dest, bearing, closing_velocity, arrival_status)
        ),
        build_rmb
    )
);