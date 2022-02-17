// Author: Tom Solberg <me@sbg.dev>
// Copyright © 2022, Tom Solberg, all rights reserved.
// Created: 14 February 2022

/*!

*/

#![allow(unused)]

#[test]
fn test_derive_i32() {
    #[tuna_macros::tuna]
    mod test {
        #[min = 0]
        #[max = 20]
        const FOO: i32 = 10;
    }
}

#[test]
fn test_derive_i64() {
    #[tuna_macros::tuna]
    mod test {
        #[min = 0]
        #[max = 20]
        const FOO: i64 = 10;
    }
}

#[test]
fn test_derive_f32() {
    #[tuna_macros::tuna]
    mod test {
        #[min = 0.0]
        #[max = 20.0]
        const FOO: f32 = 10.0;
    }
}

#[test]
fn test_derive_f64() {
    #[tuna_macros::tuna]
    mod test {
        #[min = 0.0]
        #[max = 20.0]
        const FOO: f64 = 10.0;
    }
}

#[test]
fn test_derive_bool() {
    #[tuna_macros::tuna]
    mod test {
        const FOO: bool = false;
    }
}
