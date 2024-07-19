use super::NbtValue;
use std::collections::HashMap;

pub trait NbtWriterTrait {
    fn write_byte(&mut self, value: i8);
    fn write_short(&mut self, value: i16);
    fn write_int(&mut self, value: i32);
    fn write_long(&mut self, value: i64);
    fn write_float(&mut self, value: f32);
    fn write_double(&mut self, value: f64);
    fn write_byte_array(&mut self, value: Vec<i8>);
    fn write_string(&mut self, value: String);
    fn write_list(&mut self, value: Vec<NbtValue>);
    fn write_compound(&mut self, value: HashMap<String, NbtValue>);
    fn write_int_array(&mut self, value: Vec<i32>);
    fn write_long_array(&mut self, value: Vec<i64>);
}

pub struct NbtWriter {
    data: Vec<u8>,
}

impl NbtWriter {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    pub fn get_data(self) -> Vec<u8> {
        self.data
    }
}

impl NbtWriterTrait for NbtWriter {
    fn write_byte(&mut self, value: i8) {
        self.data.push(value as u8);
    }

    fn write_short(&mut self, value: i16) {
        self.data.push((value >> 8) as u8);
        self.data.push(value as u8);
    }

    fn write_int(&mut self, value: i32) {
        self.data.push((value >> 24) as u8);
        self.data.push((value >> 16) as u8);
        self.data.push((value >> 8) as u8);
        self.data.push(value as u8);
    }

    fn write_long(&mut self, value: i64) {
        self.data.push((value >> 56) as u8);
        self.data.push((value >> 48) as u8);
        self.data.push((value >> 40) as u8);
        self.data.push((value >> 32) as u8);
        self.data.push((value >> 24) as u8);
        self.data.push((value >> 16) as u8);
        self.data.push((value >> 8) as u8);
        self.data.push(value as u8);
    }

    fn write_float(&mut self, value: f32) {
        self.write_int(value.to_bits() as i32);
    }

    fn write_double(&mut self, value: f64) {
        self.write_long(value.to_bits() as i64);
    }

    fn write_byte_array(&mut self, value: Vec<i8>) {
        self.write_int(value.len() as i32);
        for byte in value {
            self.write_byte(byte);
        }
    }

    fn write_string(&mut self, value: String) {
        let bytes = value.as_bytes();
        self.write_short(bytes.len() as i16);
        for byte in bytes {
            self.write_byte(*byte as i8);
        }
    }

    fn write_list(&mut self, value: Vec<NbtValue>) {
        let tag = match value.first() {
            Some(tag) => tag,
            None => panic!("Expected at least one element in list"),
        };
        self.write_byte(match tag {
            NbtValue::Byte(_) => 1,
            NbtValue::Short(_) => 2,
            NbtValue::Int(_) => 3,
            NbtValue::Long(_) => 4,
            NbtValue::Float(_) => 5,
            NbtValue::Double(_) => 6,
            NbtValue::ByteArray(_) => 7,
            NbtValue::String(_) => 8,
            NbtValue::List(_) => 9,
            NbtValue::Compound(_) => 10,
            NbtValue::IntArray(_) => 11,
            NbtValue::LongArray(_) => 12,
        });
        self.write_int(value.len() as i32);
        for element in value {
            match element {
                NbtValue::Byte(v) => self.write_byte(v),
                NbtValue::Short(v) => self.write_short(v),
                NbtValue::Int(v) => self.write_int(v),
                NbtValue::Long(v) => self.write_long(v),
                NbtValue::Float(v) => self.write_float(v),
                NbtValue::Double(v) => self.write_double(v),
                NbtValue::ByteArray(v) => self.write_byte_array(v),
                NbtValue::String(v) => self.write_string(v),
                NbtValue::List(v) => self.write_list(v),
                NbtValue::Compound(v) => self.write_compound(v),
                NbtValue::IntArray(v) => self.write_int_array(v),
                NbtValue::LongArray(v) => self.write_long_array(v),
            }
        }
    }

    fn write_compound(&mut self, value: HashMap<String, NbtValue>) {
        for (key, value) in value {
            self.write_byte(8);
            self.write_string(key);
            match value {
                NbtValue::Byte(v) => {self.write_byte(v);},
                NbtValue::Short(v) => {self.write_short(v);},
                NbtValue::Int(v) => {self.write_int(v);},
                NbtValue::Long(v) => {self.write_long(v);},
                NbtValue::Float(v) => {self.write_float(v);},
                NbtValue::Double(v) => {self.write_double(v);},
                NbtValue::ByteArray(v) => {self.write_byte_array(v);},
                NbtValue::String(v) => {self.write_string(v);},
                NbtValue::List(v) => {self.write_list(v);},
                NbtValue::Compound(v) => {self.write_compound(v);},
                NbtValue::IntArray(v) => {self.write_int_array(v);},
                NbtValue::LongArray(v) => {self.write_long_array(v);},
            }
        }
        self.write_byte(0);
    }

    fn write_int_array(&mut self, value: Vec<i32>) {
        self.write_int(value.len() as i32);
        for int in value {
            self.write_int(int);
        }
    }

    fn write_long_array(&mut self, value: Vec<i64>) {
        self.write_int(value.len() as i32);
        for long in value {
            self.write_long(long);
        }
    }
}