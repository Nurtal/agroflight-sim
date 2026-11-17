use crate::drone::Drone;
use crate::trajectory::Trajectory;

pub struct MissionMetrics {
    pub total_distance_m: f64,
    pub flight_duration_s: f64,
    pub average_speed_mps: f64,
}

impl MissionMetrics {
    pub fn calculate(drone: &Drone, trajectory: &Trajectory) -> Self {
        let total_distance_m = trajectory.total_distance();

        let flight_duration_s = total_distance_m / drone.cruise_speed_mps;

        let average_speed_mps = total_distance_m / flight_duration_s;

        Self {
            total_distance_m,
            flight_duration_s,
            average_speed_mps,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;

    #[test]
    fn calculates_flight_duration() {
        let drone = Drone {
            model: String::from("Test drone"),
            mass_kg: 25.0,
            battery_capacity_ah: 2.0,
            max_speed_mps: 10.0,
            cruise_speed_mps: 5.0,
        };

        let trajectory = Trajectory {
            waypoints: vec![Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 100.0 }],
        };

        let metrics = MissionMetrics::calculate(&drone, &trajectory);

        assert_eq!(metrics.total_distance_m, 100.0);
        assert_eq!(metrics.flight_duration_s, 20.0);
        assert_eq!(metrics.average_speed_mps, 5.0);
    }
}
