
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        (dx * dx + dy * dy).sqrt()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_distance_correctly() {
        let point_a = Point {x: 0.0, y: 0.0 };
        let point_b = Point {x: 3.0, y: 4.0 };

        assert_eq!(point_a.distance_to(&point_b), 5.0);
    }
}
