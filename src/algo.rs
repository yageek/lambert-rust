use ::point;
use ::zone;
use ::consts;
use std::f32;


/// Convert latitude to latitude iso
pub fn latitude_iso_from_latitude(lat: f32, e: f32) -> f32 {
    return f32::log(f32::tan(f32::consts::FRAC_PI_4+lat/2.0)*f32::powf((1.0-e*f32::sin(lat))/(1.0+e*f32::sin(lat)),e/2.0), f32::consts::E);
}

pub fn latitude_from_latitude_iso(lat_iso: f32, e: f32, eps: f32) -> f32 {

    let mut phi_0 = 2.0*f32::atan(f32::exp(lat_iso)) - f32::consts::FRAC_PI_2;
    let mut phi_i = 2.0*f32::atan(f32::powf((1.0+e*f32::sin(phi_0))/(1.0-e*f32::sin(phi_0)),e/2.0)*f32::exp(lat_iso)) - f32::consts::FRAC_PI_2;
    let mut delta = 0.0;

    loop {

        delta = f32::abs(phi_i - phi_0);

        if delta < eps {
            break;
        }

        phi_0 = phi_i;
        phi_i = 2.0*f32::atan(f32::powf((1.0+e*f32::sin(phi_0))/(1.0-e*f32::sin(phi_0)),e/2.0)*f32::exp(lat_iso)) - f32::consts::FRAC_PI_2;
    }

    return phi_i
}
#[test]
fn test_latitude_from_latitude_iso(){
    let lat_iso: [f32; 3] = [1.00552653648,-0.30261690060 ,0.2000000000];
	let e: [f32; 3] = [0.08199188998,0.08199188998,0.08199188998];
	let eps: [f32; 3] = [1.0e-11,1.0e-11,1.0e-11];

	let phi: [f32; 3] = [0.87266462600, -0.29999999997 ,0.19998903369];

		for index in 0..3 {
			let result = latitude_from_latitude_iso(lat_iso[index], e[index], eps[index]);
			assert_delta!(result, phi[index], 1e-7);
		}
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

    lat_iso = -1.0/n*f32::log(f32::abs(R/C), f32::consts::E);

    let lat = latitude_from_latitude_iso(lat_iso, e, eps);

    return point::Point { x: lon, y: lat, z: org.z, unit: point::AngleUnit::Radian};
}

#[test]
fn test_lambert_to_geographic(){
    let expected = point::Point::new(0.145512099,0.872664626, 0.0, point::AngleUnit::Radian);
    let org = point::Point::new(1029705.083, 272723.849, 0.0, point::AngleUnit::Radian);

    let delta = 1e-7;
    let dest = lambert_to_geographic(org, zone::Zone::LambertI, ::consts::LON_MERID_GREENWICH, ::consts::E_CLARK_IGN, delta);
    assert_delta!(dest.x, expected.x, delta);
    assert_delta!(dest.y, expected.y, delta);
    assert_delta!(dest.z, expected.z, delta);
}


pub fn lambert_normal(lat: f32, a: f32, e: f32) -> f32 {
    return a/f32::sqrt(1.0-e*e*f32::sin(lat)*f32::sin(lat));
}

#[test]
fn test_lambert_normal(){
    let n = 6393174.9755;
    let lat = 0.97738438100;
    let a = 6378388.0000;
    let e = 0.081991890;

    let calc = lambert_normal(lat,a,e);
    assert_eq!(n, calc);
}

pub fn geographic_to_cartesian(lon: f32, lat: f32, he: f32, a: f32, e: f32) -> point::Point {

    let N = lambert_normal(lat, a, e);

    let mut pt = point::Point::new(0.0,0.0,0.0, point::AngleUnit::Radian);
    pt.x = (N+he)*f32::cos(lat)*f32::cos(lon);

 	pt.y = (N+he)*f32::cos(lat)*f32::sin(lon);

 	pt.z = (N*(1.0-e*e)+he)*f32::sin(lat);
    return pt
}

