extern crate std;

use std::f32;
use ::zone;

#[derive(Clone, Copy)]
pub enum AngleUnit {
   Radian,
   Degree,
   Grad,
}

fn factor(from:AngleUnit, to:AngleUnit) -> f32 {
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

    }
}

pub struct Point {
    x: f32,
    y: f32,
    z: f32,
    unit: AngleUnit,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32, unit: AngleUnit) -> Point {
        Point { x: x, y: y, z: z, unit: unit}
    }

    pub fn convert_unit(&mut self, unit:AngleUnit) {
        let point_unit = self.unit;
        self.scale(factor(point_unit, unit));
    }

    fn scale(&mut self, factor:f32) {
        self.x = self.x * factor;
        self.y = self.y * factor;
        self.z = self.z * factor;
    }
}

fn latitude_iso_from_latitude(lat: f32, e: f32) -> f32 {
    return f32::log2(f32::tan(f32::consts::FRAC_PI_4+lat/2.0)*f32::powf((1.0-e*f32::sin(lat))/(1.0+e*f32::sin(lat)),e/2.0));
}

fn latitude_from_latitude_iso(lat_iso: f32, e: f32, eps: f32) -> f32 {

    let mut phi_0 = 2.0*f32::atan(f32::exp(lat_iso)) - f32::consts::FRAC_PI_2;
    let mut phi_i = 2.0*f32::atan(f32::powf((1.0+e*f32::sin(phi_0))/(1.0-e*f32::sin(phi_0)),e/2.0)*f32::exp(lat_iso)) - f32::consts::FRAC_PI_2;
    let mut delta = 0.0;

    loop {

        delta = f32::abs(phi_i - phi_0);

        if delta > eps {
            break;
        }

        phi_0 = phi_i;
        phi_i = 2.0*f32::atan(f32::powf((1.0+e*f32::sin(phi_0))/(1.0-e*f32::sin(phi_0)),e/2.0)*f32::exp(lat_iso)) - f32::consts::FRAC_PI_2;
    }

    return phi_i
}

fn lambert_to_geographic(org: &Point, zone: zone::Zone, lon_merid: f32, e: f32, eps: f32) -> Point {

    let n = zone::n(zone);
    let C = zone::c(zone);
    let x_s = zone::xs(zone);
    let y_s = zone::ys(zone);

    let mut x = org.x;
    let mut y = org.y;

    let mut lon: f32 = 0.0;
    let mut gamma: f32 = 0.0;
    let mut R: f32 = 0.0;
    let mut lat_iso: f32 = 0.0;

    R = f32::sqrt((x-x_s)*(x-x_s)+(y-y_s)*(y-y_s));
    gamma = f32::atan((x-x_s)/(y_s-y));

    lon = lon_merid + gamma/n;

    lat_iso = -1.0/n*f32::log2(f32::abs(R/C));

    let lat = latitude_from_latitude_iso(lat_iso, e, eps);

    return Point { x: lon, y: lat, z: org.z, unit: org.unit};
}


fn lambert_normal(lat: f32, a: f32, e: f32) -> f32 {
    return a/f32::sqrt(1.0-e*e*f32::sin(lat)*f32::sin(lat));
}

fn geographic_to_cartesian(lon: f32, lat: f32, he: f32, a: f32, e: f32) -> Point {

    let N = lambert_normal(lat, a, e);

    let mut pt = Point::new(0.0,0.0,0.0, AngleUnit::Radian);
    pt.x = (N+he)*f32::cos(lat)*f32::cos(lon);

 	pt.y = (N+he)*f32::cos(lat)*f32::sin(lon);

 	pt.z = (N*(1.0-e*e)+he)*f32::sin(lat);
    return pt
}

fn cartesian_to_geographic(point: &Point, meridien: f32, a: f32, e: f32, eps: f32) -> Point{

    let (x, y, z) = (point.x, point.y, point.z);
    let lon = meridien + f32::atan(y/x);

 	let module = f32::sqrt(x*x + y*y);

 	let mut phi_0 = f32::atan(z/(module*(1.0-(a*e*e)/f32::sqrt(x*x+y*y+z*z))));
 	let mut phi_i = f32::atan(z/module/(1.0-a*e*e*f32::cos(phi_0)/(module * f32::sqrt(1.0-e*e*f32::sin(phi_0)*f32::sin(phi_0)))));
 	let mut delta = 0.0;

    loop {
        delta  = f32::abs(phi_i - phi_0);

        if delta > eps {
            break;
        }

        phi_0 = phi_i;
        phi_i = f32::atan(z/module/(1.0-a*e*e*f32::cos(phi_0)/(module * f32::sqrt(1.0-e*e*f32::sin(phi_0)*f32::sin(phi_0)))));


    }

    let he = module/f32::cos(phi_i) - a/f32::sqrt(1.0-e*e*f32::sin(phi_i)*f32::sin(phi_i));
 	return Point { x:lon, y:phi_i, z: he, unit: AngleUnit::Radian};
}

#[test]
fn test_new(){
    let point = Point::new(55.0,1.0,0.0,AngleUnit::Degree);
    assert_eq!(point.x, 55.0);
    assert_eq!(point.y, 1.0);
    assert_eq!(point.z, 0.0);
}

#[test]
fn test_scale(){
    let mut point = Point::new(55.0,1.0,0.0,AngleUnit::Degree);
    point.scale(2.0);
    assert_eq!(point.x, 110.0);
    assert_eq!(point.y, 2.0);
    assert_eq!(point.z, 0.0);
}
#[test]
fn test_convert(){
    let mut point = Point::new(180.0,360.0,0.0,AngleUnit::Degree);
    point.convert_unit(AngleUnit::Radian);
    assert_eq!(point.x, std::f32::consts::PI);
    assert_eq!(point.y, 2.0 * std::f32::consts::PI);
    assert_eq!(point.z, 0.0);
}
