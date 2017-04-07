use core::fmt::{self, Display};

use ser::{self, Serialize, Serializer, SerializeMap, SerializeStruct, Impossible};

#[cfg(any(feature = "std", feature = "collections"))]
use self::content::{SerializeTupleVariantAsMapValue, SerializeStructVariantAsMapValue};

#[cfg(feature = "std")]
use std::error;

/// Not public API.
pub fn serialize_tagged_newtype<S, T>(serializer: S,
                                      type_ident: &'static str,
                                      variant_ident: &'static str,
                                      tag: &'static str,
                                      variant_name: &'static str,
                                      value: &T)
                                      -> Result<S::Ok, S::Error>
    where S: Serializer,
          T: Serialize
{
    value.serialize(TaggedSerializer {
                        type_ident: type_ident,
                        variant_ident: variant_ident,
                        tag: tag,
                        variant_name: variant_name,
                        delegate: serializer,
                    })
}

struct TaggedSerializer<S> {
    type_ident: &'static str,
    variant_ident: &'static str,
    tag: &'static str,
    variant_name: &'static str,
    delegate: S,
}

enum Unsupported {
    Boolean,
    Integer,
    Float,
    Char,
    String,
    ByteArray,
    Optional,
    Unit,
    UnitStruct,
    Sequence,
    Tuple,
    TupleStruct,
    #[cfg(not(any(feature = "std", feature = "collections")))]
    Enum,
}

impl Display for Unsupported {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Unsupported::Boolean => formatter.write_str("a boolean"),
            Unsupported::Integer => formatter.write_str("an integer"),
            Unsupported::Float => formatter.write_str("a float"),
            Unsupported::Char => formatter.write_str("a char"),
            Unsupported::String => formatter.write_str("a string"),
            Unsupported::ByteArray => formatter.write_str("a byte array"),
            Unsupported::Optional => formatter.write_str("an optional"),
            Unsupported::Unit => formatter.write_str("unit"),
            Unsupported::UnitStruct => formatter.write_str("a unit struct"),
            Unsupported::Sequence => formatter.write_str("a sequence"),
            Unsupported::Tuple => formatter.write_str("a tuple"),
            Unsupported::TupleStruct => formatter.write_str("a tuple struct"),
            #[cfg(not(any(feature = "std", feature = "collections")))]
            Unsupported::Enum => formatter.write_str("an enum"),
        }
    }
}

impl<S> TaggedSerializer<S>
    where S: Serializer
{
    fn bad_type(self, what: Unsupported) -> S::Error {
        ser::Error::custom(format_args!(
            "cannot serialize tagged newtype variant {}::{} containing {}",
            self.type_ident,
            self.variant_ident,
            what))
    }
}

