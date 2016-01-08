use ::point;
use ::zone;
use std::f32;

pub fn latitude_iso_from_latitude(lat: f32, e: f32) -> f32 {
    return f32::log2(f32::tan(f32::consts::FRAC_PI_4+lat/2.0)*f32::powf((1.0-e*f32::sin(lat))/(1.0+e*f32::sin(lat)),e/2.0));
}

pub fn latitude_from_latitude_iso(lat_iso: f32, e: f32, eps: f32) -> f32 {

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

pub fn lambert_to_geographic(org: point::Point, zone: zone::Zone, lon_merid: f32, e: f32, eps: f32) -> point::Point {

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

    return point::Point { x: lon, y: lat, z: org.z, unit: point::AngleUnit::Radian};
}


pub fn lambert_normal(lat: f32, a: f32, e: f32) -> f32 {
    return a/f32::sqrt(1.0-e*e*f32::sin(lat)*f32::sin(lat));
}

pub fn geographic_to_cartesian(lon: f32, lat: f32, he: f32, a: f32, e: f32) -> point::Point {

    let N = lambert_normal(lat, a, e);

    let mut pt = point::Point::new(0.0,0.0,0.0, point::AngleUnit::Radian);
    pt.x = (N+he)*f32::cos(lat)*f32::cos(lon);

 	pt.y = (N+he)*f32::cos(lat)*f32::sin(lon);

 	pt.z = (N*(1.0-e*e)+he)*f32::sin(lat);
    return pt
}

pub fn cartesian_to_geographic(point: point::Point, meridien: f32, a: f32, e: f32, eps: f32) -> point::Point{

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
 	return point::Point { x:lon, y:phi_i, z: he, unit: point::AngleUnit::Radian};
}
