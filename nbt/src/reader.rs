use crate::{Error, Nbt, NbtAfter764, NbtReadTrait, Result, Value};

pub struct NbtReader<'a> {
    pub data: &'a [u8],
    pub cursor: usize,
}

impl<'a> NbtReader<'a> {
    pub fn new(data: &'a[u8]) -> Self {
        Self {
            data,
            cursor: 0,
        }
    }

    #[inline]
    pub fn read_byte(&mut self) -> Result<i8> {
        if self.cursor >= self.data.len() {
            return Err(Error::Io(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, format!("Unexpected EOF at {}", self.cursor))));
        }
        let value = self.data[self.cursor] as i8;
        self.cursor += 1;
        Ok(value)
    }

    #[inline]
    pub fn read_short(&mut self) -> Result<i16> {
        if self.cursor >= self.data.len() {
            return Err(Error::Io(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, format!("Unexpected EOF at {}", self.cursor))));
        }
        let value = ((self.data[self.cursor] as i16) << 8) | (self.data[self.cursor + 1] as i16);
        self.cursor += 2;
        Ok(value)
    }

    #[inline]
    pub fn read_int(&mut self) -> Result<i32> {
        if self.cursor >= self.data.len() {
            return Err(Error::Io(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, format!("Unexpected EOF at {}", self.cursor))));
        }
        let value = ((self.data[self.cursor] as i32) << 24)
            | ((self.data[self.cursor + 1] as i32) << 16)
            | ((self.data[self.cursor + 2] as i32) << 8)
            | (self.data[self.cursor + 3] as i32);
        self.cursor += 4;
        Ok(value)
    }

    #[inline]
    pub fn read_long(&mut self) -> Result<i64> {
        if self.cursor >= self.data.len() {
            return Err(Error::Io(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, format!("Unexpected EOF at {}", self.cursor))));
        }
        let value = ((self.data[self.cursor] as i64) << 56)
            | ((self.data[self.cursor + 1] as i64) << 48)
            | ((self.data[self.cursor + 2] as i64) << 40)
            | ((self.data[self.cursor + 3] as i64) << 32)
            | ((self.data[self.cursor + 4] as i64) << 24)
            | ((self.data[self.cursor + 5] as i64) << 16)
            | ((self.data[self.cursor + 6] as i64) << 8)
            | (self.data[self.cursor + 7] as i64);
        self.cursor += 8;
        Ok(value)
    }

    #[inline]
    pub fn read_float(&mut self) -> Result<f32> {
        if self.cursor >= self.data.len() {
            return Err(Error::Io(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, format!("Unexpected EOF at {}", self.cursor))));
        }
        let value = f32::from_be_bytes([
            self.data[self.cursor],
            self.data[self.cursor + 1],
            self.data[self.cursor + 2],
            self.data[self.cursor + 3],
        ]);
        self.cursor += 4;
        Ok(value)
    }

    #[inline]
    pub fn read_double(&mut self) -> Result<f64> {
        if self.cursor >= self.data.len() {
            return Err(Error::Io(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, format!("Unexpected EOF at {}", self.cursor))));
        }
        let value = f64::from_be_bytes([
            self.data[self.cursor],
            self.data[self.cursor + 1],
            self.data[self.cursor + 2],
            self.data[self.cursor + 3],
            self.data[self.cursor + 4],
            self.data[self.cursor + 5],
            self.data[self.cursor + 6],
            self.data[self.cursor + 7],
        ]);
        self.cursor += 8;
        Ok(value)
    }

    #[inline]
    pub fn read_byte_array(&mut self) -> Result<Vec<i8>> {
        let len = match self.read_int() {
            Ok(v) => v,
            Err(e) => return Err(e),
        } as usize;
        let mut value = Vec::with_capacity(len);
        for _ in 0..len {
            value.push(self.read_byte()?);
        }
        Ok(value)
    }

    #[inline]
    pub fn read_string(&mut self) -> Result<String> {
        let len = match self.read_short() {
            Ok(v) => v,
            Err(e) => return Err(e),
        } as usize;
        let value = match String::from_utf8(self.data[self.cursor..self.cursor + len].to_vec()) {
            Ok(v) => v,
            Err(e) => return Err(Error::Utf8(e)),
        };
        self.cursor += len;
        Ok(value)
    }

    #[inline]
    pub fn read_int_array(&mut self) -> Result<Vec<i32>> {
        let len = match self.read_int() {
            Ok(v) => v,
            Err(e) => return Err(e),
        } as usize;
        let mut value = Vec::with_capacity(len);
        for _ in 0..len {
            value.push(self.read_int()?);
        }
        Ok(value)
    }

    #[inline]
    pub fn read_long_array(&mut self) -> Result<Vec<i64>> {
        let len = match self.read_int() {
            Ok(v) => v,
            Err(e) => return Err(e),
        } as usize;
        let mut value = Vec::with_capacity(len);
        for _ in 0..len {
            value.push(self.read_long()?);
        }
        Ok(value)
    }

    #[inline]
    pub fn read_list<R: NbtReadTrait> (&mut self, r: &mut R) -> Result<Vec<Value>> {
        r.read_list(self)
    }

    
    pub fn read_compound<R: NbtReadTrait> (&mut self, r: &mut R) -> Result<Vec<(String, Value)> > {
        r.read_compound(self)
    }
}

