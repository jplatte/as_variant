# as_variant

[![docs.rs](https://img.shields.io/crates/v/as_variant?color=blue&label=docs&style=flat-square)](https://docs.rs/as_variant)

A simple Rust macro to convert enums with newtype variants to `Option`s.

Basic example:

```rust
use std::ops::Deref;

use as_variant::as_variant;

enum Value {
    Integer(i64),
    String(String),
    Array(Vec<Value>),
}

impl Value {
    pub fn as_integer(&self) -> Option<i64> {
        as_variant!(self, Self::Integer).copied()
    }

    pub fn as_str(&self) -> Option<&str> {
        as_variant!(self, Self::String).map(Deref::deref)
    }

    pub fn as_array(&self) -> Option<&[Value]> {
        as_variant!(self, Self::Array).map(Deref::deref)
    }

    pub fn into_string(self) -> Option<String> {
        as_variant!(self, Self::String)
    }

    pub fn into_array(self) -> Option<Vec<Value>> {
        as_variant!(self, Self::Array)
    }
}
```
