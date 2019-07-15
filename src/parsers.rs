use crate::parse::*;
use crate::errors::NmeaSentenceError;

use nom::{one_of, named, map_res, tag};

fn parse_num<I: core::str::FromStr>(data: &[u8]) -> Result<I, NmeaSentenceError> {
    str::parse::<I>(unsafe { core::str::from_utf8_unchecked(data) }).map_err(|_| NmeaSentenceError::GeneralParsingError)
}

named!(parse_utc_stamp<GpsTime>,
    map_res!(
        do_parse!(
            hour: map_res!(take!(2), parse_num::<u8>) >>
            minute:  map_res!(take!(2), parse_num::<u8>) >>
            second:  map_res!(take_until!(","), parse_num::<f32>) >>
            (hour, minute, second)
        ),
        |timestamp: (u8, u8, f32)| -> Result<GpsTime, NmeaSentenceError>{
            
            Ok(GpsTime{
                hour: timestamp.0,
                minute: timestamp.1,
                second: timestamp.2,
            })
        }
    )
);

named!(parse_date<GpsDate>, 
    map_res!(
        do_parse!(
            day: map_res!(take!(2), parse_num::<u8>) >>
            month: map_res!(take!(2), parse_num::<u8>) >>
            year: map_res!(take!(2), parse_num::<u64>) >>
            (day, month, year)
        ),
        |data: (u8, u8, u64)| -> Result<GpsDate, NmeaSentenceError>{
            Ok(GpsDate {
                day: data.0,
                month: data.1,
                year: data.2,
            })
        }
    )
);

named!(parse_gps_position<GpsPosition>,
    map_res!(
        do_parse!(
            lat_deg: map_res!(take!(2), parse_num::<u8>) >>
            lat_min: map_res!(take_until!(","), parse_num::<f32>) >>
            char!(',') >>
            lat_dir: one_of!("NS") >>
            char!(',') >>
            lon_deg: map_res!(take!(3), parse_num::<u8>) >>
            lon_min: map_res!(take_until!(","), parse_num::<f32>) >>
            char!(',') >>
            lon_dir: one_of!("EW") >>
            (lat_deg, lat_min, lat_dir, lon_deg, lon_min, lon_dir)
        ),
        | position: (u8, f32, char, u8, f32, char) | -> Result<GpsPosition, NmeaSentenceError>{
            Ok(GpsPosition{
                lat: (position.0 as f32) + position.1 / 60.,
                lat_dir: LatitudeDirection::try_from(position.2)?,
                lon: (position.3 as f32) + position.4 / 60.,
                lon_dir: LongitudeDirection::try_from(position.5)?,
            })
        }
    )
);

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

fn build_rmc<'a>(sentence: (Option<GpsTime>, Option<char>, GpsPosition, Option<f32>, Option<f32>, Option<GpsDate>, Option<f32>, Option<char>)) -> Result<RmcData, NmeaSentenceError<'a>> {
    Ok(RmcData{
        time: sentence.0,
        status: match sentence.1 {
            Some(status) => Some(RmStatus::try_from(status)?),
            None => None
        },
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
            status: opt!(one_of!("AV")) >>
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