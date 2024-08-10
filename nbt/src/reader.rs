use crate::{Error, Nbt, NbtAfter764, NbtReadTrait, Result, Value};

pub struct NbtReader<'a> {
    pub data: &'a mut [u8],
    pub cursor: usize,
}

impl NbtReader<'_> {
    pub fn new(data: &mut [u8]) -> NbtReader {
        NbtReader {
            data,
            cursor: 0,
        }
    }

    #[inline]
    pub fn roll_back(&mut self, n: usize) {
        self.cursor -= n;
    }

    #[inline]
    pub fn roll_down(&mut self, n: usize) {
        self.cursor += n;
    }

    #[inline]
    pub fn read_u8 (&mut self) -> u8 {
        let value = self.data[self.cursor];
        self.cursor += 1;
        value
    }

    #[inline]
    pub fn read_i8 (&mut self) -> i8 {
        self.read_u8() as i8
    }

    #[inline]
    pub fn read_i16(&mut self) -> i16 {
        let value = i16::from_be_bytes([self.data[self.cursor], self.data[self.cursor + 1]]);
        self.cursor += 2;
        value
    }

    #[inline]
    pub fn read_i32(&mut self) -> i32 {
        let value = i32::from_be_bytes([
            self.data[self.cursor],
            self.data[self.cursor + 1],
            self.data[self.cursor + 2],
            self.data[self.cursor + 3],
        ]);
        self.cursor += 4;
        value
    }

    #[inline]
    pub fn read_i64(&mut self) -> i64 {
        let value = i64::from_be_bytes([
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
        value
    }

    #[inline]
    pub fn read_f32(&mut self) -> f32 {
        let value = f32::from_be_bytes([
            self.data[self.cursor],
            self.data[self.cursor + 1],
            self.data[self.cursor + 2],
            self.data[self.cursor + 3],
        ]);
        self.cursor += 4;
        value
    }

    #[inline]
    pub fn read_f64(&mut self) -> f64 {
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
        value
    }

    #[inline]
    pub fn read_i8_array(&mut self, len: usize) -> Vec<i8> {
        let mut value = Vec::with_capacity(len);
        for _ in 0..len {
            value.push(self.read_i8());
        }
        value
    }

    #[inline]
    pub fn read_i32_array(&mut self, len: usize) -> Vec<i32> {
        let mut value = Vec::with_capacity(len);
        for _ in 0..len {
            value.push(self.read_i32());
        }
        value
    }

    #[inline]
    pub fn read_i64_array(&mut self, len: usize) -> Vec<i64> {
        let mut value = Vec::with_capacity(len);
        for _ in 0..len {
            value.push(self.read_i64());
        }
        value
    }

    #[inline]
    pub fn read_string(&mut self, len: usize) -> Result<String> {
        let value = match String::from_utf8(self.data[self.cursor..self.cursor + len].to_vec()) {
            Ok(v) => v,
            Err(e) => return Err(Error::Utf8(e)),
        };
        self.cursor += len;
        Ok(value)
    }
}

impl NbtReadTrait for Nbt {
    #[inline]
    fn read_byte_array(r: &mut self::NbtReader) -> Result<Vec<i8>> {
        let len = r.read_i32() as usize;
        Ok(r.read_i8_array(len))
    }

    #[inline]
    fn read_nbt_string(r: &mut self::NbtReader) -> Result<String> {
        let len = r.read_i16() as usize;
        r.read_string(len)
    }

    #[inline]
    fn read_int_array(r: &mut self::NbtReader) -> Result<Vec<i32>> {
        let len = r.read_i32() as usize;
        Ok(r.read_i32_array(len))
    }

    #[inline]
    fn read_long_array(r: &mut self::NbtReader) -> Result<Vec<i64>> {
        let len = r.read_i32() as usize;
        Ok(r.read_i64_array(len))
    }

    #[inline]
    fn read_list(r: &mut self::NbtReader) -> Result<Vec<Value>> {
        let type_id = r.read_u8();
        let len = r.read_i32() as usize;
        let mut list = Vec::with_capacity(len);
        for _ in 0..len {
            let value = match type_id {
                1 => Value::Byte(r.read_i8()),
                2 => Value::Short(r.read_i16()),
                3 => Value::Int(r.read_i32()),
                4 => Value::Long(r.read_i64()),
                5 => Value::Float(r.read_f32()),
                6 => Value::Double(r.read_f64()),
                7 => Value::ByteArray(Nbt::read_byte_array(r)?),
                8 => Value::String(Nbt::read_nbt_string(r)?),
                9 => Value::List(Nbt::read_list(r)?),
                10 => Value::Compound(None, Nbt::read_compound(r)?),
                11 => Value::IntArray(Nbt::read_int_array(r)?),
                12 => Value::LongArray(Nbt::read_long_array(r)?),
                _ => return Err(Error::InvalidTag(type_id)),
            };
            list.push(value);
        }
        Ok(list)
    }

    #[inline]
    fn read_compound(r: &mut self::NbtReader) -> Result<Vec<(String, Value)>> {
        let mut compound = Vec::new();
        loop {
            let type_id = r.read_u8();
            if type_id == 0 {
                break;
            }
            let name = Nbt::read_nbt_string(r)?;
            let value = match type_id {
                1 => Value::Byte(r.read_i8()),
                2 => Value::Short(r.read_i16()),
                3 => Value::Int(r.read_i32()),
                4 => Value::Long(r.read_i64()),
                5 => Value::Float(r.read_f32()),
                6 => Value::Double(r.read_f64()),
                7 => Value::ByteArray(Nbt::read_byte_array(r)?),
                8 => Value::String(Nbt::read_nbt_string(r)?),
                9 => Value::List(Nbt::read_list(r)?),
                10 => Value::Compound(Some(name.clone()), Nbt::read_compound(r)?),
                11 => Value::IntArray(Nbt::read_int_array(r)?),
                12 => Value::LongArray(Nbt::read_long_array(r)?),
                _ => return Err(Error::InvalidTag(type_id)),
            };
            compound.push((name, value));
        }
        Ok(compound)
    }

    fn from_reader(r: &mut self::NbtReader) -> Result<Value> {
        match r.read_u8() {
            10 => {
                let name = Nbt::read_nbt_string(r)?;
                Ok(Value::Compound(Some(name), Nbt::read_compound(r)?))
            }
            x => Err(Error::RootTagNotCompound(x)),
        }
    }
}

impl NbtReadTrait for NbtAfter764 {
    #[inline]
    fn read_byte_array(r: &mut self::NbtReader) -> Result<Vec<i8>> {
        Nbt::read_byte_array(r)
    }

    #[inline]
    fn read_nbt_string(r: &mut self::NbtReader) -> Result<String> {
        Nbt::read_nbt_string(r)
    }

    #[inline]
    fn read_int_array(r: &mut self::NbtReader) -> Result<Vec<i32>> {
        Nbt::read_int_array(r)
    }

    #[inline]
    fn read_long_array(r: &mut self::NbtReader) -> Result<Vec<i64>> {
        Nbt::read_long_array(r)
    }

    #[inline]
    fn read_list(r: &mut self::NbtReader) -> Result<Vec<Value>> {
        Nbt::read_list(r)
    }

    #[inline]
    fn read_compound(r: &mut self::NbtReader) -> Result<Vec<(String, Value)>> {
        Nbt::read_compound(r)
    }

    fn from_reader(r: &mut self::NbtReader) -> Result<Value> {
        match r.read_u8() {
            10 => {
                Ok(Value::Compound(None, Nbt::read_compound(r)?))
            }
            x => Err(Error::RootTagNotCompound(x)),
        }
    }
}