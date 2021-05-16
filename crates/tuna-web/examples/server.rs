// Author: Tom Solberg <me@sbg.dev>
// Copyright © 2021, Tom Solberg, all rights reserved.
// Created: 14 May 2021

/*!

*/

use tuna::*;
use tuna_web::*;

const FLOAT_VALUE1: Float32 = Float32::new("float", "name1", 1.0, Some(0.0), Some(1.0));
const FLOAT_VALUE2: Float32 = Float32::new("float", "name2", 1.0, None, Some(1.0));
const FLOAT_VALUE3: Float32 = Float32::new("float", "name3", 1.0, Some(0.0), None);

const BOOL_VALUE1: Boolean = Boolean::new("bool", "name1", true);
const BOOL_VALUE2: Boolean = Boolean::new("bool", "name2", true);
const BOOL_VALUE3: Boolean = Boolean::new("bool", "name3", false);

fn main() {
    FLOAT_VALUE1.register();
    FLOAT_VALUE2.register();
    FLOAT_VALUE3.register();

    BOOL_VALUE1.register();
    BOOL_VALUE2.register();
    BOOL_VALUE3.register();

    env_logger::init();
    let mut server = tuna_web::TunaServer::new(4450).unwrap();

    loop {
        server.loop_once();
    }
}
