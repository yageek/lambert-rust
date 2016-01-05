enum Unit {
   Radian,
   Degree,
   Meter,
   Grad,
}

fn factor(from:Unit, to:Unit) -> f32 {
    match (from, to) {
        // Logical conversion
        (Unit::Degree, Unit::Degree) => 1.0,
        (Unit::Radian, Unit::Radian) => 1.0,
        (Unit::Meter, Unit::Meter) => 1.0,
        (Unit::Grad, Unit::Grad) => 1.0,

        //-> Radian
        (Unit::Degree, Unit::Radian) => 
    }
}
struct Point {
    x: f32,
    y: f32,
    z: f32,
    unit: Unit,
}

impl Point {
    fn new(x: f32, y: f32, z: f32, unit: Unit) -> Point {
        Point { x: x, y: y, z: z, unit: unit}
    }
    fn scale(&mut self, factor:f32) {
        self.x = self.x * factor;
        self.y = self.y * factor;
        self.z = self.z * factor;
    }
}
