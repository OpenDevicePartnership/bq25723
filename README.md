# BQ25723 Rust Device Driver

A `#[no_std]` platform-agnostic driver for the [BQ25723](https://www.ti.com/lit/ds/symlink/bq25723.pdf) buck-boost battery charge controller, capable of charging a 2- to 5-cell battery, using the [embedded-hal](https://docs.rs/embedded-hal) traits.

A higher level API will be built on top of the lower level register accessor using the [embedded-batteries](https://github.com/OpenDevicePartnership/embedded-batteries) traits.


## MSRV

Currently, rust `1.85` and up is supported.

## License

Licensed under the terms of the [MIT license](http://opensource.org/licenses/MIT).

## Contribution

Unless you explicitly state otherwise, any contribution submitted for
inclusion in the work by you shall be licensed under the terms of the
MIT license.

License: MIT
