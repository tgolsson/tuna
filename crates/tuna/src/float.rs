// Author: Tom Solberg <me@sbg.dev>
// Copyright © 2021, Tom Solberg, all rights reserved.
// Created: 14 May 2021

/*!

*/

use nanoserde::{DeJson, SerJson};

#[derive(Copy, Clone, Debug)]
pub struct Float32 {
    pub(crate) category: &'static str,
    pub(crate) name: &'static str,
    pub(crate) default: f32,

    pub(crate) min: Option<f32>,
    pub(crate) max: Option<f32>,
}

#[derive(Clone, Debug, SerJson, DeJson)]
pub struct Float32Variable {
    pub(crate) default: f32,

    pub(crate) min: Option<f32>,
    pub(crate) max: Option<f32>,
    pub(crate) current: f32,
}

impl Float32 {
    pub const fn new(
        category: &'static str,
        name: &'static str,
        default: f32,
        min: Option<f32>,
        max: Option<f32>,
    ) -> Self {
        Self {
            category,
            name,
            default,
            min,
            max,
        }
    }

    pub fn register(&self) {
        crate::register(self.category, self.name, self)
    }

    pub fn read(&self) -> Option<f32> {
        crate::get::<Float32>(self.category, self.name).or_else(|| {
            self.register();
            Some(self.default)
        })
    }

    pub fn write(&self, value: f32) {
        crate::set::<Float32>(self.category, self.name, value);
    }

    pub fn reset(&self) {
        crate::reset::<Float32>(self.category, self.name);
    }
}

#[cfg(test)]
mod tests {
    use crate::tuna::{self, Float32};

    use serial_test::serial;

    const TEST_VALUE1: Float32 = Float32::new("float", "name1", 1.0, Some(0.0), Some(1.0));
    const TEST_VALUE2: Float32 = Float32::new("float", "name2", 1.0, None, Some(1.0));
    const TEST_VALUE3: Float32 = Float32::new("float", "name3", 1.0, Some(0.0), None);

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
