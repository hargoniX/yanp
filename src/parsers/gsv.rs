use super::utils::*;
use crate::errors::NmeaSentenceError;
use crate::parse::*;
use nom::alt;

named!(parse_gsv_sat<GsvSatellite>,
    map_res!(
        do_parse!(
            sat_id: opt!(map_res!(take_until!(","), parse_num::<u8>)) >>
            char!(',') >>
            elevation: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            true_azimuth: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            snr: opt!(alt!(map_res!(take_until!(","), parse_num::<u8>) | map_res!(take_until!(","), parse_num::<u8>))) >>
            one_of!(",*") >>
            (sat_id, elevation, true_azimuth, snr)
        ),
        |sentence: (Option<u8>, Option<f32>, Option<f32>, Option<u8>)| -> Result<GsvSatellite, NmeaSentenceError> {
            Ok(GsvSatellite {
                sat_id: sentence.0,
                elevation: sentence.1,
                true_azimuth: sentence.2,
                snr: sentence.3,
            })
        }
    )
);

fn build_gsv<'a>(
    sentence: (
        Option<u16>,
        Option<u16>,
        Option<u8>,
        Option<GsvSatellite>,
        Option<GsvSatellite>,
        Option<GsvSatellite>,
        Option<GsvSatellite>,
    ),
) -> Result<GsvData, NmeaSentenceError<'a>> {
    Ok(GsvData {
        number_of_sentences: sentence.0,
        sentence_num: sentence.1,
        sats_in_view: sentence.2,
        sats_info: [sentence.3, sentence.4, sentence.5, sentence.6],
    })
}

named!(pub (crate) parse_gsv<GsvData>,
    map_res!(
        do_parse!(
            number_of_sentences: opt!(map_res!(take_until!(","), parse_num::<u16>)) >>
            char!(',') >>
            sentence_num: opt!(map_res!(take_until!(","), parse_num::<u16>)) >>
            char!(',') >>
            sats_in_view: opt!(map_res!(take_until!(","), parse_num::<u8>)) >>
            char!(',') >>
            sat1_info: opt!(complete!(parse_gsv_sat)) >>
            sat2_info: opt!(complete!(parse_gsv_sat)) >>
            sat3_info: opt!(complete!(parse_gsv_sat)) >>
            sat4_info: opt!(complete!(parse_gsv_sat)) >>
            (number_of_sentences, sentence_num, sats_in_view, sat1_info, sat2_info, sat3_info, sat4_info)
        ),
        build_gsv
    )
);
