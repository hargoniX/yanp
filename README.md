# YANP - Yet Another NMEA Parser
A no_std Rust NMEA 0183 sentence parser.

## Currently supported Sentences:
* BOD
* BWC
* GBS
* GGA
* GLL
* GSA
* GNS
* GSV
* HDT
* RMA
* RMB
* RMC
* STN
* VBW
* VTG
* WPL

## Usage
Put this in your Cargo.toml:
```toml
#[dependencies]
yanp = "0.0.1"
```
And in your code:
```rs
use yanp::parse_nmea_sentence;

fn main(){
    match parse_nmea_sentence(b"$GPGLL,4916.45,N,12311.12,W,225444,A,*1D\r\n") {
        Ok(val) => println!("{:#?}", val),
        Err(e) => println!("{:#?}", e),
    };
}
```
It is very important that the \r\n is included in the sentence as the library depends on this for a few slice operations as of now.

As of now the GNS sentence requires the alloc feature to be selected.