#[test]
fn test_geographic_to_cartesian(){
    let lon:[f32; 3] = [0.01745329248 ,0.00290888212 ,0.00581776423];
	let lat:[f32; 3] = [0.02036217457,0.00000000000 ,-0.03199770300];
	let he:[f32; 3] = [100.0000,10.0000 ,2000.0000];
	let a:[f32; 3] = [6378249.2000 ,6378249.2000 ,6378249.2000];
	let e:[f32; 3] = [0.08248325679 ,0.08248325679 ,0.08248325679];

    let points  = vec![
        point::Point::new(6376064.6955,111294.6230,128984.7250, point::AngleUnit::Meter),
        point::Point::new(6378232.2149,18553.5780,0.0, point::AngleUnit::Meter),
        point::Point::new(6376897.5369,37099.7050,-202730.9070, point::AngleUnit::Meter)
        ];

    let delta = 1e-1;
    for i in 0..points.len() {
        let pt  = geographic_to_cartesian(lon[i],lat[i],he[i],a[i],e[i]);

        assert_delta!(pt.x,points[i].x, delta);
        assert_delta!(pt.y,points[i].y, delta);
        assert_delta!(pt.z,points[i].z, delta);
    }
}

pub fn cartesian_to_geographic(point: point::Point, meridien: f32, a: f32, e: f32, eps: f32) -> point::Point{

    let (x, y, z) = (point.x, point.y, point.z);
    let lon = meridien + f32::atan(y/x);

 	let module = f32::sqrt(x*x + y*y);

 	let mut phi_0 = f32::atan(z/(module*(1.0-(a*e*e)/f32::sqrt(x*x+y*y+z*z))));
 	let mut phi_i = f32::atan(z/module/(1.0-a*e*e*f32::cos(phi_0)/(module * f32::sqrt(1.0-e*e*f32::sin(phi_0)*f32::sin(phi_0)))));
 	let mut delta = 0.0;

    loop {
        delta = f32::abs(phi_i - phi_0);
        if delta < eps {
            break;
        }
        phi_0 = phi_i;
        phi_i = f32::atan(z/module/(1.0-a*e*e*f32::cos(phi_0)/(module * f32::sqrt(1.0-e*e*f32::sin(phi_0)*f32::sin(phi_0)))));
    }

    let he = module/f32::cos(phi_i) - a/f32::sqrt(1.0-e*e*f32::sin(phi_i)*f32::sin(phi_i));
 	return point::Point { x:lon, y:phi_i, z: he, unit: point::AngleUnit::Radian};
}

#[test]
fn test_cartesian_to_geographic(){

    let a: [f32; 3] = [6378249.2000, 6378249.2000 ,6378249.2000];
    let e: [f32; 3] = [0.08248325679, 0.08248325679, 0.08248325679];
    let x: [f32; 3] = [6376064.6950, 6378232.2150, 6376897.5370];
    let y: [f32; 3] = [111294.6230, 18553.5780, 37099.7050];
    let z: [f32; 3] = [128984.7250, 0.0000, -202730.9070];
    let eps: [f32; 3] = [1e-11,1e-11,1e-11];

    let lon: [f32; 3] = [0.01745329248, 0.00290888212, 0.00581776423];
    let lat: [f32; 3] = [0.02036217457, 0.00000000000, -0.03199770301];
    let he: [f32; 3] = [99.9995, 10.0001, 2000.0001];

    let delta = 1e-8;
    for i in 0..3 {
        let sample = point::Point::new(x[i],y[i],z[i], point::AngleUnit::Radian);

		let val = cartesian_to_geographic(sample,::consts::LON_MERID_PARIS,a[i],e[i],eps[i]);

		assert_delta!(val.x,lon[i],delta);
		assert_delta!(val.y,lat[i],delta);
        assert_delta!(val.z,he[i],1e-3);
    }
}
