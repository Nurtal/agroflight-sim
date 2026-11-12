use crate::geometry::Point;

pub struct Field {
    pub name: String,
    pub width_m: f64,
    pub height_m: f64,
    pub origin: Point,
}

impl Field {
    pub fn area_hectares(&self) -> f64 {
        (self.width_m * self.height_m) / 10_000.0
    }

    pub fn corners(&self) -> [Point; 4] {
        let x = self.origin.x;
        let y = self.origin.y;

        [
            Point {x, y},
            Point {x: x + self.width_m, y},
            Point {x: x + self.width_m, y: y + self.height_m},
            Point {x, y: y + self.height_m},
        ]
        
    }

    pub fn contains(&self, point: &Point) -> bool {
        point.x >= self.origin.x
            && point.x <= self.origin.x + self.width_m
            && point.y >= self.origin.y
            && point.y <= self.origin.y + self.height_m
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_field() -> Field {
        Field {
            name: String::from("Test field"),
            width_m:100.0,
            height_m: 50.0,
            origin: Point { x: 10.0, y: 20.0},
        }
    }

    #[test]
    fn calculates_area_correctly() {
        let field = create_test_field();

        assert_eq!(field.area_hectares(), 0.5);
        
    }

    #[test]
    fn generates_four_corners() {
        let field = create_test_field();
        let corners = field.corners();

        assert_eq!(corners.len(), 4);
        assert_eq!(corners[0].x, 10.0);
        assert_eq!(corners[0].y, 20.0);
        assert_eq!(corners[2].x, 110.0);
        assert_eq!(corners[2].y, 70.0)
    }

    #[test]
    fn detects_point_inside_field() {
        let field = create_test_field();
        let point = Point { x:50.0, y: 40.0};

        assert!(field.contains(&point));
    }

    #[test]
    fn detect_point_outside_field() {
        let field = create_test_field();
        let point = Point {x: 150.0, y: 40.0};

        assert!(!field.contains(&point));
    }
}
