//! Provides a simple macro to convert enums with newtype variants to `Option`s.
#![no_std]

/// Convert the given enum value to `Option`.
///
/// The second argument must be one or more `|`-separated newtype (one-element tuple) variants that
/// should be converted to `Some(_)`. Any other variants of the enum will be converted to `None`.
///
/// # Examples
///
/// ```no_run
/// use std::ops::Deref;
///
/// use as_variant::as_variant;
///
/// enum Value {
///     Integer(i64),
///     String(String),
///     Array(Vec<Value>),
/// }
///
/// impl Value {
///     pub fn as_integer(&self) -> Option<i64> {
///         as_variant!(self, Self::Integer).copied()
///     }
///
///     pub fn as_str(&self) -> Option<&str> {
///         as_variant!(self, Self::String).map(Deref::deref)
///     }
///
///     pub fn as_array(&self) -> Option<&[Value]> {
///         as_variant!(self, Self::Array).map(Deref::deref)
///     }
///
///     pub fn into_string(self) -> Option<String> {
///         as_variant!(self, Self::String)
///     }
///
///     pub fn into_array(self) -> Option<Vec<Value>> {
///         as_variant!(self, Self::Array)
///     }
/// }
/// ```
///
/// ```
/// use as_variant::as_variant;
///
/// enum Either3<A, B, C> {
///     A(A),
///     B(B),
///     C(C),
/// }
///
/// impl<T, U> Either3<T, T, U> {
///     fn as_a_or_b(&self) -> Option<&T> {
///         as_variant!(self, Self::A | Self::B)
///     }
///
///     fn into_a_or_b(self) -> Option<T> {
///         as_variant!(self, Self::A | Self::B)
///     }
/// }
///
/// let a: Either3<_, _, &str> = Either3::A(1);
/// assert_eq!(a.as_a_or_b(), Some(&1));
/// assert_eq!(a.into_a_or_b(), Some(1));
///
/// let b: Either3<_, _, u8> = Either3::B("hello");
/// assert_eq!(b.as_a_or_b(), Some(&"hello"));
/// assert_eq!(b.into_a_or_b(), Some("hello"));
///
/// let c: Either3<char, _, _> = Either3::C('c');
/// assert_eq!(c.as_a_or_b(), None);
/// assert_eq!(c.into_a_or_b(), None);
/// ```
#[macro_export]
macro_rules! as_variant {
    ( $enum:expr, $( $variants:path )|* ) => {
        match $enum {
            $( $variants(inner) )|* => ::core::option::Option::Some(inner),
            _ => ::core::option::Option::None,
        }
    }
}
