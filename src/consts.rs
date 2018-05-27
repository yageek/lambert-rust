use std;

pub const DEFAULT_EPS: f32 =  1e-15;
pub const E_CLARK_IGN: f32 = 0.08248325676;
pub const E_WGS84: f32 = 0.08181919106;

pub const A_CLARK_IGN: f32 = 6378249.2;
pub const A_WGS84: f32 = 6378137.0;

pub const LON_MERID_PARIS: f32 = 0.0;
pub const LON_MERID_GREENWICH: f32 = 0.04079234433;
pub const LON_MERID_IERS: f32 = (3.0*std::f32::consts::PI/180.0);

// Future usage
#[allow(dead_code)]
const AUTOCOMEIQUE_FIRST: f32 = 44.0*std::f32::consts::PI/180.0;

#[allow(dead_code)]
const AUTOCOMEIQUE_SECOND: f32 = 49.0*std::f32::consts::PI/180.0;

#[allow(dead_code)]
const LAT_ORIG: f32 = 46.5*std::f32::consts::PI/180.0;

#[allow(dead_code)]
const CT_X0: f32 = 700000.0;

#[allow(dead_code)]
const CT_Y0: f32 = 6600000.0;
