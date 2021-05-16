// Author: Tom Solberg <me@sbg.dev>
// Copyright © 2021, Tom Solberg, all rights reserved.
// Created: 14 May 2021

/*!

*/

use nanoserde::{DeJson, SerJson};

/// The definition of a float variable
#[derive(Copy, Clone, Debug)]
pub struct Float32 {
    pub(crate) category: &'static str,
    pub(crate) name: &'static str,
    pub(crate) default: f32,

    pub(crate) min: Option<f32>,
    pub(crate) max: Option<f32>,
}

/// The state of a boolean variable
#[derive(Clone, Debug, SerJson, DeJson)]
pub struct Float32Variable {
    pub(crate) default: f32,

    pub(crate) min: Option<f32>,
    pub(crate) max: Option<f32>,
    pub(crate) current: f32,
}

impl Float32 {
    /// Define a new boolean variable that can be registered with tuna
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

    /// Explicitly register the boolean with tuna. This is not required, but
    /// it'll reduce risk of stuttering when variables get registered.
    pub fn register(&self) {
        crate::register(self.category, self.name, self)
    }

    /// Read the variable from tuna. If the feature `auto-register` is enabled,
    /// will register on a lookup miss - otherwise it'll just return the default
    /// value.
    pub fn read(&self) -> Option<f32> {
        crate::get::<Float32>(self.category, self.name).or_else(|| {
            #[cfg(feature = "auto-register")]
            self.register();
            Some(self.default)
        })
    }

    /// Update the stored value. Will do nothing if not registered.
    pub fn write(&self, value: f32) {
        crate::set::<Float32>(self.category, self.name, value);
    }

    /// Reset to the default value.
    pub fn reset(&self) {
        crate::reset::<Float32>(self.category, self.name);
    }
}

#[cfg(test)]
mod tests {
    use super::Float32;

    use serial_test::serial;

    const TEST_VALUE1: Float32 = Float32::new("float", "name1", 0.1, Some(0.0), Some(1.0));
    const TEST_VALUE2: Float32 = Float32::new("float", "name2", 0.2, None, Some(1.0));
    const TEST_VALUE3: Float32 = Float32::new("float", "name3", 0.3, Some(0.0), None);
    const TEST_VALUE4: Float32 = Float32::new("float", "name4", 0.4, None, None);

    #[test]
    #[serial]
    fn default() {
        TEST_VALUE1.register();
        TEST_VALUE2.register();
        TEST_VALUE3.register();
        TEST_VALUE4.register();
    }

    #[test]
    #[serial]
    fn get() {
        TEST_VALUE1.reset();
        TEST_VALUE2.reset();
        TEST_VALUE3.reset();
        TEST_VALUE4.reset();
        assert_eq!(TEST_VALUE1.read(), Some(0.1));
        assert_eq!(TEST_VALUE2.read(), Some(0.2));
        assert_eq!(TEST_VALUE3.read(), Some(0.3));
        assert_eq!(TEST_VALUE4.read(), Some(0.4));
    }

    #[test]
    #[serial]
    fn set_high() {
        TEST_VALUE1.write(2.0);
        TEST_VALUE2.write(2.0);
        TEST_VALUE3.write(2.0);
        TEST_VALUE4.write(2.0);

        assert_eq!(TEST_VALUE1.read(), Some(1.0));
        assert_eq!(TEST_VALUE2.read(), Some(1.0));
        assert_eq!(TEST_VALUE3.read(), Some(2.0));
        assert_eq!(TEST_VALUE4.read(), Some(2.0));
    }

    #[test]
    #[serial]
    fn set_low() {
        TEST_VALUE1.write(-2.0);
        TEST_VALUE2.write(-2.0);
        TEST_VALUE3.write(-2.0);
        TEST_VALUE4.write(-2.0);

        assert_eq!(TEST_VALUE1.read(), Some(0.0));
        assert_eq!(TEST_VALUE2.read(), Some(-2.0));
        assert_eq!(TEST_VALUE3.read(), Some(0.0));
        assert_eq!(TEST_VALUE4.read(), Some(-2.0));
    }

    #[test]
    #[serial]
    fn reset() {
        TEST_VALUE1.reset();
        TEST_VALUE2.reset();
        TEST_VALUE3.reset();
        TEST_VALUE4.reset();

        assert_eq!(TEST_VALUE1.read(), Some(0.1));
        assert_eq!(TEST_VALUE2.read(), Some(0.2));
        assert_eq!(TEST_VALUE3.read(), Some(0.3));
        assert_eq!(TEST_VALUE4.read(), Some(0.4));
    }
}
