pub struct Field {
    pub name: String,
    pub width_m: f64,
    pub height_m: f64,
}

impl Field {
    pub fn area_hectares(&self) -> f64 {
        (self.width_m * self.height_m) / 10_000.0
    }
}
