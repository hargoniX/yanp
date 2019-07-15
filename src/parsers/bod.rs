use crate::parse::*;
use nom::{named, map_res, tag};
use crate::errors::NmeaSentenceError;
use super::utils::*;

fn build_bod<'a>(sentence: (Option<f32>, Option<f32>, Option<&'a [u8]>, Option<&'a [u8]>)) -> Result<BodData<'a>, NmeaSentenceError<'a>> {
    Ok(BodData{
        bearing_true: sentence.0,
        bearing_magnetic: sentence.1,
        to_waypoint: sentence.2,
        from_waypoint: sentence.3,
    })
}

named!(pub (crate) parse_bod<BodData>,
    map_res!(
        do_parse!(
            bearing_true: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            tag!(",T,") >>
            bearing_magnetic: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            tag!(",M,") >>
            to_waypoint: opt!(take_until!(",")) >>
            char!(',') >>
            from_waypoint: opt!(take_until!("*")) >>
            (bearing_true, bearing_magnetic, to_waypoint, from_waypoint)
        ),
        build_bod
    )
);