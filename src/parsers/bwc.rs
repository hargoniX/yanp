use crate::parse::*;
use crate::errors::NmeaSentenceError;
use super::utils::*;

fn build_bwc<'a>(sentence: (Option<GpsTime>, GpsPosition, Option<f32>, Option<f32>, Option<f32>, Option<&'a [u8]>)) -> Result<BwcData<'a>, NmeaSentenceError<'a>> {
    Ok(BwcData{
        time: sentence.0,
        waypoint_position: sentence.1,
        bearing_true: sentence.2,
        bearing_magnetic: sentence.3,
        nautical_miles: sentence.4,
        waypoint: sentence.5,
    })
}

named!(pub (crate) parse_bwc<BwcData>,
    map_res!(
        do_parse!(
            time: opt!(complete!(parse_utc_stamp)) >>
            char!(',') >>
            position: complete!(parse_gps_position) >>
            char!(',') >>
            bearing_true: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            tag!(",T,") >>
            bearing_magnetic: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            tag!(",M,") >>
            nautical_miles: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            tag!(",N,") >>
            waypoint: opt!(take_until!("*")) >>
            (time, position, bearing_true, bearing_magnetic, nautical_miles, waypoint)
        ),
        build_bwc
    )
);