impl<S> Serializer for TaggedSerializer<S>
    where S: Serializer
{
    type Ok = S::Ok;
    type Error = S::Error;

    type SerializeSeq = Impossible<S::Ok, S::Error>;
    type SerializeTuple = Impossible<S::Ok, S::Error>;
    type SerializeTupleStruct = Impossible<S::Ok, S::Error>;
    type SerializeMap = S::SerializeMap;
    type SerializeStruct = S::SerializeStruct;

    #[cfg(not(any(feature = "std", feature = "collections")))]
    type SerializeTupleVariant = Impossible<S::Ok, S::Error>;
    #[cfg(any(feature = "std", feature = "collections"))]
    type SerializeTupleVariant = SerializeTupleVariantAsMapValue<S::SerializeMap>;

    #[cfg(not(any(feature = "std", feature = "collections")))]
    type SerializeStructVariant = Impossible<S::Ok, S::Error>;
    #[cfg(any(feature = "std", feature = "collections"))]
    type SerializeStructVariant = SerializeStructVariantAsMapValue<S::SerializeMap>;

    fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Boolean))
    }

    fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Integer))
    }

    fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Integer))
    }

    fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Integer))
    }

    fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Integer))
    }

    fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Integer))
    }

    fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Integer))
    }

    fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Integer))
    }

    fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Integer))
    }

    fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Float))
    }

    fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Float))
    }

    fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Char))
    }

    fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::String))
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::ByteArray))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Optional))
    }

    fn serialize_some<T: ?Sized>(self, _: &T) -> Result<Self::Ok, Self::Error>
        where T: Serialize
    {
        Err(self.bad_type(Unsupported::Optional))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Unit))
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::UnitStruct))
    }

    fn serialize_unit_variant(self,
                              _: &'static str,
                              _: usize,
                              inner_variant: &'static str)
                              -> Result<Self::Ok, Self::Error> {
        let mut map = try!(self.delegate.serialize_map(Some(2)));
        try!(map.serialize_entry(self.tag, self.variant_name));
        try!(map.serialize_entry(inner_variant, &()));
        map.end()
    }

    fn serialize_newtype_struct<T: ?Sized>(self,
                                           _: &'static str,
                                           value: &T)
                                           -> Result<Self::Ok, Self::Error>
        where T: Serialize
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(self,
                                            _: &'static str,
                                            _: usize,
                                            inner_variant: &'static str,
                                            inner_value: &T)
                                            -> Result<Self::Ok, Self::Error>
        where T: Serialize
    {
        let mut map = try!(self.delegate.serialize_map(Some(2)));
        try!(map.serialize_entry(self.tag, self.variant_name));
        try!(map.serialize_entry(inner_variant, inner_value));
        map.end()
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(self.bad_type(Unsupported::Sequence))
    }

    fn serialize_seq_fixed_size(self, _: usize) -> Result<Self::SerializeSeq, Self::Error> {
        Err(self.bad_type(Unsupported::Sequence))
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(self.bad_type(Unsupported::Tuple))
    }

    fn serialize_tuple_struct(self,
                              _: &'static str,
                              _: usize)
                              -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(self.bad_type(Unsupported::TupleStruct))
    }

    #[cfg(not(any(feature = "std", feature = "collections")))]
    fn serialize_tuple_variant(self,
                               _: &'static str,
                               _: usize,
                               _: &'static str,
                               _: usize)
                               -> Result<Self::SerializeTupleVariant, Self::Error> {
        // Lack of push-based serialization means we need to buffer the content
        // of the tuple variant, so it requires std.
        Err(self.bad_type(Unsupported::Enum))
    }

    #[cfg(any(feature = "std", feature = "collections"))]
    fn serialize_tuple_variant(self,
                               _: &'static str,
                               _: usize,
                               inner_variant: &'static str,
                               len: usize)
                               -> Result<Self::SerializeTupleVariant, Self::Error> {
        let mut map = try!(self.delegate.serialize_map(Some(2)));
        try!(map.serialize_entry(self.tag, self.variant_name));
        try!(map.serialize_key(inner_variant));
        Ok(SerializeTupleVariantAsMapValue::new(map, inner_variant, len))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let mut map = try!(self.delegate.serialize_map(len.map(|len| len + 1)));
        try!(map.serialize_entry(self.tag, self.variant_name));
        Ok(map)
    }

    fn serialize_struct(self,
                        name: &'static str,
                        len: usize)
                        -> Result<Self::SerializeStruct, Self::Error> {
        let mut state = try!(self.delegate.serialize_struct(name, len + 1));
        try!(state.serialize_field(self.tag, self.variant_name));
        Ok(state)
    }

    #[cfg(not(any(feature = "std", feature = "collections")))]
    fn serialize_struct_variant(self,
                                _: &'static str,
                                _: usize,
                                _: &'static str,
                                _: usize)
                                -> Result<Self::SerializeStructVariant, Self::Error> {
        // Lack of push-based serialization means we need to buffer the content
        // of the struct variant, so it requires std.
        Err(self.bad_type(Unsupported::Enum))
    }

    #[cfg(any(feature = "std", feature = "collections"))]
    fn serialize_struct_variant(self,
                                _: &'static str,
                                _: usize,
                                inner_variant: &'static str,
                                len: usize)
                                -> Result<Self::SerializeStructVariant, Self::Error> {
        let mut map = try!(self.delegate.serialize_map(Some(2)));
        try!(map.serialize_entry(self.tag, self.variant_name));
        try!(map.serialize_key(inner_variant));
        Ok(SerializeStructVariantAsMapValue::new(map, inner_variant, len))
    }

    #[cfg(not(any(feature = "std", feature = "collections")))]
    fn collect_str<T: ?Sized>(self, _: &T) -> Result<Self::Ok, Self::Error>
        where T: Display
    {
        Err(self.bad_type(Unsupported::String))
    }
}

