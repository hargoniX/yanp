use yanp::parse_nmea_sentence;


fn main(){
    match parse_nmea_sentence(b"$GPGSV,4,1,13,02,02,213,,03,-3,000,,11,00,121,,14,13,172,05*67\r\n") {
        Ok(val) => println!("{:#?}", val),
        Err(e) => println!("{:#?}", e),
    };
}