use parking_lot::RwLock;

use nanoserde::{DeJson, SerJson};
use std::collections::HashMap;

mod api;
mod boolean;
mod float;

pub type TunaState = HashMap<String, HashMap<String, Tuneable>>;

lazy_static::lazy_static! {
    #[doc(hidden)]
    pub static ref TUNA_STATE: RwLock<TunaState> = RwLock::new(Default::default());
}

#[derive(Debug, SerJson, DeJson, Clone)]
pub enum Tuneable {
    Float32(Float32Variable),
    Float64(f64),
    Int(i32),
    Uint(u32),
    Size(usize),
    Boolean(BooleanVariable),
}

impl Tuneable {
    pub fn apply_to(&self, category: &str, name: &str) {
        match self {
            Self::Float32(v) => api::set::<Float32>(category, name, v.current),
            Self::Boolean(v) => api::set::<Boolean>(category, name, v.current),
            _ => unreachable!(),
        }
    }
}

#[macro_export]
macro_rules! impl_tuneable_simple {
    ($typ:ident, $var:ident, $res:ident) => {
        impl $crate::api::AsTuneable for $typ {
            type Result = $res;

            fn make_tuneable(&self) -> Tuneable {
                let $typ { default, .. } = self;

                let var = $var {
                    default: *default,
                    current: *default,
                };

                Tuneable::$typ(var)
            }

            fn update(tuneable: &mut Tuneable, var: $res) {
                match tuneable {
                    Tuneable::$typ(self_) => self_.current = var,
                    _ => {}
                }
            }

            fn reset(tuneable: &mut Tuneable) {
                match tuneable {
                    Tuneable::$typ(self_) => self_.current = self_.default,
                    _ => {}
                }
            }

            fn from_tuneable(v: &Tuneable) -> Option<$res> {
                match v {
                    Tuneable::$typ(x) => Some(x.current),
                    _ => None,
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_tuneable {
    ($typ:ident, $var:ident, $res:ident) => {
        impl $crate::api::AsTuneable for $typ {
            type Result = $res;

            fn make_tuneable(&self) -> Tuneable {
                let $typ {
                    default, min, max, ..
                } = self;

                let var = $var {
                    // name: (*name).to_owned(),
                    // category: (*category).to_owned(),
                    default: *default,
                    min: *min,
                    max: *max,
                    current: *default,
                };

                Tuneable::$typ(var)
            }

            fn update(tuneable: &mut Tuneable, var: $res) {
                match tuneable {
                    Tuneable::$typ(self_) => self_.current = var,
                    _ => {}
                }
            }

            fn reset(tuneable: &mut Tuneable) {
                match tuneable {
                    Tuneable::$typ(self_) => self_.current = self_.default,
                    _ => {}
                }
            }

            fn from_tuneable(v: &Tuneable) -> Option<$res> {
                match v {
                    Tuneable::$typ(x) => Some(x.current),
                    _ => None,
                }
            }
        }
    };
}

pub use crate::float::Float32;
use float::Float32Variable;
impl_tuneable!(Float32, Float32Variable, f32);

pub use crate::boolean::Boolean;
pub use boolean::BooleanVariable;
impl_tuneable_simple!(Boolean, BooleanVariable, bool);

pub use crate::api::*;