impl NbtReadTrait for Nbt {
    fn read_list(&mut self, r: &mut NbtReader) -> Result<Vec<Value>> {
        let tag = match r.read_byte() {
            Ok(v) => v,
            Err(e) => return Err(e),
        } as u8;
        let len = match r.read_int() {
            Ok(v) => v,
            Err(e) => return Err(e),
        } as usize;
        let mut values = Vec::with_capacity(len as usize);
        for _ in 0..len {
            let value = match tag {
                1 => Value::Byte(match r.read_byte() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                2 => Value::Short(match r.read_short() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                3 => Value::Int(match r.read_int() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                4 => Value::Long(match r.read_long() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                5 => Value::Float(match r.read_float() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                6 => Value::Double(match r.read_double() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                7 => Value::ByteArray(match r.read_byte_array() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                8 => Value::String(match r.read_string() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                9 => Value::List(match self.read_list(r) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                10 => Value::Compound(match self.read_compound(r) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                11 => Value::IntArray(match r.read_int_array() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                12 => Value::LongArray(match r.read_long_array() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                _ => return Err(Error::InvalidTag(tag)),
            };
            values.push(value);
        };
        Ok(values)
    }

    fn read_compound(&mut self, r: &mut NbtReader) -> Result<Vec<(String, Value)>> {
        let mut values = Vec::new();
        let mut vals = Vec::new();
        let byte = match r.read_byte() {
            Ok(v) => v,
            Err(e) => return Err(e),
        } as u8;
        if byte != 10 {
            return Err(Error::InvalidTag(byte));
        }
        let namefirst = match r.read_string() {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        loop {
            let tag = match r.read_byte() {
                Ok(v) => v,
                Err(e) => return Err(e),
            } as u8;
            if tag == 0 {
                break;
            }
            let name = match r.read_string() {
                Ok(v) => v,
                Err(e) => return Err(e),
            };
            let value = match tag {
                1 => Value::Byte(match r.read_byte() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                2 => Value::Short(match r.read_short() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                3 => Value::Int(match r.read_int() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                4 => Value::Long(match r.read_long() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                5 => Value::Float(match r.read_float() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                6 => Value::Double(match r.read_double() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                7 => Value::ByteArray(match r.read_byte_array() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                8 => Value::String(match r.read_string() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                9 => Value::List(match self.read_list(r) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                10 => Value::Compound(match self.read_compound(r) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                11 => Value::IntArray(match r.read_int_array() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                12 => Value::LongArray(match r.read_long_array() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                _ => return Err(Error::InvalidTag(tag)),
            };
            vals.push((name, value));
        };
        values.push((namefirst, Value::Compound(vals)));
        Ok(values)
    }
}

impl NbtReadTrait for NbtAfter764 {
    fn read_list(&mut self, r: &mut NbtReader) -> Result<Vec<Value>> {
        let tag = match r.read_byte() {
            Ok(v) => v,
            Err(e) => return Err(e),
        } as u8;
        let len = match r.read_int() {
            Ok(v) => v,
            Err(e) => return Err(e),
        } as usize;
        let mut values = Vec::with_capacity(len as usize);
        for _ in 0..len {
            let value = match tag {
                1 => Value::Byte(match r.read_byte() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                2 => Value::Short(match r.read_short() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                3 => Value::Int(match r.read_int() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                4 => Value::Long(match r.read_long() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                5 => Value::Float(match r.read_float() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                6 => Value::Double(match r.read_double() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                7 => Value::ByteArray(match r.read_byte_array() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                8 => Value::String(match r.read_string() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                9 => Value::List(match self.read_list(r) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                10 => Value::Compound(match self.read_compound(r) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                11 => Value::IntArray(match r.read_int_array() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                12 => Value::LongArray(match r.read_long_array() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                _ => return Err(Error::InvalidTag(tag)),
            };
            values.push(value);
        };
        Ok(values)
    }

    fn read_compound(&mut self, r: &mut NbtReader) -> Result<Vec<(String, Value)>> {
        let mut values = Vec::new();
        let mut vals = Vec::new();
        let byte = match r.read_byte() {
            Ok(v) => v,
            Err(e) => return Err(e),
        } as u8;
        if byte != 10 {
            return Err(Error::InvalidTag(byte));
        }
        loop {
            let tag = match r.read_byte() {
                Ok(v) => v,
                Err(e) => return Err(e),
            } as u8;
            if tag == 0 {
                break;
            }
            let value = match tag {
                1 => Value::Byte(match r.read_byte() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                2 => Value::Short(match r.read_short() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                3 => Value::Int(match r.read_int() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                4 => Value::Long(match r.read_long() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                5 => Value::Float(match r.read_float() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                6 => Value::Double(match r.read_double() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                7 => Value::ByteArray(match r.read_byte_array() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                8 => Value::String(match r.read_string() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                9 => Value::List(match self.read_list(r) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                10 => Value::Compound(match self.read_compound(r) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                11 => Value::IntArray(match r.read_int_array() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                12 => Value::LongArray(match r.read_long_array() {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                }),
                _ => return Err(Error::InvalidTag(tag)),
            };
            vals.push(("".to_string(), value));
        };
        values.push(("".to_string(), Value::Compound(vals)));
        Ok(values)
    }
}
