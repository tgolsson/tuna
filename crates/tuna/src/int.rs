// Author: Tom Solberg <tom.olsson@embark-studios.com>
// Copyright © 2021, Tom Solberg, all rights reserved.
// Created: 17 May 2021

/*!

*/

use nanoserde::{DeJson, SerJson};

/// The definition of a float variable
#[derive(Copy, Clone, Debug)]
pub struct Int32 {
    pub(crate) category: &'static str,
    pub(crate) name: &'static str,
    pub(crate) default: i32,

    pub(crate) min: Option<i32>,
    pub(crate) max: Option<i32>,
}

/// The state of a float variable
#[derive(Clone, Debug, SerJson, DeJson)]
pub struct Int32Variable {
    pub(crate) default: i32,

    pub(crate) min: Option<i32>,
    pub(crate) max: Option<i32>,
    pub(crate) current: i32,
}

/// The definition of a float variable
#[derive(Copy, Clone, Debug)]
pub struct Int64 {
    pub(crate) category: &'static str,
    pub(crate) name: &'static str,
    pub(crate) default: i64,

    pub(crate) min: Option<i64>,
    pub(crate) max: Option<i64>,
}

/// The state of a float variable
#[derive(Clone, Debug, SerJson, DeJson)]
pub struct Int64Variable {
    pub(crate) default: i64,

    pub(crate) min: Option<i64>,
    pub(crate) max: Option<i64>,
    pub(crate) current: i64,
}

#[cfg(test)]
mod tests {
    use super::{Int32, Int64};
    use serial_test::serial;

    const TEST_INT321: Int32 = Int32::new("int", "int32_1", 1, Some(0), Some(5));
    const TEST_INT322: Int32 = Int32::new("int", "int32_2", 2, None, Some(5));
    const TEST_INT323: Int32 = Int32::new("int", "int32_3", 3, Some(0), None);
    const TEST_INT324: Int32 = Int32::new("int", "int32_4", 4, None, None);

    #[test]
    #[serial]
    fn default_32() {
        TEST_INT321.register();
        TEST_INT322.register();
        TEST_INT323.register();
        TEST_INT324.register();
    }

    #[test]
    #[serial]
    fn get_32() {
        TEST_INT321.reset();
        TEST_INT322.reset();
        TEST_INT323.reset();
        TEST_INT324.reset();

        assert_eq!(TEST_INT321.read(), 1);
        assert_eq!(TEST_INT322.read(), 2);
        assert_eq!(TEST_INT323.read(), 3);
        assert_eq!(TEST_INT324.read(), 4);
    }

    #[test]
    #[serial]
    fn set_high_32() {
        TEST_INT321.write(10);
        assert_eq!(TEST_INT321.read(), 5);

        TEST_INT322.write(10);
        assert_eq!(TEST_INT322.read(), 5);

        TEST_INT323.write(10);
        assert_eq!(TEST_INT323.read(), 10);

        TEST_INT324.write(10);
        assert_eq!(TEST_INT324.read(), 10);
    }

    #[test]
    #[serial]
    fn set_low_32() {
        TEST_INT321.write(-10);
        TEST_INT322.write(-10);
        TEST_INT323.write(-10);
        TEST_INT324.write(-10);

        assert_eq!(TEST_INT321.read(), 0);
        assert_eq!(TEST_INT322.read(), -10);
        assert_eq!(TEST_INT323.read(), 0);
        assert_eq!(TEST_INT324.read(), -10);
    }

    #[test]
    #[serial]
    fn reset_32() {
        TEST_INT321.reset();
        TEST_INT322.reset();
        TEST_INT323.reset();
        TEST_INT324.reset();

        assert_eq!(TEST_INT321.read(), 1);
        assert_eq!(TEST_INT322.read(), 2);
        assert_eq!(TEST_INT323.read(), 3);
        assert_eq!(TEST_INT324.read(), 4);
    }

    const TEST_INT641: Int64 = Int64::new("int", "int64_1", 1, Some(0), Some(5));
    const TEST_INT642: Int64 = Int64::new("int", "int64_2", 2, None, Some(5));
    const TEST_INT643: Int64 = Int64::new("int", "int64_3", 3, Some(0), None);
    const TEST_INT644: Int64 = Int64::new("int", "int64_4", 4, None, None);

    #[test]
    #[serial]
    fn default_64() {
        TEST_INT641.register();
        TEST_INT642.register();
        TEST_INT643.register();
        TEST_INT644.register();
    }

    #[test]
    #[serial]
    fn get_64() {
        TEST_INT641.reset();
        TEST_INT642.reset();
        TEST_INT643.reset();
        TEST_INT644.reset();

        assert_eq!(TEST_INT641.read(), 1);
        assert_eq!(TEST_INT642.read(), 2);
        assert_eq!(TEST_INT643.read(), 3);
        assert_eq!(TEST_INT644.read(), 4);
    }

    #[test]
    #[serial]
    fn set_high_64() {
        TEST_INT641.write(10);
        TEST_INT642.write(10);
        TEST_INT643.write(10);
        TEST_INT644.write(10);

        assert_eq!(TEST_INT641.read(), 5);
        assert_eq!(TEST_INT642.read(), 5);
        assert_eq!(TEST_INT643.read(), 10);
        assert_eq!(TEST_INT644.read(), 10);
    }

    #[test]
    #[serial]
    fn set_low_64() {
        TEST_INT641.write(-10);
        TEST_INT642.write(-10);
        TEST_INT643.write(-10);
        TEST_INT644.write(-10);

        assert_eq!(TEST_INT641.read(), 0);
        assert_eq!(TEST_INT642.read(), -10);
        assert_eq!(TEST_INT643.read(), 0);
        assert_eq!(TEST_INT644.read(), -10);
    }

    #[test]
    #[serial]
    fn reset_64() {
        TEST_INT641.reset();
        TEST_INT642.reset();
        TEST_INT643.reset();
        TEST_INT644.reset();

        assert_eq!(TEST_INT641.read(), 1);
        assert_eq!(TEST_INT642.read(), 2);
        assert_eq!(TEST_INT643.read(), 3);
        assert_eq!(TEST_INT644.read(), 4);
    }
}
