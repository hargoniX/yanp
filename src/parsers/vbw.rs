use crate::parse::*;
use crate::errors::NmeaSentenceError;
use super::utils::*;



fn build_vbw<'a>(sentence: (Option<&'a [u8]>, Option<&'a [u8]>, Option<char>, Option<&'a [u8]>, Option<&'a [u8]>, Option<char>)) -> Result<VbwData, NmeaSentenceError> {
    Ok(VbwData {
        lon_water_speed: invalid_height_check(sentence.0)?,
        transverse_water_speed: invalid_height_check(sentence.1)?,
        water_validity: Some(DataValidity::DataValid),
        lon_ground_speed: invalid_height_check(sentence.3)?,
        transverse_ground_speed: invalid_height_check(sentence.4)?,
        ground_validity: Some(DataValidity::DataValid),
    })
}

named!(pub (crate) parse_vbw<VbwData>,
    map_res!(
        do_parse!(
            lon_water_speed: opt!(take_until!(",")) >>
            char!(',') >>
            transverse_water_speed: opt!(take_until!(",")) >>
            char!(',') >>
            status: opt!(char!('A')) >>
            char!(',') >>
            lon_ground_speed: opt!(take_until!(",")) >>
            char!(',') >>
            transverse_ground_speed: opt!(take_until!(",")) >>
            char!(',') >>
            status2: opt!(char!('A')) >>
            char!('*') >>
            (lon_water_speed, transverse_water_speed, status, lon_ground_speed, transverse_ground_speed, status2)
        ),
        build_vbw
    )
);