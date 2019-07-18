use super::utils::*;
use crate::errors::NmeaSentenceError;
use crate::parse::*;

fn build_gll<'a>(
    sentence: (GpsPosition, Option<GpsTime>, Option<char>),
) -> Result<GllData, NmeaSentenceError<'a>> {
    Ok(GllData {
        position: sentence.0,
        time: sentence.1,
        status: translate_option!(sentence.2, GllStatus),
    })
}

named!(pub (crate) parse_gll<GllData>,
    map_res!(
        do_parse!(
            position: complete!(parse_gps_position) >>
            char!(',') >>
            time: opt!(complete!(parse_utc_stamp)) >>
            char!(',') >>
            status: opt!(one_of!("AVP")) >>
            opt!(take_until!("*")) >>
            char!('*') >>
            (position, time, status)
        ),
        build_gll
    )
);
