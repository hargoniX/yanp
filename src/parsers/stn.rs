use crate::parse::*;
use nom::{named, map_res, take_until};
use crate::errors::NmeaSentenceError;
use super::utils::*;

named!(pub (crate) parse_stn<StnData>,
    map_res!(
        do_parse!(
            talker_id: map_res!(take_until!("*"), parse_num::<u8>) >>
            char!('*') >>
            (talker_id)
        ),
        |sentence:(u8) | -> Result<StnData, NmeaSentenceError> {
            Ok(StnData {
                talker_id: sentence
            })
        }
    )
);