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
    fn write_list(&mut self, w: &mut writer::NbtWriter, values: &[Value]) -> Result<()>;
    fn write_compound(&mut self, w: &mut writer::NbtWriter, values: &[(String, Value)]) -> Result<()>;
}

pub enum Error {
    Io(io::Error),
    Utf8(std::string::FromUtf8Error),
    InvalidTag(u8),
}

#[derive(Debug)]
pub enum Value {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<Value>),
    Compound(Vec<(String, Value)>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl Value {
    pub fn tag(&self) -> u8 {
        match self {
            Value::End => 0,
            Value::Byte(_) => 1,
            Value::Short(_) => 2,
            Value::Int(_) => 3,
            Value::Long(_) => 4,
            Value::Float(_) => 5,
            Value::Double(_) => 6,
            Value::ByteArray(_) => 7,
            Value::String(_) => 8,
            Value::List(_) => 9,
            Value::Compound(_) => 10,
            Value::IntArray(_) => 11,
            Value::LongArray(_) => 12,
        }
    }

    pub fn value(&self) -> String {
        match self {
            Value::End => "End".to_string(),
            Value::Byte(v) => format!("{}", v),
            Value::Short(v) => format!("{}", v),
            Value::Int(v) => format!("{}", v),
            Value::Long(v) => format!("{}", v),
            Value::Float(v) => format!("{}", v),
            Value::Double(v) => format!("{}", v),
            Value::ByteArray(v) => format!("{:?}", v),
            Value::String(v) => format!("{}", v),
            Value::List(v) => {
                let mut i = 0;
                let mut s = String::new();
                for value in v {
                    s = format!("List(index: {}, tag: {}, value{:?}\n)", i, value.tag(), value.value());
                    i+=1;
                };
                s
            },
            Value::Compound(v) => {
                let mut i = 0;
                let mut s = String::new();
                for (key, value) in v {
                    s = format!("Compound(index: {}, key: {}, tag: {}, value{:?}\n)", i, key, value.tag(), value.value());
                    i+=1;
                };
                s
            },
            Value::IntArray(v) => format!("{:?}", v),
            Value::LongArray(v) => format!("{:?}", v),
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