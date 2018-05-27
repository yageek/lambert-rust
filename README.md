# lambert-rust

[![Crates version](https://img.shields.io/crates/v/lambert.svg?)](https://crates.io/crates/lambert)
[![Build Status](https://travis-ci.org/yageek/lambert-rust.svg?branch=master)](https://travis-ci.org/yageek/lambert-rust)
[![MIT License](http://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE.md)

lambert-rust is a crate helping to convert Lambert coordinates to WGS84.

# Usage

```rust
let point = Point::new(369419.0, 1986498.0, 0.0)
                    .wgs84_from_meter(Zone::Lambert93)
                    .convert_unit(AngleUnit::Radian, AngleUnit::Degree);
println!("WGS84 Lat:{}, Lon:{}", point.y, point.x);
```
