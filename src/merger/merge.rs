use serde_value::Value;

#[derive(Debug, Clone)]
pub struct MergeValue {
    input: Value,
    mixin: Value,
}

impl MergeValue {
    pub fn new(input: Value, mixin: Value) -> MergeValue {
        MergeValue {
            input,
            mixin,
        }
    }

    pub fn merge(&mut self) {
        match &self.input {
            Value::Bool(x) => {
                println!("This is a bool: {:#?}", x);
            },
            Value::U8(x) => {
                println!("This is a u8: {:#?}", x);
            },
            Value::U16(x) => {
                println!("This is a u16: {:#?}", x);
            },
            Value::U32(x) => {
                println!("This is a u32: {:#?}", x);
            },
            Value::U64(x) => {
                println!("This is a u64: {:#?}", x);
            },
            Value::I8(x) => {
                println!("This is a i8: {:#?}", x);
            },
            Value::I16(x) => {
                println!("This is a i16: {:#?}", x);
            },
            Value::I32(x) => {
                println!("This is a 132: {:#?}", x);
            },
            Value::I64(x) => {
                println!("This is a i64: {:#?}", x);
            },
            Value::F32(x) => {
                println!("This is a f32: {:#?}", x);
            },
            Value::F64(x) => {
                println!("This is a f64: {:#?}", x);
            },
            Value::Char(x) => {
                println!("This is a char: {:#?}", x);
            },
            Value::String(x) => {
                println!("This is a string: {:#?}", x);
            },
            Value::Unit => {
                println!("This is a unit: {:#?}", Value::Unit);
            },
            Value::Option(x) => {
                println!("This is a option: {:#?}", x);
            },
            Value::Newtype(x) => {
                println!("This is a newtype: {:#?}", x);
            },
            Value::Seq(x) => {
                println!("This is a seq: {:#?}", x);
            },
            Value::Map(x) => {
                println!("This is a bool: {:#?}", x);
            },
            Value::Bytes(x) => {
                println!("This is a map: {:#?}", x);
            },
        }
    }
}
