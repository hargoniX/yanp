use crate::parse::*;
use crate::errors::NmeaSentenceError;
use super::utils::*;

fn build_wpl<'a>(sentence: (GpsPosition, Option<&'a [u8]>)) -> Result<WplData<'a>, NmeaSentenceError<'a>> {
    Ok(WplData{
        position: Some(sentence.0),
        waypoint_name: sentence.1,
    })
}

named!(pub (crate) parse_wpl<WplData>,
    map_res!(
        do_parse!(
            waypoint_position: complete!(parse_gps_position) >>
            char!(',') >>
            waypoint_id: opt!(take_until!("*")) >>
            char!('*') >>
            (waypoint_position, waypoint_id)
        ),
        build_wpl
    )
);