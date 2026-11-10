mod drone;
mod field;
mod mission;

use drone::Drone;
use field::Field;
use mission::{Mission, MissionType};

fn main() {

    let drone = Drone {
        model: String::from("AgroQuad-X1"),
        mass_kg: 25.0,
        battery_capacity_ah: 20.0,
        max_speed_mps: 5.0,
    };

    let field = Field {
        name: String::from("Field-001"),
        width_m: 500.0,
        height_m: 200.0,
    };

    let mission = Mission{
        name: String::from("Mission-001"),
        mission_type: MissionType::Spraying,
        target_altitude_m: 30.0,
        
    };
    
    println!("AGROFLIGHT SIM");
    println!("==============");

    println!("\nDRONE");
    println!("Model: {}", drone.model);
    println!("Mass: {}", drone.mass_kg);
    println!("Battery: {}", drone.battery_capacity_ah);
    println!("Max Speed: {}", drone.max_speed_mps);

    println!("\nFIELD");
    println!("Name: {}", field.name);
    println!("Dimensions: {} x {}", field.width_m, field.height_m);
    println!("Area: {} hectares", field.area_hectares());

    println!("\nMISSION");
    println!("Name: {}", mission.name);
    println!("Altitude: {}", mission.target_altitude_m);
    
}
