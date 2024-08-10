use crate::{Nbt, NbtAfter764, NbtWriteTrait, Value, Result, Error};

pub struct NbtWriter {
    pub data: Vec<u8>,
}

impl NbtWriter {
    pub fn new() -> NbtWriter {
        NbtWriter {
            data: Vec::new(),
        }
    }

    #[inline]
    pub fn write_u8(&mut self, value: u8) {
        self.data.push(value);
    }

    #[inline]
    pub fn write_i8(&mut self, value: i8) {
        self.write_u8(value as u8);
    }

    #[inline]
    pub fn write_i16(&mut self, value: i16) {
        self.data.extend_from_slice(&value.to_be_bytes());
    }

    #[inline]
    pub fn write_i32(&mut self, value: i32) {
        self.data.extend_from_slice(&value.to_be_bytes());
    }

    #[inline]
    pub fn write_i64(&mut self, value: i64) {
        self.data.extend_from_slice(&value.to_be_bytes());
    }

    #[inline]
    pub fn write_f32(&mut self, value: f32) {
        self.data.extend_from_slice(&value.to_be_bytes());
    }

    #[inline]
    pub fn write_f64(&mut self, value: f64) {
        self.data.extend_from_slice(&value.to_be_bytes());
    }

    #[inline]
    pub fn write_string(&mut self, value: &str) {
        self.write_i16(value.len() as i16);
        self.data.extend_from_slice(value.as_bytes());
    }
}

impl NbtWriteTrait for Nbt {
    #[inline]
    fn write_byte_array(w: &mut NbtWriter, value: &[i8]) {
        w.write_i32(value.len() as i32);
        for &v in value {
            w.write_i8(v);
        }
    }

    #[inline]
    fn write_nbt_string(w: &mut NbtWriter, value: &str) {
        w.write_string(value);
    }

    #[inline]
    fn write_int_array(w: &mut NbtWriter, value: &[i32]) {
        w.write_i32(value.len() as i32);
        for &v in value {
            w.write_i32(v);
        }
    }

    #[inline]
    fn write_long_array(w: &mut NbtWriter, value: &[i64]) {
        w.write_i32(value.len() as i32);
        for &v in value {
            w.write_i64(v);
        }
    }

    #[inline]
    fn write_list(w: &mut NbtWriter, value: &[Value]) -> Result<()> {
        if value.is_empty() {
            w.write_i8(0);
            w.write_i32(0);
            return Ok(());
        }
        let tag = value.first().expect("empty list").tag();
        if !value.iter().all(|v| v.tag() == tag) {
            return Err(Error::ListTypeNotSame);
        }
        w.write_u8(tag);
        w.write_i32(value.len() as i32);
        for v in value {
            match v {
                Value::Byte(v) => w.write_i8(*v),
                Value::Short(v) => w.write_i16(*v),
                Value::Int(v) => w.write_i32(*v),
                Value::Long(v) => w.write_i64(*v),
                Value::Float(v) => w.write_f32(*v),
                Value::Double(v) => w.write_f64(*v),
                Value::ByteArray(v) => Self::write_byte_array(w, v),
                Value::String(v) => Self::write_nbt_string(w, v),
                Value::IntArray(v) => Self::write_int_array(w, v),
                Value::LongArray(v) => Self::write_long_array(w, v),
                Value::List(v) => Self::write_list(w, v)?,
                Value::Compound(_, v) => Self::write_compound(w, None, v)?,
            }
        }
        Ok(())
    }

    #[inline]
    fn write_compound(w: &mut NbtWriter, name: Option<&String>, value: &[(String, Value)]) -> Result<()> {
       if let Some(name) = name {
           Self::write_nbt_string(w, &name);
       }
       for (name, value) in value {
            w.write_u8(value.tag());
            if let Value::Compound(_, _) = value {
            } else {
                Self::write_nbt_string(w, name);
            }
            match value {
                Value::Byte(v) => w.write_i8(*v),
                Value::Short(v) => w.write_i16(*v),
                Value::Int(v) => w.write_i32(*v),
                Value::Long(v) => w.write_i64(*v),
                Value::Float(v) => w.write_f32(*v),
                Value::Double(v) => w.write_f64(*v),
                Value::ByteArray(v) => Self::write_byte_array(w, v),
                Value::String(v) => Self::write_nbt_string(w, v),
                Value::IntArray(v) => Self::write_int_array(w, v),
                Value::LongArray(v) => Self::write_long_array(w, v),
                Value::List(v) => Self::write_list(w, v)?,
                Value::Compound(name, v) => Self::write_compound(w, name.as_ref(), v)?,
            }
        }
        w.write_u8(0);
        Ok(())
    }

    fn write_to(w: &mut NbtWriter, value: &Value) -> Result<()> {
        match value {
            Value::Compound(name, data) => {
                w.write_u8(value.tag());
                Self::write_compound(w, name.as_ref(), data)?;
            }
            x => return Err(Error::RootTagNotCompound(x.tag())),
        }
        Ok(())
    }

    fn write_to_with_name(w: &mut self::NbtWriter, name: &str, value: &Value) -> Result<()> {
        w.write_u8(value.tag());
        Self::write_nbt_string(w, name);
        Self::write_to(w, value)
    }
}

impl NbtWriteTrait for NbtAfter764 {
    fn write_byte_array(w: &mut self::NbtWriter, value: &[i8]) {
        Nbt::write_byte_array(w, value);
    }

    fn write_nbt_string(w: &mut self::NbtWriter, value: &str) {
        Nbt::write_nbt_string(w, value);
    }

    fn write_int_array(w: &mut self::NbtWriter, value: &[i32]) {
        Nbt::write_int_array(w, value);
    }

    fn write_long_array(w: &mut self::NbtWriter, value: &[i64]) {
        Nbt::write_long_array(w, value);
    }

    fn write_list(w: &mut self::NbtWriter, value: &[Value]) -> Result<()> {
        Nbt::write_list(w, value)
    }

    fn write_compound(w: &mut self::NbtWriter, name: Option<&String>, value: &[(String, Value)]) -> Result<()> {
        Nbt::write_compound(w, name, value)
    }

    fn write_to(w: &mut self::NbtWriter, value: &Value) -> Result<()> {
        match value {
            Value::Compound(_, data) => {
                w.write_u8(value.tag());
                for (key, v) in data{
                    w.write_u8(v.tag());
                    Self::write_nbt_string(w, key);
                    match v {
                        Value::Byte(v) => w.write_i8(*v),
                        Value::Short(v) => w.write_i16(*v),
                        Value::Int(v) => w.write_i32(*v),
                        Value::Long(v) => w.write_i64(*v),
                        Value::Float(v) => w.write_f32(*v),
                        Value::Double(v) => w.write_f64(*v),
                        Value::ByteArray(v) => Self::write_byte_array(w, v),
                        Value::String(v) => Self::write_nbt_string(w, v),
                        Value::IntArray(v) => Self::write_int_array(w, v),
                        Value::LongArray(v) => Self::write_long_array(w, v),
                        Value::List(v) => Self::write_list(w, v)?,
                        Value::Compound(name, v) => Self::write_compound(w, name.as_ref(), v)?,
                    }
                }
                w.write_u8(0);
                Ok(())
            }
            x => return Err(Error::RootTagNotCompound(x.tag())),
        }
    }

    fn write_to_with_name(w: &mut self::NbtWriter, _name: &str, value: &Value) -> Result<()> {
        Nbt::write_to(w, value)
    }
}