use yanp::parse_nmea_sentence;


fn main(){
    match parse_nmea_sentence(b"$GPGLL,4916.45,N,12311.12,W,225444,A,*1D\r\n") {
        Ok(val) => println!("{:#?}", val),
        Err(e) => println!("{:#?}", e),
    };
}