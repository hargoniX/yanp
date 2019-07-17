use crate::parse::*;
use crate::errors::NmeaSentenceError;
use super::utils::*;

named!(pub (crate) parse_hdt<HdtData>,
    map_res!(
        do_parse!(
            heading_true: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            tag!(",T*") >>
            (heading_true)
        ),
        | sentence: (Option<f32>) | -> Result<HdtData, NmeaSentenceError> {
            Ok(HdtData {
                heading_true: sentence,
            })
        }
    )
);