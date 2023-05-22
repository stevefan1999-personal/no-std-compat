#![no_std]

extern crate no_std_compat2 as std;
use std::prelude::v1::*;

// Test https://gitlab.com/jD91mZM2/no-std-compat2/issues/1
#[allow(unused_imports)]
use std::alloc::alloc_zeroed;

#[cfg(feature = "compat_hash")]
// If using the compat_hash feature of no_std_compat2, HashSet is
// implemented by the hashbrown crate.
use std::collections::HashSet as TheSet;

#[cfg(not(feature = "compat_hash"))]
// If you decide not to use this compatibility feature, you will have
// to use something like BTreeSet.
use std::collections::BTreeSet as TheSet;

#[cfg(feature = "compat_hash")]
use std::hash::Hash as RequiredTrait;
#[cfg(not(feature = "compat_hash"))]
use std::cmp::Ord as RequiredTrait;

pub fn remove_duplicates<T: Clone + RequiredTrait + Eq>(vec: &mut Vec<T>) {
    let mut found = TheSet::new();
    vec.retain(|item| {
        if found.contains(item) {
            return false;
        }
        found.insert(item.clone());
        true
    });
}

pub fn greet_someone(who: &str) -> Vec<String> {
    vec![
        format!("Greetings, {}!", who),
        format!("Hello, {}.", who),
        format!("How do you do, {}?", who)
    ]
}
