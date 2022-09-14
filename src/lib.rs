//! Provides a simple macro to convert enums with newtype variants to `Option`s.
#![no_std]

/// Convert the given enum value to `Option`.
///
/// There are two syntactic forms to this macro:
///
/// 1. Enum expression followed by a comma and then one or more `|`-separated newtype (one-element
///    tuple) variants names that should be converted to `Some(_)`. Any other variants of the enum
///    will be converted to `None`.
///
///    ```no_run
///    # use as_variant::as_variant;
///    enum Result<T, E> {
///        Ok(T),
///        Err(E),
///    }
///
///    impl<T, E> Result<T, E> {
///        pub fn ok(self) -> Option<T> {
///            as_variant!(self, Self::Ok)
///        }
///    }
///    ```
/// 2. Enum expression followed by a comma, then a pattern matching one or more variants of that
///    enum and possibly capturing variables, then a fat right arrow followed by an expression that
///    will be put inside `Some(_)` if the pattern matches. If the pattern doesn't match, `None`
///    will be returned.
///
///    ```no_run
///    # use std::{
///    #     net::{Ipv4Addr, Ipv6Addr},
///    #     path::PathBuf,
///    # };
///    # use as_variant::as_variant;
///    enum ListenConfig {
///        Ipv4 { addr: Ipv4Addr, port: u16 },
///        Ipv6 { addr: Ipv6Addr, port: u16 },
///        Unix { path: PathBuf },
///    }
///
///    impl ListenConfig {
///        fn port(&self) -> Option<u16> {
///            as_variant!(self, Self::Ipv4 { port, .. } | Self::Ipv6 { port, .. } => *port)
///        }
///
///        fn privileged_port(&self) -> Option<u16> {
///            as_variant!(
///                self,
///                // using a guard after the pattern also works:    vvvvvvvvvvvvvvv
///                Self::Ipv4 { port, .. } | Self::Ipv6 { port, .. } if *port < 1024 => *port,
///            )
///        }
///    }
///    ```
///
/// # More Examples
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
    };
    ( $enum:expr, $pattern:pat $( if $guard:expr )? => $inner:expr $(,)? ) => {
        match $enum {
            $pattern $( if $guard )? => ::core::option::Option::Some($inner),
            _ => ::core::option::Option::None,
        }
    };
}
