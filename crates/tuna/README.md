# tuna

Tuna is a tool for managing CVARs during game development, as a set of global
global variables, on which manipulation can be built.

At the core, the goal of tuna is to be *easy* to use, while avoiding unsafe
code.

This is how you use it:

``` rust
extern crate tuna;

const ENABLE_LOGGING: tuna::Boolean = tuna::Boolean::new("logging", "enable", false);

fn main() {
    ENABLE_LOGGING.register();

    for i in 0..10 {
	    eprintln!("xx");
        if (ENABLE_LOGGING.read()) {
            eprintln!("looping once");
        }
		if i == 5 {
		    ENABLE_LOGGING.write(true);
		}
    }
}
```

The register call can be omitted, at some performance cost during the first
read.

There's also a utility macro to create a category more easily:

``` rust
extern crate tuna;

#[tuna::tuna]
mod logging {
	pub(super) const ENABLE: bool = false;
}

fn main() {
    logging::register();

    for i in 0..10 {
        if (logging::ENABLE.read()) {
            eprintln!("looping once");
        }

		if i == 5 {
		    logging::ENABLE.write(true);
		}
    }
}
```

Note that `tuna` is a work in progress! I'm working on it due to a need, but I
want to dogfood it while I build it - not build a whole thing on its own.

## Alternatives:

* [cvar](https://crates.io/crates/cvar) - much more customizable, less batteries-included
