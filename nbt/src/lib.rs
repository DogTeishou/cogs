use std::io;

#[cfg(test)]
pub mod test;

pub mod reader;
pub mod writer;

pub struct Nbt;

impl Nbt {
    pub fn new() -> Nbt {
        Nbt
    }
}

pub struct NbtAfter764;

impl NbtAfter764 {
    pub fn new() -> NbtAfter764 {
        NbtAfter764
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait NbtReadTrait {
    fn read_list(&mut self, r: &mut reader::NbtReader) -> Result<Vec<Value>>;
    fn read_compound(&mut self, r: &mut reader::NbtReader) -> Result<Vec<(String, Value)>>;
}

pub trait NbtWriteTrait {
    fn write_list(&mut self, w: &mut writer::NbtWriter, values: &[Value]);
    fn write_compound(&mut self, w: &mut writer::NbtWriter, values: &[(String, Value)]);
}

pub enum Error {
    Io(io::Error),
    Utf8(std::string::FromUtf8Error),
    InvalidTag(u8),
}

#[derive(Debug)]
pub enum Value {
    End,
    Byte(String, i8),
    Short(String, i16),
    Int(String, i32),
    Long(String,i64),
    Float(String, f32),
    Double(String, f64),
    ByteArray(String, Vec<i8>),
    String(String, String),
    List(String, Vec<Value>),
    Compound(String, Vec<(String, Value)>),
    IntArray(String, Vec<i32>),
    LongArray(String, Vec<i64>),
}

impl Value {
    pub fn tag(&self) -> u8 {
        match self {
            Value::End => 0,
            Value::Byte(_, _) => 1,
            Value::Short(_, _) => 2,
            Value::Int(_, _) => 3,
            Value::Long(_, _) => 4,
            Value::Float(_, _) => 5,
            Value::Double(_, _) => 6,
            Value::ByteArray(_, _) => 7,
            Value::String(_, _) => 8,
            Value::List(_, _) => 9,
            Value::Compound(_, _) => 10,
            Value::IntArray(_, _) => 11,
            Value::LongArray(_, _) => 12,
        }
    }

    pub fn value(&self) -> String {
        match self {
            Value::End => "End".to_string(),
            Value::Byte(s, v) => format!("{} {}", s, v),
            Value::Short(s, v) => format!("{} {}", s, v),
            Value::Int(s, v) => format!("{} {}", s, v),
            Value::Long(s, v) => format!("{} {}", s, v),
            Value::Float(s, v) => format!("{} {}", s, v),
            Value::Double(s, v) => format!("{} {}", s, v),
            Value::ByteArray(s, v) => format!("{} {:?}", s, v),
            Value::String(s, v) => format!("{} {}", s, v),
            Value::List(s, v) => {
                let mut i = 0;
                let mut str = String::new();
                for value in v {
                    str = format!("{} List(index: {}, tag: {}, value{:?}\n)", s,i, value.tag(), value.value());
                    i+=1;
                };
                str
            },
            Value::Compound(s, v) => {
                let mut i = 0;
                let mut str = String::new();
                for (key, value) in v {
                    str = format!("{} Compound(index: {}, key: {}, tag: {}, value{:?}\n)", s, i, key, value.tag(), value.value());
                    i+=1;
                };
                str
            },
            Value::IntArray(s, v) => format!("{} {:?}", s, v),
            Value::LongArray(s, v) => format!("{} {:?}", s, v),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::Utf8(e) => write!(f, "UTF-8 error: {}", e),
            Error::InvalidTag(tag) => write!(f, "Invalid tag: {}", tag),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}