use crate::Nbt;
use std::fs;

#[test]
fn test_nbt() {
    use crate::reader::NbtReader;

    let data = fs::read("src/test/hello_world.nbt").unwrap();
    println!("{:?} {:?}", data, data.len());
    let mut reader = NbtReader::new(&data);
    let values = match reader.read_compound(&mut Nbt::new()) {
        Ok(v) => v,
        Err(e) => panic!("Failed to read NBT: {}", e),
    };

    println!("{:?}", values);
}