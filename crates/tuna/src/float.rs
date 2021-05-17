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

/// The state of a float variable
#[derive(Clone, Debug, SerJson, DeJson)]
pub struct Float32Variable {
    pub(crate) default: f32,

    pub(crate) min: Option<f32>,
    pub(crate) max: Option<f32>,
    pub(crate) current: f32,
}

/// The definition of a float variable
#[derive(Copy, Clone, Debug)]
pub struct Float64 {
    pub(crate) category: &'static str,
    pub(crate) name: &'static str,
    pub(crate) default: f64,

    pub(crate) min: Option<f64>,
    pub(crate) max: Option<f64>,
}

/// The state of a float variable
#[derive(Clone, Debug, SerJson, DeJson)]
pub struct Float64Variable {
    pub(crate) default: f64,

    pub(crate) min: Option<f64>,
    pub(crate) max: Option<f64>,
    pub(crate) current: f64,
}

#[cfg(test)]
mod tests {
    use super::{Float32, Float64};

    use serial_test::serial;

    const TEST_FLOAT32_1: Float32 = Float32::new("float", "float32_1", 0.1, Some(0.0), Some(1.0));
    const TEST_FLOAT32_2: Float32 = Float32::new("float", "float32_2", 0.2, None, Some(1.0));
    const TEST_FLOAT32_3: Float32 = Float32::new("float", "float32_3", 0.3, Some(0.0), None);
    const TEST_FLOAT32_4: Float32 = Float32::new("float", "float32_4", 0.4, None, None);

    #[test]
    #[serial]
    fn default_32() {
        TEST_FLOAT32_1.register();
        TEST_FLOAT32_2.register();
        TEST_FLOAT32_3.register();
        TEST_FLOAT32_4.register();
    }

    #[test]
    #[serial]
    fn get_32() {
        TEST_FLOAT32_1.reset();
        TEST_FLOAT32_2.reset();
        TEST_FLOAT32_3.reset();
        TEST_FLOAT32_4.reset();
        assert_eq!(TEST_FLOAT32_1.read(), 0.1);
        assert_eq!(TEST_FLOAT32_2.read(), 0.2);
        assert_eq!(TEST_FLOAT32_3.read(), 0.3);
        assert_eq!(TEST_FLOAT32_4.read(), 0.4);
    }

    #[test]
    #[serial]
    fn set_high_32() {
        TEST_FLOAT32_1.write(2.0);
        TEST_FLOAT32_2.write(2.0);
        TEST_FLOAT32_3.write(2.0);
        TEST_FLOAT32_4.write(2.0);

        assert_eq!(TEST_FLOAT32_1.read(), 1.0);
        assert_eq!(TEST_FLOAT32_2.read(), 1.0);
        assert_eq!(TEST_FLOAT32_3.read(), 2.0);
        assert_eq!(TEST_FLOAT32_4.read(), 2.0);
    }

    #[test]
    #[serial]
    fn set_low_32() {
        TEST_FLOAT32_1.write(-2.0);
        TEST_FLOAT32_2.write(-2.0);
        TEST_FLOAT32_3.write(-2.0);
        TEST_FLOAT32_4.write(-2.0);

        assert_eq!(TEST_FLOAT32_1.read(), 0.0);
        assert_eq!(TEST_FLOAT32_2.read(), -2.0);
        assert_eq!(TEST_FLOAT32_3.read(), 0.0);
        assert_eq!(TEST_FLOAT32_4.read(), -2.0);
    }

    #[test]
    #[serial]
    fn reset_32() {
        TEST_FLOAT32_1.reset();
        TEST_FLOAT32_2.reset();
        TEST_FLOAT32_3.reset();
        TEST_FLOAT32_4.reset();

        assert_eq!(TEST_FLOAT32_1.read(), 0.1);
        assert_eq!(TEST_FLOAT32_2.read(), 0.2);
        assert_eq!(TEST_FLOAT32_3.read(), 0.3);
        assert_eq!(TEST_FLOAT32_4.read(), 0.4);
    }

    const TEST_FLOAT64_1: Float64 = Float64::new("float", "float64_1", 0.1, Some(0.0), Some(1.0));
    const TEST_FLOAT64_2: Float64 = Float64::new("float", "float64_2", 0.2, None, Some(1.0));
    const TEST_FLOAT64_3: Float64 = Float64::new("float", "float64_3", 0.3, Some(0.0), None);
    const TEST_FLOAT64_4: Float64 = Float64::new("float", "float64_4", 0.4, None, None);

    #[test]
    #[serial]
    fn default_64() {
        TEST_FLOAT64_1.register();
        TEST_FLOAT64_2.register();
        TEST_FLOAT64_3.register();
        TEST_FLOAT64_4.register();
    }

    #[test]
    #[serial]
    fn get_64() {
        TEST_FLOAT64_1.reset();
        TEST_FLOAT64_2.reset();
        TEST_FLOAT64_3.reset();
        TEST_FLOAT64_4.reset();
        assert_eq!(TEST_FLOAT64_1.read(), 0.1);
        assert_eq!(TEST_FLOAT64_2.read(), 0.2);
        assert_eq!(TEST_FLOAT64_3.read(), 0.3);
        assert_eq!(TEST_FLOAT64_4.read(), 0.4);
    }

    #[test]
    #[serial]
    fn set_high_64() {
        TEST_FLOAT64_1.write(2.0);
        TEST_FLOAT64_2.write(2.0);
        TEST_FLOAT64_3.write(2.0);
        TEST_FLOAT64_4.write(2.0);

        assert_eq!(TEST_FLOAT64_1.read(), 1.0);
        assert_eq!(TEST_FLOAT64_2.read(), 1.0);
        assert_eq!(TEST_FLOAT64_3.read(), 2.0);
        assert_eq!(TEST_FLOAT64_4.read(), 2.0);
    }

    #[test]
    #[serial]
    fn set_low_64() {
        TEST_FLOAT64_1.write(-2.0);
        TEST_FLOAT64_2.write(-2.0);
        TEST_FLOAT64_3.write(-2.0);
        TEST_FLOAT64_4.write(-2.0);

        assert_eq!(TEST_FLOAT64_1.read(), 0.0);
        assert_eq!(TEST_FLOAT64_2.read(), -2.0);
        assert_eq!(TEST_FLOAT64_3.read(), 0.0);
        assert_eq!(TEST_FLOAT64_4.read(), -2.0);
    }

    #[test]
    #[serial]
    fn reset_64() {
        TEST_FLOAT64_1.reset();
        TEST_FLOAT64_2.reset();
        TEST_FLOAT64_3.reset();
        TEST_FLOAT64_4.reset();

        assert_eq!(TEST_FLOAT64_1.read(), 0.1);
        assert_eq!(TEST_FLOAT64_2.read(), 0.2);
        assert_eq!(TEST_FLOAT64_3.read(), 0.3);
        assert_eq!(TEST_FLOAT64_4.read(), 0.4);
    }
}