/// Used only by Serde doc tests. Not public API.
#[doc(hidden)]
#[derive(Debug)]
pub struct Error;

impl ser::Error for Error {
    fn custom<T: Display>(_: T) -> Self {
        unimplemented!()
    }
}

#[cfg(feature = "std")]
impl error::Error for Error {
    fn description(&self) -> &str {
        unimplemented!()
    }
}

impl Display for Error {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

#[cfg(any(feature = "std", feature = "collections"))]
mod content {
    use core::marker::PhantomData;

    #[cfg(all(not(feature = "std"), feature = "collections"))]
    use collections::{String, Vec};

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    use alloc::boxed::Box;

    #[cfg(feature = "collections")]
    use collections::borrow::ToOwned;

    use ser::{self, Serialize, Serializer};

    pub struct SerializeTupleVariantAsMapValue<M> {
        map: M,
        name: &'static str,
        fields: Vec<Content>,
    }

    impl<M> SerializeTupleVariantAsMapValue<M> {
        pub fn new(map: M, name: &'static str, len: usize) -> Self {
            SerializeTupleVariantAsMapValue {
                map: map,
                name: name,
                fields: Vec::with_capacity(len),
            }
        }
    }

    impl<M> ser::SerializeTupleVariant for SerializeTupleVariantAsMapValue<M>
        where M: ser::SerializeMap
    {
        type Ok = M::Ok;
        type Error = M::Error;

        fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), M::Error> {
            let value = try!(value.serialize(ContentSerializer::<M::Error>::new()));
            self.fields.push(value);
            Ok(())
        }

        fn end(mut self) -> Result<M::Ok, M::Error> {
            try!(self.map.serialize_value(&Content::TupleStruct(self.name, self.fields)));
            self.map.end()
        }
    }

    pub struct SerializeStructVariantAsMapValue<M> {
        map: M,
        name: &'static str,
        fields: Vec<(&'static str, Content)>,
    }

    impl<M> SerializeStructVariantAsMapValue<M> {
        pub fn new(map: M, name: &'static str, len: usize) -> Self {
            SerializeStructVariantAsMapValue {
                map: map,
                name: name,
                fields: Vec::with_capacity(len),
            }
        }
    }

    impl<M> ser::SerializeStructVariant for SerializeStructVariantAsMapValue<M>
        where M: ser::SerializeMap
    {
        type Ok = M::Ok;
        type Error = M::Error;

        fn serialize_field<T: ?Sized + Serialize>(&mut self,
                                                key: &'static str,
                                                value: &T)
                                                -> Result<(), M::Error> {
            let value = try!(value.serialize(ContentSerializer::<M::Error>::new()));
            self.fields.push((key, value));
            Ok(())
        }

        fn end(mut self) -> Result<M::Ok, M::Error> {
            try!(self.map.serialize_value(&Content::Struct(self.name, self.fields)));
            self.map.end()
        }
    }

    #[derive(Debug)]
    enum Content {
        Bool(bool),

