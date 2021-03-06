// Author: Tom Solberg <me@sbg.dev>
// Copyright © 2021, Tom Solberg, all rights reserved.
// Created: 14 May 2021

/*!

*/
use std::any::Any;

use crate::{Tuneable, TUNA_STATE};

/// Implemented by types that can be used to make tuneables; i.e., manipulated state with various constraints
pub trait AsTuneable: Any + Clone + Sized {
    type Result: std::fmt::Debug + Copy;

    fn make_tuneable(&self) -> Tuneable;
    fn update(tuneable: &mut Tuneable, var: Self::Result) -> bool;
    fn reset(tuneable: &mut Tuneable);
    fn from_tuneable(v: &Tuneable) -> Option<Self::Result>;
}

/// Register a tuneable variable with a default variable. If variable
/// already exists, won't do anything.
pub fn register<T: AsTuneable>(category: &str, name: &str, value: &T) {
    let mut tuna = TUNA_STATE.write();
    log::debug!(
        "Registering variable: {}/{} -> {:?}",
        category,
        name,
        std::any::type_name::<T>(),
    );
    if !tuna.contains_key(category) {
        tuna.insert(category.to_owned(), Default::default());
    }

    let group = tuna.get_mut(category).expect("must be inserted above");

    if group.contains_key(name) {
        return;
    }

    group.insert(name.to_owned(), value.make_tuneable());
}

/// Get a the value of tunable variable, if it matches the expected type
pub fn get<T: AsTuneable>(category: &str, name: &str) -> Option<T::Result> {
    let tuna = TUNA_STATE.read();
    let res: Option<T::Result> = tuna
        .get(category)
        .and_then(|group| group.get(name))
        .and_then(|value| T::from_tuneable(value));

    log::trace!("Reading variable {}/{} as {:?}", category, name, res);
    res
}

/// Set a tuneable variable, if it makes the expected type
pub fn set<T: AsTuneable>(category: &str, name: &str, value: T::Result) -> bool {
    let mut tuna = TUNA_STATE.write();

    if let Some(tuneable) = tuna.get_mut(category).and_then(|group| group.get_mut(name)) {
        if T::update(tuneable, value) {
            log::debug!("Setting variable {}/{} to {:?}", category, name, value);
            true
        } else {
            false
        }
    } else {
        false
    }
}

/// Reset the variable to default value
pub fn reset<T: AsTuneable>(category: &str, name: &str) {
    let mut tuna = TUNA_STATE.write();

    if let Some(tuneable) = tuna.get_mut(category).and_then(|group| group.get_mut(name)) {
        T::reset(tuneable);
    }
}

/// Check if the value is registered
pub fn is_registered(category: &str, name: &str) -> bool {
    let mut tuna = TUNA_STATE.write();

    tuna.get_mut(category)
        .and_then(|group| group.get_mut(name))
        .is_some()
}
