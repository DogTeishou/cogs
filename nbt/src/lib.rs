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
    fn read_byte_array(r: &mut reader::NbtReader) -> Result<Vec<i8>>;
    fn read_nbt_string(r: &mut reader::NbtReader) -> Result<String>;
    fn read_int_array(r: &mut reader::NbtReader) -> Result<Vec<i32>>;
    fn read_long_array(r: &mut reader::NbtReader) -> Result<Vec<i64>>;
    fn read_list(r: &mut reader::NbtReader) -> Result<Vec<Value>>;
    fn read_compound(r: &mut reader::NbtReader) -> Result<Vec<(String, Value)>>;
    fn from_reader(r: &mut reader::NbtReader) -> Result<Value>;
}

pub trait NbtWriteTrait {
    fn write_byte_array(w: &mut writer::NbtWriter, value: &[i8]);
    fn write_nbt_string(w: &mut writer::NbtWriter, value: &str);
    fn write_int_array(w: &mut writer::NbtWriter, value: &[i32]);
    fn write_long_array(w: &mut writer::NbtWriter, value: &[i64]);
    fn write_list(w: &mut writer::NbtWriter, value: &[Value]) -> Result<()>;
    fn write_compound(w: &mut writer::NbtWriter, name: Option<&String>, value: &[(String, Value)]) -> Result<()>;
    fn write_to(w: &mut writer::NbtWriter, value: &Value) -> Result<()>;
    fn write_to_with_name(w: &mut writer::NbtWriter, name: &str, value: &Value) -> Result<()>;
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Utf8(std::string::FromUtf8Error),
    InvalidTag(u8),
    RootTagNotCompound(u8),
    ListTypeNotSame,
}

#[derive(Debug)]
pub enum Value {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<Value>),
    Compound(Option<String>, Vec<(String, Value)>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl Value {
    pub fn tag(&self) -> u8 {
        match self {
            Value::Byte(_) => 1,
            Value::Short(_) => 2,
            Value::Int(_) => 3,
            Value::Long(_) => 4,
            Value::Float(_) => 5,
            Value::Double(_) => 6,
            Value::ByteArray(_) => 7,
            Value::String(_) => 8,
            Value::List(_) => 9,
            Value::Compound(_, _) => 10,
            Value::IntArray(_) => 11,
            Value::LongArray(_) => 12,
        }
    }

    pub fn value(&self) -> String {
        match self {
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
                let mut str = String::new();
                for value in v {
                    str = format!("List(index: {}, tag: {}, value{}\n)", i, value.tag(), value.value());
                    i+=1;
                };
                str
            },
            Value::Compound(s, v) => {
                let mut i = 0;
                let mut str = String::new();
                let s = match s {
                    Some(ss) => ss.clone(),
                    None => "".to_string(),
                };
                for (key, value) in v {
                    str = format!("Name: {} Compound(index: {}, key: {}, tag: {}, value{}\n)", s, i, key, value.tag(), value.value());
                    i+=1;
                };
                str
            },
            Value::IntArray(v) => format!("{:?}", v),
            Value::LongArray(v) => format!("{:?}", v),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::RootTagNotCompound(tag) => write!(f, "Root tag is not a compound, got: {}", tag),
            Error::ListTypeNotSame => write!(f, "List type is not same"),
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