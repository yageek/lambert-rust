extern crate std;

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

    fn scale(&mut self, factor:f32) {
        self.x = self.x * factor;
        self.y = self.y * factor;
        self.z = self.z * factor;
    }

    pub fn convert_unit(&mut self, unit:AngleUnit) {
        let point_unit = self.unit;
        self.scale(factor(point_unit, unit));
    }
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
