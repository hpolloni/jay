extern crate jay;

use jay::class_parse::parse;
use std::fs::File;
use std::io::Read;

#[test]
fn can_parse_empty_class() {
    let mut file = File::open("example.data")?;
    let mut data = Vec::new();
    file.read_to_end(&mut data);

}