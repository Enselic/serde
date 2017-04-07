#[doc(hidden)]
#[macro_export]
macro_rules! __serialize_unimplemented_method {
    ($func:ident $(<$t:ident>)* ($($arg:ty),*) -> $ret:ident) => {
        fn $func $(<$t: ?Sized + $crate::Serialize>)* (self $(, _: $arg)*) -> $crate::export::Result<Self::$ret, Self::Error> {
            unimplemented!()
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __serialize_unimplemented_helper {
    (bool) => {
        __serialize_unimplemented_method!(serialize_bool(bool) -> Ok);
    };
    (i8) => {
        __serialize_unimplemented_method!(serialize_i8(i8) -> Ok);
    };
    (i16) => {
        __serialize_unimplemented_method!(serialize_i16(i16) -> Ok);
    };
    (i32) => {
        __serialize_unimplemented_method!(serialize_i32(i32) -> Ok);
    };
    (i64) => {
        __serialize_unimplemented_method!(serialize_i64(i64) -> Ok);
    };
    (u8) => {
        __serialize_unimplemented_method!(serialize_u8(u8) -> Ok);
    };
    (u16) => {
        __serialize_unimplemented_method!(serialize_u16(u16) -> Ok);
    };
    (u32) => {
        __serialize_unimplemented_method!(serialize_u32(u32) -> Ok);
    };
    (u64) => {
        __serialize_unimplemented_method!(serialize_u64(u64) -> Ok);
    };
    (f32) => {
        __serialize_unimplemented_method!(serialize_f32(f32) -> Ok);
    };
    (f64) => {
        __serialize_unimplemented_method!(serialize_f64(f64) -> Ok);
    };
    (char) => {
        __serialize_unimplemented_method!(serialize_char(char) -> Ok);
    };
    (str) => {
        __serialize_unimplemented_method!(serialize_str(&str) -> Ok);
    };
    (bytes) => {
        __serialize_unimplemented_method!(serialize_bytes(&[u8]) -> Ok);
    };
    (none) => {
        __serialize_unimplemented_method!(serialize_none() -> Ok);
    };
    (some) => {
        __serialize_unimplemented_method!(serialize_some<T>(&T) -> Ok);
    };
    (unit) => {
        __serialize_unimplemented_method!(serialize_unit() -> Ok);
    };
    (unit_struct) => {
        __serialize_unimplemented_method!(serialize_unit_struct(&str) -> Ok);
    };
    (unit_variant) => {
        __serialize_unimplemented_method!(serialize_unit_variant(&str, usize, &str) -> Ok);
    };
    (newtype_struct) => {
        __serialize_unimplemented_method!(serialize_newtype_struct<T>(&str, &T) -> Ok);
    };
    (newtype_variant) => {
        __serialize_unimplemented_method!(serialize_newtype_variant<T>(&str, usize, &str, &T) -> Ok);
    };
    (seq) => {
        type SerializeSeq = $crate::ser::Impossible<Self::Ok, Self::Error>;
        __serialize_unimplemented_method!(serialize_seq(Option<usize>) -> SerializeSeq);
    };
    (seq_fixed_size) => {
        __serialize_unimplemented_method!(serialize_seq_fixed_size(usize) -> SerializeSeq);
    };
    (tuple) => {
        type SerializeTuple = $crate::ser::Impossible<Self::Ok, Self::Error>;
        __serialize_unimplemented_method!(serialize_tuple(usize) -> SerializeTuple);
    };
    (tuple_struct) => {
        type SerializeTupleStruct = $crate::ser::Impossible<Self::Ok, Self::Error>;
        __serialize_unimplemented_method!(serialize_tuple_struct(&str, usize) -> SerializeTupleStruct);
    };
    (tuple_variant) => {
        type SerializeTupleVariant = $crate::ser::Impossible<Self::Ok, Self::Error>;
        __serialize_unimplemented_method!(serialize_tuple_variant(&str, usize, &str, usize) -> SerializeTupleVariant);
    };
    (map) => {
        type SerializeMap = $crate::ser::Impossible<Self::Ok, Self::Error>;
        __serialize_unimplemented_method!(serialize_map(Option<usize>) -> SerializeMap);
    };
    (struct) => {
        type SerializeStruct = $crate::ser::Impossible<Self::Ok, Self::Error>;
        __serialize_unimplemented_method!(serialize_struct(&str, usize) -> SerializeStruct);
    };
    (struct_variant) => {
        type SerializeStructVariant = $crate::ser::Impossible<Self::Ok, Self::Error>;
        __serialize_unimplemented_method!(serialize_struct_variant(&str, usize, &str, usize) -> SerializeStructVariant);
    };
}

/// Used only by Serde doc tests. Not public API.
#[doc(hidden)]
#[macro_export]
macro_rules! __serialize_unimplemented {
    ($($func:ident)*) => {
        $(
            __serialize_unimplemented_helper!($func);
        )*
    };
}

/// Used only by Serde doc tests. Not public API.
#[doc(hidden)]
#[macro_export]
macro_rules! __serde_ignore_tokens {
    ($($tt:tt)+) => {}
}