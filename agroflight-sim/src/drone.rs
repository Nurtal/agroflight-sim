use crate::errors::SimulationError;

pub struct Drone {
    // Model
    pub model: String,

    // Physical characs
    pub empty_mass_kg: f64,
    pub max_payload_kg: f64,
    pub payload_kg: f64,

    // Battery
    pub battery_capacity_ah: f64,
    pub battery_voltage_v: f64,

    // Flight
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

    pub fn total_mass_kg(&self) -> f64 {
        self.empty_mass_kg + self.payload_kg
    }

    pub fn battery_energy_wh(&self) -> f64 {
        self.battery_capacity_ah * self.battery_voltage_v
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn create_test_drone() -> Drone {
        Drone {
            model: String::from("Test Drone"),
            empty_mass_kg: 25.0,
            max_payload_kg: 15.0,
            payload_kg: 10.0,
            battery_capacity_ah: 20.0,
            battery_voltage_v: 48.0,
            max_speed_mps: 10.0,
            cruise_speed_mps: 5.0,
        }
    }

    #[test]
    fn calculates_total_mass() {
        let drone = create_test_drone();
        assert_eq!(drone.total_mass_kg(), 35.0);
    }

    #[test]
    fn calculates_battery_energy() {
        let drone = create_test_drone();
        assert_eq!(drone.battery_energy_wh(), 960.0);
    }
}
