
use std;
use zone::Zone;
use consts;
use algo;

macro_rules! assert_delta {
    ( $left:expr, $right:expr, $d:expr ) => {
        {
            if $left > $right {
                if ($left - $right) > $d {
                    panic!("left:{} | right:{} | delta:{}\n", $left, $right, ($left - $right));
                 }
            } else {
                if ($right - $left) > $d {
                    panic!("left:{} | right:{} | delta:{}\n", $left, $right, ($right - $left));
                }
            }
        }
    };
}

/// The `AngleUnit` enum reprensts the unity expressed by
/// by the coordinate.
#[derive(Clone, Copy)]
pub enum AngleUnit {
   Radian,
   Degree,
   Grad,
   Meter,
}

/// `factor` is a private method
/// returning the factor value
/// to pass to one `AngleUnit` to another.
fn factor(from: AngleUnit, to: AngleUnit) -> f32 {
    match (from, to) {
        // Logical conversion
        (AngleUnit::Degree, AngleUnit::Degree) => 1.0,
        (AngleUnit::Radian, AngleUnit::Radian) => 1.0,
        (AngleUnit::Grad, AngleUnit::Grad) => 1.0,

        //Degree <-> Radian
        (AngleUnit::Degree, AngleUnit::Radian) => std::f32::consts::PI / 180.0,
        (AngleUnit::Radian, AngleUnit::Degree) => 180.0 / std::f32::consts::PI ,

        //Grad <-> Radian
        (AngleUnit::Grad, AngleUnit::Radian) => std::f32::consts::PI / 200.0,
        (AngleUnit::Radian, AngleUnit::Grad) => 200.0 / std::f32::consts::PI,

        //Degree <-> Grad
        (AngleUnit::Degree, AngleUnit::Grad) => 200.0/180.0,
        (AngleUnit::Grad, AngleUnit::Degree) => 180.0/200.0,
        _ => 1.0,
    }
}
/// The `Point` struct represents a Point
#[derive(Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Point {
    /// `new` returns a new Point.
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point { x: x, y: y, z: z}
    }
    /// `convertunit` convert the point to the given unit.
    /// Convert to Meter does not transform x,y,z.
    pub fn convert_unit(&self, from: AngleUnit, to: AngleUnit) -> Point {
        self.scale(factor(from, to))
    }

    /// `scale` scale the point
    /// by a factor value.
    fn scale(&self, factor:f32) -> Point {
        Point { x: self.x *factor, y: self.y *factor, z: self.z *factor }
    }

    /// Convert the `MeterPoint` to WGS84
    pub fn convert_wgs84(&self, zone: Zone) -> Point {
       
        let mut pt = Point::new(self.x, self.y, self.z);

        match zone {
            Zone::Lambert93 => pt = algo::lambert_to_geographic(pt, zone, consts::LON_MERID_IERS,consts::E_WGS84,consts::DEFAULT_EPS),
            _ => {
                pt = algo::lambert_to_geographic(pt, zone, consts::LON_MERID_PARIS, consts::E_CLARK_IGN, consts::DEFAULT_EPS);
                pt = algo::geographic_to_cartesian(pt.x, pt.y, pt.z, consts::A_CLARK_IGN, consts::E_CLARK_IGN);

                pt.x -= 168.0;
                pt.y -= 60.0;
                pt.z += 320.0;
                pt = algo::cartesian_to_geographic(pt, consts::LON_MERID_GREENWICH,consts::A_WGS84, consts::E_WGS84, consts::DEFAULT_EPS);
            }
        }
        pt
    }
}

#[test]
fn test_new() {
    let point = Point::new(55.0, 1.0, 0.0);
    assert_eq!(point.x, 55.0);
    assert_eq!(point.y, 1.0);
    assert_eq!(point.z, 0.0);
}

#[test]
fn test_scale() {
    let point = Point::new(55.0, 1.0, 0.0)
                .scale(2.0);
    assert_eq!(point.x, 110.0);
    assert_eq!(point.y, 2.0);
    assert_eq!(point.z, 0.0);
}

#[test]
fn test_convert() {
    let point = Point::new(180.0, 360.0, 0.0)
                .convert_unit(AngleUnit::Degree, AngleUnit::Radian);
    assert_eq!(point.x, std::f32::consts::PI);
    assert_eq!(point.y, 2.0 * std::f32::consts::PI);
    assert_eq!(point.z, 0.0);
}

#[test]
fn test_wgs84_zone_1(){
    let expected_point = Point::new(7.68639475277068, 48.5953456709144, 0.0);
    let point = Point::new(994300.623, 113409.981, 0.0)
                .convert_wgs84(Zone::LambertI)
                .convert_unit(AngleUnit::Meter, AngleUnit::Degree);

    let delta = 1e-3;
    assert_delta!(point.x, expected_point.x, delta);
    assert_delta!(point.y, expected_point.y, delta);

}

#[test]
fn test_wgs84_lambert93(){
    let expected_point = Point::new(2.56865, 49.64961, 0.0);
    let point = Point::new(668832.5384, 6950138.7285, 0.0)
                .convert_wgs84(Zone::Lambert93)
                .convert_unit(AngleUnit::Meter, AngleUnit::Degree);
    let delta = 1e-3;

    assert_delta!(point.x, expected_point.x, delta);
    assert_delta!(point.y, expected_point.y, delta);
}

#[test]
fn test_wgs84_lambert_iie(){
    let expected_point = Point::new(-0.579117201473994, 44.84071560809383, 0.0);
    let point = Point::new(369419.0,1986498.0,0.0)
                    .convert_wgs84(Zone::LambertIIe)
                    .convert_unit(AngleUnit::Meter, AngleUnit::Degree);
    let delta = 1e-3;

    assert_delta!(point.x, expected_point.x, delta);
    assert_delta!(point.y, expected_point.y, delta);
}
