use vector2d::Vector2Df;

pub fn from_lra_zoom(zoom: f32) -> f64 {
    f64::log(f64::from(zoom), 2.0)
}

pub fn from_lra_scenery_width(width: u8) -> f64 {
    f64::from(width) / 10.0
}

pub fn from_web_gravity(gravity: Vector2Df) -> Vector2Df {
    gravity / 0.175
}

pub fn to_web_gravity(gravity: Vector2Df) -> Vector2Df {
    gravity * 0.175
}
