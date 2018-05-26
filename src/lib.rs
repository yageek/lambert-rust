#[macro_use]
mod point;
mod zone;
mod consts;
mod algo;
mod prelude;

#[cfg(test)]
mod tests {
    use super::prelude::*;
    #[test]
    fn test_usage() {
        let mut point = Point::new(369419.0,1986498.0,0.0,AngleUnit::Meter);
        point.convert_wgs84(Zone::Lambert93);
 
        println!("WGS84 Lat:{}, Lon:{}", point.y, point.x);
    }
}