        U8(u8),
        U16(u16),
        U32(u32),
        U64(u64),

        I8(i8),
        I16(i16),
        I32(i32),
        I64(i64),

        F32(f32),
        F64(f64),

        Char(char),
        String(String),
        Bytes(Vec<u8>),

        None,
        Some(Box<Content>),

        Unit,
        UnitStruct(&'static str),
        UnitVariant(&'static str, usize, &'static str),
        NewtypeStruct(&'static str, Box<Content>),
        NewtypeVariant(&'static str, usize, &'static str, Box<Content>),

        Seq(Vec<Content>),
        SeqFixedSize(Vec<Content>),
        Tuple(Vec<Content>),
        TupleStruct(&'static str, Vec<Content>),
        TupleVariant(&'static str, usize, &'static str, Vec<Content>),
        Map(Vec<(Content, Content)>),
        Struct(&'static str, Vec<(&'static str, Content)>),
        StructVariant(&'static str, usize, &'static str, Vec<(&'static str, Content)>),
    }

    impl Serialize for Content {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: Serializer
        {
            match *self {
                Content::Bool(b) => serializer.serialize_bool(b),
                Content::U8(u) => serializer.serialize_u8(u),
                Content::U16(u) => serializer.serialize_u16(u),
                Content::U32(u) => serializer.serialize_u32(u),
                Content::U64(u) => serializer.serialize_u64(u),
                Content::I8(i) => serializer.serialize_i8(i),
                Content::I16(i) => serializer.serialize_i16(i),
                Content::I32(i) => serializer.serialize_i32(i),
                Content::I64(i) => serializer.serialize_i64(i),
                Content::F32(f) => serializer.serialize_f32(f),
                Content::F64(f) => serializer.serialize_f64(f),
                Content::Char(c) => serializer.serialize_char(c),
                Content::String(ref s) => serializer.serialize_str(s),
                Content::Bytes(ref b) => serializer.serialize_bytes(b),
                Content::None => serializer.serialize_none(),
                Content::Some(ref c) => serializer.serialize_some(&**c),
                Content::Unit => serializer.serialize_unit(),
                Content::UnitStruct(n) => serializer.serialize_unit_struct(n),
                Content::UnitVariant(n, i, v) => serializer.serialize_unit_variant(n, i, v),
                Content::NewtypeStruct(n, ref c) => serializer.serialize_newtype_struct(n, &**c),
                Content::NewtypeVariant(n, i, v, ref c) => {
                    serializer.serialize_newtype_variant(n, i, v, &**c)
                }
                Content::Seq(ref elements) => elements.serialize(serializer),
                Content::SeqFixedSize(ref elements) => {
                    use ser::SerializeSeq;
                    let mut seq = try!(serializer.serialize_seq_fixed_size(elements.len()));
                    for e in elements {
                        try!(seq.serialize_element(e));
                    }
                    seq.end()
                }
                Content::Tuple(ref elements) => {
                    use ser::SerializeTuple;
                    let mut tuple = try!(serializer.serialize_tuple(elements.len()));
                    for e in elements {
                        try!(tuple.serialize_element(e));
                    }
                    tuple.end()
                }
                Content::TupleStruct(n, ref fields) => {
                    use ser::SerializeTupleStruct;
                    let mut ts = try!(serializer.serialize_tuple_struct(n, fields.len()));
                    for f in fields {
                        try!(ts.serialize_field(f));
                    }
                    ts.end()
                }
                Content::TupleVariant(n, i, v, ref fields) => {
                    use ser::SerializeTupleVariant;
                    let mut tv = try!(serializer.serialize_tuple_variant(n, i, v, fields.len()));
                    for f in fields {
                        try!(tv.serialize_field(f));
                    }
                    tv.end()
                }
                Content::Map(ref entries) => {
                    use ser::SerializeMap;
                    let mut map = try!(serializer.serialize_map(Some(entries.len())));
                    for &(ref k, ref v) in entries {
                        try!(map.serialize_entry(k, v));
                    }
                    map.end()
                }
                Content::Struct(n, ref fields) => {
                    use ser::SerializeStruct;
                    let mut s = try!(serializer.serialize_struct(n, fields.len()));
                    for &(k, ref v) in fields {
                        try!(s.serialize_field(k, v));
                    }
                    s.end()
                }
                Content::StructVariant(n, i, v, ref fields) => {
                    use ser::SerializeStructVariant;
                    let mut sv = try!(serializer.serialize_struct_variant(n, i, v, fields.len()));
                    for &(k, ref v) in fields {
                        try!(sv.serialize_field(k, v));
                    }
                    sv.end()
                }
            }
        }
    }

    struct ContentSerializer<E> {
        error: PhantomData<E>,
    }

    impl<E> ContentSerializer<E> {
        fn new() -> Self {
            ContentSerializer { error: PhantomData }
        }
    }

    impl<E> Serializer for ContentSerializer<E>
        where E: ser::Error
    {
        type Ok = Content;
        type Error = E;

        type SerializeSeq = SerializeSeq<E>;
        type SerializeTuple = SerializeTuple<E>;
        type SerializeTupleStruct = SerializeTupleStruct<E>;
        type SerializeTupleVariant = SerializeTupleVariant<E>;
        type SerializeMap = SerializeMap<E>;
        type SerializeStruct = SerializeStruct<E>;
        type SerializeStructVariant = SerializeStructVariant<E>;

        fn serialize_bool(self, v: bool) -> Result<Content, E> {
            Ok(Content::Bool(v))
        }

        fn serialize_i8(self, v: i8) -> Result<Content, E> {
            Ok(Content::I8(v))
        }

        fn serialize_i16(self, v: i16) -> Result<Content, E> {
            Ok(Content::I16(v))
        }

        fn serialize_i32(self, v: i32) -> Result<Content, E> {
            Ok(Content::I32(v))
        }

        fn serialize_i64(self, v: i64) -> Result<Content, E> {
            Ok(Content::I64(v))
        }

        fn serialize_u8(self, v: u8) -> Result<Content, E> {
            Ok(Content::U8(v))
        }

        fn serialize_u16(self, v: u16) -> Result<Content, E> {
            Ok(Content::U16(v))
        }

        fn serialize_u32(self, v: u32) -> Result<Content, E> {
            Ok(Content::U32(v))
        }

        fn serialize_u64(self, v: u64) -> Result<Content, E> {
            Ok(Content::U64(v))
        }

        fn serialize_f32(self, v: f32) -> Result<Content, E> {
            Ok(Content::F32(v))
        }

        fn serialize_f64(self, v: f64) -> Result<Content, E> {
            Ok(Content::F64(v))
        }

        fn serialize_char(self, v: char) -> Result<Content, E> {
            Ok(Content::Char(v))
        }

        fn serialize_str(self, value: &str) -> Result<Content, E> {
            Ok(Content::String(value.to_owned()))
        }

        fn serialize_bytes(self, value: &[u8]) -> Result<Content, E> {
            Ok(Content::Bytes(value.to_owned()))
        }

        fn serialize_none(self) -> Result<Content, E> {
            Ok(Content::None)
        }

        fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Content, E> {
            Ok(Content::Some(Box::new(try!(value.serialize(self)))))
        }

        fn serialize_unit(self) -> Result<Content, E> {
            Ok(Content::Unit)
        }

        fn serialize_unit_struct(self, name: &'static str) -> Result<Content, E> {
            Ok(Content::UnitStruct(name))
        }

        fn serialize_unit_variant(self,
                                name: &'static str,
                                variant_index: usize,
                                variant: &'static str)
                                -> Result<Content, E> {
            Ok(Content::UnitVariant(name, variant_index, variant))
        }

        fn serialize_newtype_struct<T: ?Sized + Serialize>(self,
                                                        name: &'static str,
                                                        value: &T)
                                                        -> Result<Content, E> {
            Ok(Content::NewtypeStruct(name, Box::new(try!(value.serialize(self)))))
        }

        fn serialize_newtype_variant<T: ?Sized + Serialize>(self,
                                                            name: &'static str,
                                                            variant_index: usize,
                                                            variant: &'static str,
                                                            value: &T)
                                                            -> Result<Content, E> {
            Ok(Content::NewtypeVariant(name,
                                    variant_index,
                                    variant,
                                    Box::new(try!(value.serialize(self)))))
        }

        fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, E> {
            Ok(SerializeSeq {
                fixed_size: false,
                elements: Vec::with_capacity(len.unwrap_or(0)),
                error: PhantomData,
            })
        }

        fn serialize_seq_fixed_size(self, size: usize) -> Result<Self::SerializeSeq, E> {
            Ok(SerializeSeq {
                fixed_size: true,
                elements: Vec::with_capacity(size),
                error: PhantomData,
            })
        }

        fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, E> {
            Ok(SerializeTuple {
                elements: Vec::with_capacity(len),
                error: PhantomData,
            })
        }

        fn serialize_tuple_struct(self,
                                name: &'static str,
                                len: usize)
                                -> Result<Self::SerializeTupleStruct, E> {
            Ok(SerializeTupleStruct {
                name: name,
                fields: Vec::with_capacity(len),
                error: PhantomData,
            })
        }

        fn serialize_tuple_variant(self,
                                name: &'static str,
                                variant_index: usize,
                                variant: &'static str,
                                len: usize)
                                -> Result<Self::SerializeTupleVariant, E> {
            Ok(SerializeTupleVariant {
                name: name,
                variant_index: variant_index,
                variant: variant,
                fields: Vec::with_capacity(len),
                error: PhantomData,
            })
        }

        fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, E> {
            Ok(SerializeMap {
                entries: Vec::with_capacity(len.unwrap_or(0)),
                key: None,
                error: PhantomData,
            })
        }

        fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, E> {
            Ok(SerializeStruct {
                name: name,
                fields: Vec::with_capacity(len),
                error: PhantomData,
            })
        }

        fn serialize_struct_variant(self,
                                    name: &'static str,
                                    variant_index: usize,
                                    variant: &'static str,
                                    len: usize)
                                    -> Result<Self::SerializeStructVariant, E> {
            Ok(SerializeStructVariant {
                name: name,
                variant_index: variant_index,
                variant: variant,
                fields: Vec::with_capacity(len),
                error: PhantomData,
            })
        }
    }

    struct SerializeSeq<E> {
        fixed_size: bool,
        elements: Vec<Content>,
        error: PhantomData<E>,
    }

    impl<E> ser::SerializeSeq for SerializeSeq<E>
        where E: ser::Error
    {
        type Ok = Content;
        type Error = E;

        fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), E> {
            let value = try!(value.serialize(ContentSerializer::<E>::new()));
            self.elements.push(value);
            Ok(())
        }

        fn end(self) -> Result<Content, E> {
            Ok(if self.fixed_size {
                Content::SeqFixedSize(self.elements)
            } else {
                Content::Seq(self.elements)
            })
        }
    }

    struct SerializeTuple<E> {
        elements: Vec<Content>,
        error: PhantomData<E>,
    }

    impl<E> ser::SerializeTuple for SerializeTuple<E>
        where E: ser::Error
    {
        type Ok = Content;
        type Error = E;

        fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), E> {
            let value = try!(value.serialize(ContentSerializer::<E>::new()));
            self.elements.push(value);
            Ok(())
        }

        fn end(self) -> Result<Content, E> {
            Ok(Content::Tuple(self.elements))
        }
    }

    struct SerializeTupleStruct<E> {
        name: &'static str,
        fields: Vec<Content>,
        error: PhantomData<E>,
    }

    impl<E> ser::SerializeTupleStruct for SerializeTupleStruct<E>
        where E: ser::Error
    {
        type Ok = Content;
        type Error = E;

        fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), E> {
            let value = try!(value.serialize(ContentSerializer::<E>::new()));
            self.fields.push(value);
            Ok(())
        }

        fn end(self) -> Result<Content, E> {
            Ok(Content::TupleStruct(self.name, self.fields))
        }
    }

    struct SerializeTupleVariant<E> {
        name: &'static str,
        variant_index: usize,
        variant: &'static str,
        fields: Vec<Content>,
        error: PhantomData<E>,
    }

    impl<E> ser::SerializeTupleVariant for SerializeTupleVariant<E>
        where E: ser::Error
    {
        type Ok = Content;
        type Error = E;

        fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), E> {
            let value = try!(value.serialize(ContentSerializer::<E>::new()));
            self.fields.push(value);
            Ok(())
        }

        fn end(self) -> Result<Content, E> {
            Ok(Content::TupleVariant(self.name, self.variant_index, self.variant, self.fields))
        }
    }

