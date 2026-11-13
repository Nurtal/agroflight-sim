use crate::field::Field;
use crate::geometry::Point;

pub struct CoveragePlan {
    pub lanes: Vec<(Point, Point)>,
}

impl CoveragePlan {
    pub fn generate(field: &Field, coverage_width_m: f64) -> Self {
        let number_of_lanes = (field.width_m / coverage_width_m).ceil() as usize;
        let mut lanes = Vec::new();

        for i in 0..number_of_lanes {
            let x = field.origin.x + (i as f64 + 0.5) * coverage_width_m;
            let start = Point {x, y: field.origin.y};
            let end = Point {x, y: field.origin.y + field.height_m};

            lanes.push((start, end));
        }
        Self { lanes }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_field() -> Field {
        Field { 
            name: String::from("Test field"),
            width_m: 100.0,
            height_m: 50.0,
            origin: Point { x:10.0, y:20.0 },
        }
    }

    #[test]
    fn generates_correct_number_of_lanes() {
        let field = create_test_field();
        let plan = CoveragePlan::generate(&field, 10.0);
        assert_eq!(plan.lanes.len(), 10);
    }

    #[test]
    fn generates_lanes_at_correct_positions() {
        let field = create_test_field();
        let plan = CoveragePlan::generate(&field, 10.0);

        let (start, end) = &plan.lanes[0];

        assert_eq!(start.x, 15.0);
        assert_eq!(start.y, 20.0);
        assert_eq!(end.x, 15.0);
        assert_eq!(end.y, 70.0);
        
    }

    #[test]
    fn generates_lanes_for_non_multiple_width() {
        let field = Field {
            name: String::from("Odd field"),
            width_m: 105.0,
            height_m: 50.0,
            origin: Point { x: 0.0, y: 0.0},
        };

        let plan = CoveragePlan::generate(&field, 10.0);
        
        assert_eq!(plan.lanes.len(), 11);
    }
}
