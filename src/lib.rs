#[macro_use]
pub mod point;
pub mod zone;
mod consts;
mod algo;

#[cfg(test)]
mod tests {
    use super::point::Point;
    use super::zone::Zone;
    #[test]
    fn test_usage() {
        let point = Point::new(369419.0, 1986498.0, 0.0)
                    .convert_wgs84(Zone::Lambert93);
        println!("WGS84 Lat:{}, Lon:{}", point.y, point.x);
    }
}