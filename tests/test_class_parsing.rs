extern crate jay;

use std::fs::File;
use jay::class_parse::parse;

#[test]
fn can_parse_empty_class() {
    let file = File::open("tests/resources/Empty.class").unwrap();

    let parse_result = parse(file).unwrap();

}