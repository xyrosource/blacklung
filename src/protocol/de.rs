use serde::de;

pub mod errors {
    error_chain!{}
}

use self::errors::*;
use std::fmt::Display;
use std::result;
use bytes::Bytes;

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::from_kind(ErrorKind::Msg(format!("{}", msg)))
    }
}

pub fn from_bytes<T>(bytes: &Bytes) -> Result<T>
    where T: de::Deserialize
{
    let mut d = Deserializer::new(bytes);
    T::deserialize(&mut d)
}

pub struct Deserializer<'a> {
    src: &'a Bytes,
}

impl<'a, 'b> de::Deserializer for &'b mut Deserializer<'a> {
    type Error = Error;

    fn deserialize<V>(self, visitor: V) -> result::Result<V::Value, Self::Error>
        where V: de::Visitor
    {
        unimplemented!()
    }

    forward_to_deserialize! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        seq_fixed_size bytes byte_buf map struct unit enum newtype_struct
        struct_field ignored_any unit_struct tuple_struct tuple option
    }
}

impl<'a> Deserializer<'a> {
    pub fn new(src: &'a Bytes) -> Deserializer<'a> {
        Deserializer { src: src }
    }
}
