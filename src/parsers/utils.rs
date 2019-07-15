use crate::parse::*;
use nom::{one_of, named, map_res};
use crate::errors::NmeaSentenceError;

pub (crate) fn parse_num<I: core::str::FromStr>(data: &[u8]) -> Result<I, NmeaSentenceError> {
    str::parse::<I>(unsafe { core::str::from_utf8_unchecked(data) }).map_err(|_| NmeaSentenceError::GeneralParsingError)
}

macro_rules! translate_option {
    ($input:expr, $status:ident) => {
        match $input {
            Some(value) => Some($status::try_from(value)?),
            None => None
        }
    }
}

named!(pub (crate) parse_utc_stamp<GpsTime>,
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

named!(pub (crate) parse_date<GpsDate>, 
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

named!(pub (crate) parse_gps_position<GpsPosition>,
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