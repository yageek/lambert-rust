//! Lambert to WGS84 projection conversion crate
//! 
//! # Examples
//! 
//! ```rust
//! extern crate lambert;
//! use lambert::{Point, AngleUnit, Zone};
//!    
//! // Enter coordinates in point
//! let point = Point::new(369419.0, 1986498.0, 0.0)
//!             .wgs84_from_meter(Zone::Lambert93)
//!             .convert_unit(AngleUnit::Radian, AngleUnit::Degree);
//!                
//! println!("WGS84 Lat:{}, Lon:{}", point.y, point.x);
//! ```

#[macro_use]
mod point;
mod zone;
mod consts;
mod algo;

pub use point::{Point, AngleUnit};
pub use zone::Zone;

#[cfg(test)]
mod tests {
    use super::{Point, AngleUnit, Zone};
    #[test]
    fn test_usage() {
      let point = Point::new(369419.0, 1986498.0, 0.0)
                    .wgs84_from_meter(Zone::Lambert93)
                    .convert_unit(AngleUnit::Radian, AngleUnit::Degree);
        println!("WGS84 Lat:{}, Lon:{}", point.y, point.x);
    }
}