use crate::drone::Drone;
use crate::errors::SimulationError;
use crate::trajectory::Trajectory;

pub struct MissionMetrics {
    pub total_distance_m: f64,
    pub flight_duration_s: f64,
    pub average_speed_mps: f64,
}

impl MissionMetrics {
    pub fn calculate(drone: &Drone, trajectory: &Trajectory) -> Result<Self, SimulationError> {
        drone.validate()?;
        if trajectory.waypoints.len() < 2 {
            return Err(SimulationError::InvalidTrajectory);
        }

        let total_distance_m = trajectory.total_distance();

        if !total_distance_m.is_finite() || total_distance_m <= 0.0 {
            return Err(SimulationError::InvalidTrajectory);
        }

        let flight_duration_s = total_distance_m / drone.cruise_speed_mps;

        let average_speed_mps = total_distance_m / flight_duration_s;

        Ok(Self {
            total_distance_m,
            flight_duration_s,
            average_speed_mps,
        })
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
            empty_mass_kg: 25.0,
            max_payload_kg: 15.0,
            payload_kg: 10.0,
            battery_capacity_ah: 2.0,
            battery_voltage_v: 48.0,
            max_speed_mps: 10.0,
            cruise_speed_mps: 5.0,
        };

        let trajectory = Trajectory {
            waypoints: vec![Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 100.0 }],
        };

        let metrics = MissionMetrics::calculate(&drone, &trajectory).unwrap();

        assert_eq!(metrics.total_distance_m, 100.0);
        assert_eq!(metrics.flight_duration_s, 20.0);
        assert_eq!(metrics.average_speed_mps, 5.0);
    }
}