    struct SerializeMap<E> {
        entries: Vec<(Content, Content)>,
        key: Option<Content>,
        error: PhantomData<E>,
    }

    impl<E> ser::SerializeMap for SerializeMap<E>
        where E: ser::Error
    {
        type Ok = Content;
        type Error = E;

        fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), E> {
            let key = try!(key.serialize(ContentSerializer::<E>::new()));
            self.key = Some(key);
            Ok(())
        }

        fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), E> {
            let key = self.key.take().expect("serialize_value called before serialize_key");
            let value = try!(value.serialize(ContentSerializer::<E>::new()));
            self.entries.push((key, value));
            Ok(())
        }

        fn end(self) -> Result<Content, E> {
            Ok(Content::Map(self.entries))
        }

        fn serialize_entry<K: ?Sized + Serialize, V: ?Sized + Serialize>(&mut self,
                                                                        key: &K,
                                                                        value: &V)
                                                                        -> Result<(), E> {
            let key = try!(key.serialize(ContentSerializer::<E>::new()));
            let value = try!(value.serialize(ContentSerializer::<E>::new()));
            self.entries.push((key, value));
            Ok(())
        }
    }

    struct SerializeStruct<E> {
        name: &'static str,
        fields: Vec<(&'static str, Content)>,
        error: PhantomData<E>,
    }

