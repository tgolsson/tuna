// Author: Tom Solberg <me@sbg.dev>
// Copyright © 2021, Tom Solberg, all rights reserved.
// Created: 14 May 2021

/*!

*/
use std::any::Any;

use crate::{Tuneable, TUNA_STATE};

/// Implemented by types that can be
pub trait AsTuneable: Any + Clone + Sized {
    type Result;

    fn make_tuneable(&self) -> Tuneable;
    fn update(tuneable: &mut Tuneable, var: Self::Result);
    fn reset(tuneable: &mut Tuneable);
    fn from_tuneable(v: &Tuneable) -> Option<Self::Result>;
}

/// Register a tuneable variable with a default variable. If variable
/// already exists, won't do anything.
pub fn register<T: AsTuneable>(category: &str, name: &str, value: &T) {
    let mut tuna = TUNA_STATE.write();

    if !tuna.contains_key(category) {
        tuna.insert(category.to_owned(), Default::default());
    }

    let group = tuna.get_mut(category).expect("must be inserted above");

    if group.contains_key(name) {
        return;
    }

    group.insert(name.to_owned(), value.make_tuneable());
}

/// Get a tunable variable or a default.
pub fn get<T: AsTuneable>(category: &str, name: &str) -> Option<T::Result> {
    let tuna = TUNA_STATE.read();

    tuna.get(category)
        .and_then(|group| group.get(name))
        .and_then(|value| T::from_tuneable(value))
}

pub fn set<T: AsTuneable>(category: &str, name: &str, value: T::Result) {
    let mut tuna = TUNA_STATE.write();

    if let Some(tuneable) = tuna.get_mut(category).and_then(|group| group.get_mut(name)) {
        T::update(tuneable, value);
    }
}

pub fn reset<T: AsTuneable>(category: &str, name: &str) {
    let mut tuna = TUNA_STATE.write();

    if let Some(tuneable) = tuna.get_mut(category).and_then(|group| group.get_mut(name)) {
        T::reset(tuneable);
    }
}
