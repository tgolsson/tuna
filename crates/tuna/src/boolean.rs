// Author: Tom Solberg <me@sbg.dev>
// Copyright © 2021, Tom Solberg, all rights reserved.
// Created: 15 May 2021

/*!

*/

use nanoserde::{DeJson, SerJson};

#[derive(Copy, Clone, Debug)]
pub struct Boolean {
    pub(crate) category: &'static str,
    pub(crate) name: &'static str,
    pub(crate) default: bool,
}

#[derive(Copy, Clone, Debug, SerJson, DeJson)]
pub struct BooleanVariable {
    pub default: bool,
    pub current: bool,
}

impl Boolean {
    pub const fn new(category: &'static str, name: &'static str, default: bool) -> Self {
        Self {
            category,
            name,
            default,
        }
    }

    pub fn register(&self) {
        crate::register(self.category, self.name, self)
    }

    pub fn read(&self) -> Option<bool> {
        crate::get::<Boolean>(self.category, self.name).or_else(|| {
            self.register();
            Some(self.default)
        })
    }

    pub fn write(&self, value: bool) {
        crate::set::<Boolean>(self.category, self.name, value);
    }

    pub fn reset(&self) {
        crate::reset::<Boolean>(self.category, self.name);
    }
}

#[cfg(test)]
mod tests {
    use crate::tuna::{self, Boolean};
    use serial_test::serial;

    const TEST_VALUE1: Boolean = Boolean::new("bool", "name1", true);
    const TEST_VALUE2: Boolean = Boolean::new("bool", "name2", true);
    const TEST_VALUE3: Boolean = Boolean::new("bool", "name3", false);

    #[test]
    #[serial]
    fn test_default() {
        TEST_VALUE1.register();
    }

    #[test]
    #[serial]
    fn test_get() {
        TEST_VALUE1.register();
        assert_eq!(TEST_VALUE1.read(), Some(1.0));
    }

    #[test]
    #[serial]
    fn test_set() {
        TEST_VALUE1.write(2.0);
        assert_eq!(TEST_VALUE1.read(), Some(2.0));
    }

    #[test]
    #[serial]
    fn test_reset() {
        TEST_VALUE1.reset();
        assert_eq!(TEST_VALUE1.read(), Some(1.0));
    }
}
