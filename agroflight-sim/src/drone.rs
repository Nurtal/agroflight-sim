use crate::errors::SimulationError;

pub struct Drone {
    pub model: String,
    pub mass_kg: f64,
    pub battery_capacity_ah: f64,
    pub max_speed_mps: f64,
    pub cruise_speed_mps: f64,
}

impl Drone {
    pub fn validate(&self) -> Result<(), SimulationError> {
        if !self.max_speed_mps.is_finite()
            || !self.cruise_speed_mps.is_finite()
            || self.max_speed_mps <= 0.0
            || self.cruise_speed_mps <= 0.0
            || self.cruise_speed_mps > self.max_speed_mps
        {
            return Err(SimulationError::InvalidDroneSpeed);
        }

        Ok(())
    }
}
