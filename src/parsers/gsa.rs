use crate::parse::*;
use nom::{take, named, map_res, take_until, opt};
use crate::errors::NmeaSentenceError;
use super::utils::*;

fn build_gsa<'a>(sentence: (Option<char>, Option<u8>, Option<u8>, Option<u8>, Option<u8>, Option<u8>, Option<u8>, Option<u8>, Option<u8>, Option<u8>, Option<u8>, Option<u8>, Option<u8>, Option<u8>, Option<f32>, Option<f32>, Option<f32>)) -> Result<GsaData, NmeaSentenceError<'a>> {
    Ok(GsaData{
        selection_mode: translate_option!(sentence.0, GsaSelectionMode),
        mode: translate_option!(sentence.1, GsaMode),
        satellites: [sentence.2, sentence.3, sentence.4, sentence.5, sentence.6, sentence.7, sentence.8, sentence.9, sentence.10, sentence.11, sentence.12, sentence.13],
        pdob: sentence.14,
        hdop: sentence.15,
        vdop: sentence.16,
    })
}

named!(parse_sat_id<Option<u8>>,
    map_res!(
        do_parse!(
            id: opt!(map_res!(take_until!(","), parse_num::<u8>)) >>
            char!(',') >>
            (id)
        ),
        |sentence: (Option<u8>)| -> Result<Option<u8>, NmeaSentenceError> {
            Ok(sentence)
        }
    )
);

named!(pub (crate) parse_gsa<GsaData>,
    map_res!(
        do_parse!(
            selection_mode: opt!(one_of!("MA")) >>
            char!(',') >>
            mode: opt!(map_res!(take!(1), parse_num::<u8>)) >>
            char!(',') >>
            sat1: complete!(parse_sat_id) >>
            sat2: complete!(parse_sat_id) >>
            sat3: complete!(parse_sat_id) >>
            sat4: complete!(parse_sat_id) >>
            sat5: complete!(parse_sat_id) >>
            sat6: complete!(parse_sat_id) >>
            sat7: complete!(parse_sat_id) >>
            sat8: complete!(parse_sat_id) >>
            sat9: complete!(parse_sat_id) >>
            sat10: complete!(parse_sat_id) >>
            sat11: complete!(parse_sat_id) >>
            sat12: complete!(parse_sat_id) >> 
            pdob: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            hdop: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            vdop: opt!(map_res!(take_until!("*"), parse_num::<f32>)) >>
            char!('*') >>
            (selection_mode, mode, sat1, sat2, sat3, sat4, sat5, sat6, sat7, sat8, sat9, sat10, sat11, sat12, pdob, hdop, vdop)
        ),
        build_gsa
    )
);