    impl<E> ser::SerializeStruct for SerializeStruct<E>
        where E: ser::Error
    {
        type Ok = Content;
        type Error = E;

        fn serialize_field<T: ?Sized + Serialize>(&mut self,
                                                key: &'static str,
                                                value: &T)
                                                -> Result<(), E> {
            let value = try!(value.serialize(ContentSerializer::<E>::new()));
            self.fields.push((key, value));
            Ok(())
        }

        fn end(self) -> Result<Content, E> {
            Ok(Content::Struct(self.name, self.fields))
        }
    }

    struct SerializeStructVariant<E> {
        name: &'static str,
        variant_index: usize,
        variant: &'static str,
        fields: Vec<(&'static str, Content)>,
        error: PhantomData<E>,
    }

    impl<E> ser::SerializeStructVariant for SerializeStructVariant<E>
        where E: ser::Error
    {
        type Ok = Content;
        type Error = E;

        fn serialize_field<T: ?Sized + Serialize>(&mut self,
                                                key: &'static str,
                                                value: &T)
                                                -> Result<(), E> {
            let value = try!(value.serialize(ContentSerializer::<E>::new()));
            self.fields.push((key, value));
            Ok(())
        }

        fn end(self) -> Result<Content, E> {
            Ok(Content::StructVariant(self.name, self.variant_index, self.variant, self.fields))
        }
    }
}