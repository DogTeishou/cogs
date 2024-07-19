use std::collections::HashMap;
use super::NbtValue;

pub trait NbtReaderTrait {
    fn read_byte(&mut self) -> NbtValue;
    fn read_short(&mut self) -> NbtValue;
    fn read_int(&mut self) -> NbtValue;
    fn read_long(&mut self) -> NbtValue;
    fn read_float(&mut self) -> NbtValue;
    fn read_double(&mut self) -> NbtValue;
    fn read_byte_array(&mut self) -> NbtValue;
    fn read_string(&mut self) -> NbtValue;
    fn read_list(&mut self) -> NbtValue;
    fn read_compound(&mut self) -> NbtValue;
    fn read_int_array(&mut self) -> NbtValue;
    fn read_long_array(&mut self) -> NbtValue;
}

pub struct NbtReader {
    data: Vec<u8>,
    cursor: usize,
}

impl NbtReader {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            cursor: 0,
        }
    }
}

impl NbtReaderTrait for NbtReader {
    fn read_byte(&mut self) -> NbtValue {
        let value = self.data[self.cursor] as i8;
        self.cursor += 1;
        NbtValue::Byte(value)
    }

    fn read_short(&mut self) -> NbtValue {
        let big = self.data[self.cursor] as i16;
        let little = self.data[self.cursor + 1] as i16;
        let value = (big << 8) | little;
        self.cursor += 2;
        NbtValue::Short(value)
    }

    fn read_int(&mut self) -> NbtValue {
        let byte1 = self.data[self.cursor] as i32;
        let byte2 = self.data[self.cursor + 1] as i32;
        let byte3 = self.data[self.cursor + 2] as i32;
        let byte4 = self.data[self.cursor + 3] as i32;
        let value = (byte1 << 24) | (byte2 << 16) | (byte3 << 8) | byte4;
        self.cursor += 4;
        NbtValue::Int(value)
    }

    fn read_long(&mut self) -> NbtValue {
        let byte1 = self.data[self.cursor] as i64;
        let byte2 = self.data[self.cursor + 1] as i64;
        let byte3 = self.data[self.cursor + 2] as i64;
        let byte4 = self.data[self.cursor + 3] as i64;
        let byte5 = self.data[self.cursor + 4] as i64;
        let byte6 = self.data[self.cursor + 5] as i64;
        let byte7 = self.data[self.cursor + 6] as i64;
        let byte8 = self.data[self.cursor + 7] as i64;
        let value = (byte1 << 56) | (byte2 << 48) | (byte3 << 40) | (byte4 << 32) | (byte5 << 24) | (byte6 << 16) | (byte7 << 8) | byte8;
        self.cursor += 8;
        NbtValue::Long(value)
    }

    fn read_float(&mut self) -> NbtValue{
        let value = self.read_int();
        let value = match value {
            NbtValue::Int(v) => v,
            _ => panic!("Expected NbtValue::Int"),
        };
        self.cursor += 4;
        NbtValue::Float(value as f32)
    }

    fn read_double(&mut self) -> NbtValue {
        let value = self.read_long();
        let value = match value {
            NbtValue::Long(v) => v,
            _ => panic!("Expected NbtValue::Long"),
        };
        self.cursor += 8;
        NbtValue::Double(value as f64)
    }

    fn read_byte_array(&mut self) -> NbtValue {
        let len = self.read_int();
        let len = match len {
            NbtValue::Int(v) => v,
            _ => panic!("Expected NbtValue::Int"),
        };
        let mut value = Vec::with_capacity(len as usize);
        for _ in 0..len {
            value.push(self.data[self.cursor] as i8);
            self.cursor += 1;
        }
        NbtValue::ByteArray(value)
    }

    fn read_string(&mut self) -> NbtValue {
        let len = self.read_short();
        let len = match len {
            NbtValue::Short(v) => v,
            _ => panic!("Expected NbtValue::Short"),
        };
        let mut value = String::with_capacity(len as usize);
        for _ in 0..len {
            value.push(self.data[self.cursor] as char);
            self.cursor += 1;
        }
        NbtValue::String(value)
    }

    fn read_list(&mut self) -> NbtValue {
        let tag = self.read_byte();
        let tag = match tag {
            NbtValue::Byte(v) => v,
            _ => panic!("Expected NbtValue::Byte"),  
        };
        let len = self.read_int();
        let len = match len {
            NbtValue::Int(v) => v,
            _ => panic!("Expected NbtValue::Int"),
        };
        let mut value = Vec::with_capacity(len as usize);
        for _ in 0..len {
            match tag {
                1 => {value.push(self.read_byte());},
                2 => {value.push(self.read_short());},
                3 => {value.push(self.read_int());},
                4 => {value.push(self.read_long());},
                5 => {value.push(self.read_float());},
                6 => {value.push(self.read_double());},
                7 => {value.push(self.read_byte_array());},
                8 => {value.push(self.read_string());},
                9 => {value.push(self.read_list());},
                10 => {value.push(self.read_compound());},
                11 => {value.push(self.read_int_array());},
                12 => {value.push(self.read_long_array());},
                _ => {panic!("Invalid tag");},
            };
        }
        NbtValue::List(value)
    }

    fn read_compound(&mut self) -> NbtValue {
        let mut value = HashMap::new();
        loop {
            let tag = self.read_byte();
            let tag = match tag {
                NbtValue::Byte(v) => v,
                _ => panic!("Expected NbtValue::Byte"),
            };
            if tag == 0 {
                break;
            }
            let name = self.read_string();
            let name = match name {
                NbtValue::String(v) => v,
                _ => panic!("Expected NbtValue::String"),
            };
            match tag {
                1 => {value.insert(name, self.read_byte());},
                2 => {value.insert(name, self.read_short());},
                3 => {value.insert(name, self.read_int());},
                4 => {value.insert(name, self.read_long());},
                5 => {value.insert(name, self.read_float());},
                6 => {value.insert(name, self.read_double());},
                7 => {value.insert(name, self.read_byte_array());},
                8 => {value.insert(name, self.read_string());},
                9 => {value.insert(name, self.read_list());},
                10 => {value.insert(name, self.read_compound());},
                11 => {value.insert(name, self.read_int_array());},
                12 => {value.insert(name, self.read_long_array());},
                _ => {panic!("Invalid tag");},
            };
        }
        NbtValue::Compound(value)
    }

    fn read_int_array(&mut self) -> NbtValue {
        let len = self.read_int();
        let len = match len {
            NbtValue::Int(v) => v,
            _ => panic!("Expected NbtValue::Int"),
        };
        let mut value = Vec::with_capacity(len as usize);
        for _ in 0..len {
            let int = self.read_int();
            let int = match int {
                NbtValue::Int(v) => v,
                _ => panic!("Expected NbtValue::Int"),
            };
            value.push(int);
        }
        NbtValue::IntArray(value)
    }

    fn read_long_array(&mut self) -> NbtValue {
        let len = self.read_int();
        let len = match len {
            NbtValue::Int(v) => v,
            _ => panic!("Expected NbtValue::Int"),
        };
        let mut value = Vec::with_capacity(len as usize);
        for _ in 0..len {
            let long = self.read_long();
            let long = match long {
                NbtValue::Long(v) => v,
                _ => panic!("Expected NbtValue::Long"),
            };
            value.push(long);
        }
        NbtValue::LongArray(value)
    }
}