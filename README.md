[![MIT License](http://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE.md)

# lambert-rust
lambert-rust is a crate helping to convert Lambert coordinates to WGS84.

# Usage

```rust
    extern crate lambert;
    let mut loc= lambert::point::Point::new(668832.5384, 6950138.7285,lambert::zone::Zone::Lambert93);

    println!("WGS84 Lat:{}, Lon:{}", loc.y, loc.x);
```
