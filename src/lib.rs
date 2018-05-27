#[macro_use]
pub mod point;
pub mod zone;
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
                    .convert_wgs84(Zone::Lambert93)
                    .convert_unit(AngleUnit::Radian, AngleUnit::Degree);
        println!("WGS84 Lat:{}, Lon:{}", point.y, point.x);
    }
}