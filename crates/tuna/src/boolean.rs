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
        assert_eq!(TEST_VALUE1.read(), Some(true));
        assert_eq!(TEST_VALUE2.read(), Some(false));
    }

    #[test]
    #[serial]
    fn set() {
        TEST_VALUE1.write(false);
        TEST_VALUE2.write(true);
        assert_eq!(TEST_VALUE1.read(), Some(false));
        assert_eq!(TEST_VALUE2.read(), Some(true));
    }

    #[test]
    #[serial]
    fn reset() {
        TEST_VALUE1.reset();
        TEST_VALUE2.reset();
        assert_eq!(TEST_VALUE1.read(), Some(true));
        assert_eq!(TEST_VALUE2.read(), Some(false));
    }
}
