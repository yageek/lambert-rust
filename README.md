# lambert-rust

[![Crates version](https://img.shields.io/crates/v/lambert.svg?)](https://crates.io/crates/lambert)
[![Build Status](https://travis-ci.org/yageek/lambert-rust.svg?branch=master)](https://travis-ci.org/yageek/lambert-rust)
[![MIT License](http://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE.md)

lambert-rust is a crate helping to convert Lambert coordinates to WGS84.

# Usage

```rust
    // Import crate
    extern crate lambert;

    // Import element
    use lambert::prelude::*;
    let mut loc = Point::new(668832.5384, 6950138.7285, 0.0, Zone::Lambert93);
    
    println!("WGS84 Lat:{}, Lon:{}", loc.y, loc.x);
```
