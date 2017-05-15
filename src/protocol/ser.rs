use serde::ser;

pub mod errors {
    error_chain!{}
}

use self::errors::*;
use std::fmt::Display;
use std::result;
use bytes::BytesMut;

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::from_kind(ErrorKind::Msg(format!("{}", msg)))
    }
}

pub fn to_bytes<T>(data: T, buf: &mut BytesMut) -> Result<()>
    where T: ser::Serialize
{
    let mut s = Serializer::new(buf);
    data.serialize(s)
}

struct Serializer<'a> {
    dst: &'a mut BytesMut,
}

impl<'a> ser::Serializer for Serializer<'a> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeMap = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = ser::Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i8(self, v: i8) -> result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i16(self, v: i16) -> result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i32(self, v: i32) -> result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i64(self, v: i64) -> result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u8(self, v: u8) -> result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u16(self, v: u16) -> result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u32(self, v: u32) -> result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u64(self, v: u64) -> result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_f32(self, v: f32) -> result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_f64(self, v: f64) -> result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_char(self, v: char) -> result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_str(self, v: &str) -> result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_bytes(self, v: &[u8]) -> result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_none(self) -> result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_some<T: ?Sized>(self, v: &T) -> result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit(self) -> result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, v: &'static str) -> result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_variant(self,
                              name: &'static str,
                              variant_index: usize,
                              variant: &'static str)
                              -> result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T: ?Sized>(self,
                                           name: &'static str,
                                           value: &T)
                                           -> result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_newtype_variant<T: ?Sized>(self,
                                            name: &'static str,
                                            variant_index: usize,
                                            variant: &'static str,
                                            value: &T)
                                            -> result::Result<Self::Ok, Self::Error>
        where T: ser::Serialize
    {
        unimplemented!()
    }

    fn serialize_seq(mut self,
                     len: Option<usize>)
                     -> result::Result<Self::SerializeSeq, Self::Error> {
        unimplemented!()
    }

    fn serialize_seq_fixed_size(self,
                                size: usize)
                                -> result::Result<Self::SerializeSeq, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple(self, _len: usize) -> result::Result<Self::SerializeTuple, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_struct(self,
                              name: &'static str,
                              len: usize)
                              -> result::Result<Self::SerializeTupleStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_variant(self,
                               name: &'static str,
                               variant_index: usize,
                               variant: &'static str,
                               len: usize)
                               -> result::Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_map(mut self,
                     _len: Option<usize>)
                     -> result::Result<Self::SerializeMap, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct(mut self,
                        name: &'static str,
                        len: usize)
                        -> result::Result<Self::SerializeStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct_variant(self,
                                name: &'static str,
                                variant_index: usize,
                                variant: &'static str,
                                len: usize)
                                -> result::Result<Self::SerializeStructVariant, Self::Error> {
        unimplemented!()
    }
}

impl<'a> Serializer<'a> {
    pub fn new(dst: &'a mut BytesMut) -> Serializer<'a> {
        Serializer { dst: dst }
    }
}
