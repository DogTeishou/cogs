use flate2::read::GzDecoder;

use crate::{reader::NbtReader, writer::NbtWriter, Nbt, NbtReadTrait, NbtWriteTrait};
use std::{fs, io::Read};

#[test]
fn test_nbt() {
    let mut data = fs::read("src/test/hello_world.nbt").unwrap();
    println!("{:?}", data);
    let mut reader = NbtReader::new(&mut data);
    let value = Nbt::from_reader(&mut reader).unwrap();
    println!("{:?}", value);
    let mut writer = NbtWriter::new();
    Nbt::write_to(&mut writer, &value).unwrap();
    println!("{:?}", writer.data);
    assert_eq!(data, writer.data);
}

#[test]
fn test_nbt_with_gzip() {
    let data1 = fs::read("src/test/bigtest.nbt").unwrap();
    let mut d = GzDecoder::new(&data1[..]);
    let mut data = Vec::new();
    d.read_to_end(&mut data).unwrap();
    let mut reader = NbtReader::new(&mut data);
    let value = Nbt::from_reader(&mut reader).unwrap();
    println!("{:?}", value);
    let mut writer = NbtWriter::new();
    Nbt::write_to(&mut writer, &value).unwrap();
    println!("{:?}", writer.data);
    assert_eq!(data, writer.data);
}