# `tuna` Changelog

## [Unreleased]

* `AsTuneable::update` and `tuna::set` function now returns a bool if the write succeded
* The `AsTuneable` now requires that the output type implements `Debug`
* `tuna::register` and `set` will now log at `debug` level
* `tuna::read` will log at trace level
* There's now a derive macro to easily create and register multiple variables.


## Version 0.0.3

* Add new types:
  * Int32
  * Int64
  * Float64

* Read now returns T instead of Option<T>, as it'll always use the default value.
* Remove the auto-register feature
* Add `log` dependency
* Add `is_registered` function
