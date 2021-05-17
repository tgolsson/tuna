// Author: Tom Solberg <me@sbg.dev>
// Copyright © 2021, Tom Solberg, all rights reserved.
// Created: 15 May 2021

/*!

*/

use nanoserde::{DeJson, SerJson};

/// The definition of a boolean variable
#[derive(Copy, Clone, Debug)]
pub struct Boolean {
    pub(crate) category: &'static str,
    pub(crate) name: &'static str,
    pub(crate) default: bool,
}

/// The state of a boolean variablep
#[derive(Copy, Clone, Debug, SerJson, DeJson)]
pub struct BooleanVariable {
    pub default: bool,
    pub current: bool,
}

impl Boolean {
    /// Define a new boolean variable that can be registered with tuna
    pub const fn new(category: &'static str, name: &'static str, default: bool) -> Self {
        Self {
            category,
            name,
            default,
        }
    }

    /// Explicitly register the boolean with tuna. This is not required, but
    /// it'll reduce risk of stuttering when variables get registered.
    pub fn register(&self) {
        crate::register(self.category, self.name, self)
    }

    /// Read the variable from tuna. This will automatically call register on a
    /// lookup miss, and return the default value.
    pub fn read(&self) -> bool {
        crate::get::<Boolean>(self.category, self.name).unwrap_or_else(|| {
            self.register();
            self.default
        })
    }

    /// Update the stored value. Will register and warn if not registered already in debug builds.
    pub fn write(&self, value: bool) {
        #[cfg(debug_assertions)]
        if !crate::is_registered(self.category, self.name) {
            log::warn!("Setting unregistered value {}.{}", self.category, self.name);
            self.register();
        }

        crate::set::<Boolean>(self.category, self.name, value);
    }

    /// Reset to the default value.
    pub fn reset(&self) {
        crate::reset::<Boolean>(self.category, self.name);
    }
}

#[cfg(test)]
mod tests {
    use super::Boolean;
    use serial_test::serial;

    const TEST_VALUE1: Boolean = Boolean::new("bool", "name1", true);
    const TEST_VALUE2: Boolean = Boolean::new("bool", "name2", false);

    #[test]
    #[serial]
    fn default() {
        TEST_VALUE1.register();
        TEST_VALUE2.register();
    }

    #[test]
    #[serial]
    fn get() {
        TEST_VALUE1.reset();
        TEST_VALUE2.reset();
        assert_eq!(TEST_VALUE1.read(), true);
        assert_eq!(TEST_VALUE2.read(), false);
    }

    #[test]
    #[serial]
    fn set() {
        TEST_VALUE1.write(false);
        assert_eq!(TEST_VALUE1.read(), false);

        TEST_VALUE2.write(true);
        assert_eq!(TEST_VALUE2.read(), true);
    }

    #[test]
    #[serial]
    fn reset() {
        TEST_VALUE1.reset();
        TEST_VALUE2.reset();
        assert_eq!(TEST_VALUE1.read(), true);
        assert_eq!(TEST_VALUE2.read(), false);
    }
}
