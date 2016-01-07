#[derive(Copy, Clone)]
pub enum Zone {
    LambertI,
    LambertII,
    LambertIII,
    LambertIV,
    LambertIIe,
    Lambert93
}

pub fn n(zone:Zone) -> f32 {
    match zone {
        Zone::LambertI => 0.7604059656,
        Zone::LambertII => 0.7289686274,
        Zone::LambertIII => 0.6959127966,
        Zone::LambertIV => 0.6712679322,
        Zone::LambertIIe => 0.7289686274,
        Zone::Lambert93 =>  0.7256077650,
    }
}

pub fn c(zone:Zone) -> f32 {
    match zone {
        Zone::LambertI => 11603796.98,
        Zone::LambertII => 11745793.39,
        Zone::LambertIII => 11947992.52,
        Zone::LambertIV => 12136281.99,
        Zone::LambertIIe => 11745793.39,
        Zone::Lambert93 =>  11754255.426,
    }
}

pub fn xs(zone:Zone) -> f32 {
    match zone {
        Zone::LambertIV => 234.358,
        Zone::Lambert93 =>  700000.0,
        _ => 600000.0,
    }
}

pub fn ys(zone:Zone) -> f32 {
    match zone {
        Zone::LambertI => 5657616.674,
        Zone::LambertII => 6199695.768,
        Zone::LambertIII => 6791905.085,
        Zone::LambertIV => 7239161.542,
        Zone::LambertIIe => 8199695.768,
        Zone::Lambert93 =>  12655612.050,
    }
}
