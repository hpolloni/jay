extern crate jay;

use std::fs::File;
use std::io::Read;
use jay::class_file::ClassFile;

fn load_test_class(classname: &str) -> Vec<u8> {
    let fullpath = format!("tests/resources/{}.class", classname);
    let mut file = File::open(fullpath).unwrap();
    let mut buf: Vec<u8> = vec!();

    file.read_to_end(&mut buf).unwrap();

    buf
}

#[test]
fn can_parse_empty_class() {
    let mut buff = &load_test_class("Empty")[..];
    let class_file = ClassFile::parse(&mut buff).unwrap();

    dbg!(&class_file);

    assert_eq!("Empty", class_file.name());
    assert_eq!("java/lang/Object", class_file.super_name());
}

#[test]
fn can_parse_class_with_fields() {
    let mut buff = &load_test_class("HaveFields")[..];
    let class_file = ClassFile::parse(&mut buff).unwrap();

    dbg!(&class_file);

    assert_eq!("HaveFields", class_file.name());
    assert_eq!("java/lang/Object", class_file.super_name());

}
