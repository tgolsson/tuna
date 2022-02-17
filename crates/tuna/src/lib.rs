#[doc = include_str!("../README.md")]
use parking_lot::RwLock;

use nanoserde::{DeJson, SerJson};
use std::collections::HashMap;

mod api;
mod boolean;
mod float;
mod int;

pub use tuna_macros::tuna;

pub type TunaState = HashMap<String, HashMap<String, Tuneable>>;

lazy_static::lazy_static! {
    #[doc(hidden)]
    pub static ref TUNA_STATE: RwLock<TunaState> = RwLock::new(Default::default());
}

#[derive(Debug, SerJson, DeJson, Clone)]
pub enum Tuneable {
    Float32(Float32Variable),
    Float64(Float64Variable),
    Int32(Int32Variable),
    Int64(Int64Variable),
    Uint(u32),
    Size(usize),
    Boolean(BooleanVariable),
}

impl Tuneable {
    pub fn apply_to(&self, category: &str, name: &str) {
        match self {
            Self::Float32(v) => api::set::<Float32>(category, name, v.current),
            Self::Float64(v) => api::set::<Float64>(category, name, v.current),
            Self::Boolean(v) => api::set::<Boolean>(category, name, v.current),
            Self::Int32(v) => api::set::<Int32>(category, name, v.current),
            Self::Int64(v) => api::set::<Int64>(category, name, v.current),
            _ => unreachable!(),
        };
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

            fn update(tuneable: &mut Tuneable, var: $res) -> bool {
                match tuneable {
                    Tuneable::$typ(self_) => {
                        self_.current = var;
                        true
                    }
                    _ => false,
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
        impl $typ {
            /// Define a new float variable that can be registered with tuna
            pub const fn new(
                category: &'static str,
                name: &'static str,
                default: $res,
                min: Option<$res>,
                max: Option<$res>,
            ) -> Self {
                Self {
                    category,
                    name,
                    default,
                    min,
                    max,
                }
            }

            /// Explicitly register the float with tuna. This is not required, but
            /// it'll reduce risk of stuttering when variables get registered.
            pub fn register(&self) {
                crate::register(self.category, self.name, self)
            }

            /// Read the variable from tuna. This will automatically call register on a
            /// lookup miss, and return the default value.
            pub fn read(&self) -> $res {
                crate::get::<$typ>(self.category, self.name).unwrap_or_else(|| {
                    self.register();
                    self.default
                })
            }

            /// Update the stored value. Will do nothing if not registered.
            pub fn write(&self, value: $res) {
                #[cfg(debug_assertions)]
                if !crate::is_registered(self.category, self.name) {
                    log::warn!("Setting unregistered value {}.{}", self.category, self.name);
                    self.register();
                }

                crate::set::<$typ>(self.category, self.name, value);
            }

            /// Reset to the default value.
            pub fn reset(&self) {
                crate::reset::<$typ>(self.category, self.name);
            }
        }

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

            fn update(tuneable: &mut Tuneable, var: $res) -> bool {
                match tuneable {
                    Tuneable::$typ(self_) => {
                        let var = if let Some(min) = self_.min {
                            var.max(min)
                        } else {
                            var
                        };

                        let var = if let Some(max) = self_.max {
                            var.min(max)
                        } else {
                            var
                        };
                        self_.current = var;
                        true
                    }
                    _ => false,
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

pub use crate::float::{Float32, Float64};
use float::{Float32Variable, Float64Variable};
impl_tuneable!(Float32, Float32Variable, f32);
impl_tuneable!(Float64, Float64Variable, f64);

pub use crate::int::{Int32, Int64};
use int::{Int32Variable, Int64Variable};
impl_tuneable!(Int32, Int32Variable, i32);
impl_tuneable!(Int64, Int64Variable, i64);

pub use crate::boolean::Boolean;
use boolean::BooleanVariable;
impl_tuneable_simple!(Boolean, BooleanVariable, bool);

pub use crate::api::*;
