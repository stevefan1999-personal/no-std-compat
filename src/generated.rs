//! Generated by generate.py located at the repository root
//! ./generate.py > src/generated.rs
pub mod alloc {
    pub use core::alloc::*;
    #[cfg(feature = "alloc")] pub use alloc::alloc::*;
}
pub mod any {
    pub use core::any::*;
}
pub mod array {
    pub use core::array::*;
}
pub mod ascii {
    pub use core::ascii::*;
}
pub mod borrow {
    pub use core::borrow::*;
    #[cfg(feature = "alloc")] pub use alloc::borrow::*;
}
pub mod boxed {
    #[cfg(feature = "alloc")] pub use alloc::boxed::*;
}
pub mod cell {
    pub use core::cell::*;
}
pub mod char {
    pub use core::char::*;
}
pub mod clone {
    pub use core::clone::*;
}
pub mod cmp {
    pub use core::cmp::*;
}
pub mod collections {
    #[cfg(feature = "alloc")] pub use alloc::collections::*;
    #[cfg(all(feature = "alloc", feature = "compat_hash"))] pub use alloc::collections::BTreeMap as HashMap;
    #[cfg(all(feature = "alloc", feature = "compat_hash"))] pub use alloc::collections::BTreeSet as HashSet;
}
pub mod convert {
    pub use core::convert::*;
}
pub mod default {
    pub use core::default::*;
}
pub mod f32 {
    pub use core::f32::*;
}
pub mod f64 {
    pub use core::f64::*;
}
pub mod ffi {
    pub use core::ffi::*;
}
pub mod fmt {
    pub use core::fmt::*;
    #[cfg(feature = "alloc")] pub use alloc::fmt::*;
}
pub mod future {
    pub use core::future::*;
}
#[cfg(feature = "compat_hash")]
pub mod hash {
    pub use core::cmp::Ord as Hash;
}
pub mod hint {
    pub use core::hint::*;
}
pub mod i128 {
    pub use core::i128::*;
}
pub mod i16 {
    pub use core::i16::*;
}
pub mod i32 {
    pub use core::i32::*;
}
pub mod i64 {
    pub use core::i64::*;
}
pub mod i8 {
    pub use core::i8::*;
}
pub mod intrinsics {
    pub use core::intrinsics::*;
}
pub mod isize {
    pub use core::isize::*;
}
pub mod iter {
    pub use core::iter::*;
}
pub mod marker {
    pub use core::marker::*;
}
pub mod mem {
    pub use core::mem::*;
}
pub mod num {
    pub use core::num::*;
}
pub mod ops {
    pub use core::ops::*;
}
pub mod option {
    pub use core::option::*;
}
pub mod panic {
    pub use core::panic::*;
}
pub mod panicking {
    pub use core::panicking::*;
}
pub mod pin {
    pub use core::pin::*;
}
pub mod prelude {
    pub mod v1 {
        pub use core::prelude::v1::*;

        #[cfg(feature = "alloc")] pub use alloc::prelude::v1::*;
        #[cfg(feature = "alloc")] pub use alloc::{format, vec};
        #[cfg(feature = "compat_macros")]
        pub use crate::{print, println, eprint, eprintln, dbg};
    }
}
pub mod ptr {
    pub use core::ptr::*;
}
pub mod raw {
    pub use core::raw::*;
}
pub mod rc {
    #[cfg(feature = "alloc")] pub use alloc::rc::*;
}
pub mod result {
    pub use core::result::*;
}
pub mod slice {
    pub use core::slice::*;
    #[cfg(feature = "alloc")] pub use alloc::slice::*;
}
pub mod str {
    pub use core::str::*;
    #[cfg(feature = "alloc")] pub use alloc::str::*;
}
pub mod string {
    #[cfg(feature = "alloc")] pub use alloc::string::*;
}
pub mod sync {
    pub use core::sync::*;
    #[cfg(feature = "alloc")] pub use alloc::sync::*;
}
pub mod task {
    pub use core::task::*;
}
pub mod time {
    pub use core::time::*;
}
pub mod u128 {
    pub use core::u128::*;
}
pub mod u16 {
    pub use core::u16::*;
}
pub mod u32 {
    pub use core::u32::*;
}
pub mod u64 {
    pub use core::u64::*;
}
pub mod u8 {
    pub use core::u8::*;
}
pub mod unicode {
    pub use core::unicode::*;
}
pub mod usize {
    pub use core::usize::*;
}
pub mod vec {
    #[cfg(feature = "alloc")] pub use alloc::vec::*;
}
