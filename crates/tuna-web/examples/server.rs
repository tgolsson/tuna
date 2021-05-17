// Author: Tom Solberg <me@sbg.dev>
// Copyright © 2021, Tom Solberg, all rights reserved.
// Created: 14 May 2021

/*!

*/

use tuna::*;

const FLOAT_VALUE1: Float32 = Float32::new("float", "name1", 1.0, Some(0.0), Some(1.0));
const FLOAT_VALUE2: Float32 = Float32::new("float", "name2", 1.0, None, Some(1.0));
const FLOAT_VALUE3: Float32 = Float32::new("float", "name3", 1.0, Some(0.0), None);

const FLOAT_64_VALUE1: Float64 = Float64::new("float", "name_64_1", 1.0, Some(0.0), Some(1.0));
const FLOAT_64_VALUE2: Float64 = Float64::new("float", "name_64_2", 1.0, None, Some(1.0));
const FLOAT_64_VALUE3: Float64 = Float64::new("float", "name_64_3", 1.0, Some(0.0), None);

const INT_VALUE1: Int32 = Int32::new("int", "name20", 20, Some(0), Some(20));
const INT_VALUE2: Int32 = Int32::new("int", "name2", 20, None, Some(20));
const INT_VALUE3: Int32 = Int32::new("int", "name3", 20, Some(0), None);

const INT_64_VALUE1: Int64 = Int64::new("int", "name_64_20", 20, Some(0), Some(20));
const INT_64_VALUE2: Int64 = Int64::new("int", "name_64_2", 20, None, Some(20));
const INT_64_VALUE3: Int64 = Int64::new("int", "name_64_3", 20, Some(0), None);

const BOOL_VALUE1: Boolean = Boolean::new("bool", "name1", true);
const BOOL_VALUE2: Boolean = Boolean::new("bool", "name2", true);
const BOOL_VALUE3: Boolean = Boolean::new("bool", "name3", false);

fn main() {
    FLOAT_VALUE1.register();
    FLOAT_VALUE2.register();
    FLOAT_VALUE3.register();

    FLOAT_64_VALUE1.register();
    FLOAT_64_VALUE2.register();
    FLOAT_64_VALUE3.register();

    INT_VALUE1.register();
    INT_VALUE2.register();
    INT_VALUE3.register();

    INT_64_VALUE1.register();
    INT_64_VALUE2.register();
    INT_64_VALUE3.register();

    BOOL_VALUE1.register();
    BOOL_VALUE2.register();
    BOOL_VALUE3.register();

    env_logger::init();
    let mut server = tuna_web::TunaServer::new(4450).unwrap();

    loop {
        server.loop_once();
    }
}
