use crate::coverage::CoveragePlan;
use crate::geometry::Point;

pub struct Trajectory {
    pub waypoints: Vec<Point>,
}

impl Trajectory {
    pub fn from_coverage_plan(plan: &CoveragePlan) -> Self {
        let mut waypoints = Vec::new();

        for (index, (start, end)) in plan.lanes.iter().enumerate() {
            if index % 2 == 0 {
                waypoints.push(Point {
                    x: start.x,
                    y: start.y,
                });
                waypoints.push(Point { x: end.x, y: end.y });
            } else {
                waypoints.push(Point { x: end.x, y: end.y });
                waypoints.push(Point {
                    x: start.x,
                    y: start.y,
                });
            }
        }

        Self { waypoints }
    }

    pub fn total_distance(&self) -> f64 {
        self.waypoints
            .windows(2)
            .map(|pair| pair[0].distance_to(&pair[1]))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coverage::CoveragePlan;
    use crate::field::Field;

    fn create_test_plan() -> CoveragePlan {
        let field = Field {
            name: String::from("Test field"),
            width_m: 30.0,
            height_m: 20.0,
            origin: Point { x: 0.0, y: 0.0 },
        };

        CoveragePlan::generate(&field, 10.0).unwrap()
    }

    #[test]
    fn generates_correct_number_of_waypoints() {
        let plan = create_test_plan();
        let trajectory = Trajectory::from_coverage_plan(&plan);

        assert_eq!(trajectory.waypoints.len(), 6);
    }

    #[test]
    fn alternates_flight_direction() {
        let plan = create_test_plan();
        let trajectory = Trajectory::from_coverage_plan(&plan);

        assert_eq!(trajectory.waypoints[0].y, 0.0);
        assert_eq!(trajectory.waypoints[1].y, 20.0);

        assert_eq!(trajectory.waypoints[2].y, 20.0);
        assert_eq!(trajectory.waypoints[3].y, 0.0);
    }

    #[test]
    fn calculates_total_distance() {
        let plan = create_test_plan();
        let trajectory = Trajectory::from_coverage_plan(&plan);

        assert_eq!(trajectory.total_distance(), 80.0);
    }
}
