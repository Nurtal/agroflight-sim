mod drone;
mod field;
mod mission;
mod geometry;
mod coverage;

use drone::Drone;
use field::Field;
use mission::{Mission, MissionType};
use geometry::Point;
use coverage::CoveragePlan;

fn main() {

    let drone = Drone {
        model: String::from("AgroQuad-X1"),
        mass_kg: 25.0,
        battery_capacity_ah: 20.0,
        max_speed_mps: 5.0,
    };

    let field = Field {
        name: String::from("Field-001"),
        width_m: 100.0,
        height_m: 50.0,
        origin: Point {x: 10.0, y: 20.0},
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

    let start = Point{x: 0.0, y: 0.0};
    let end = Point{x: 3.0, y: 4.0};

    println!("\nGEOMETRY TEST");
    println!("Distance {} m", start.distance_to(&end));

    println!("\nFIELD GEOMETRY");

    let corners = field.corners();
    
    for (index, point) in corners.iter().enumerate() {
        println!(
            "Corner {}: ({}, {})",
            index + 1,
            point.x,
            point.y
        );
    }

    let test_point = Point {x:50.0, y:40.0};
    println!(
        "Point (50, 40) inside field: {}",
        field.contains(&test_point)
    );


    let coverage_width_m = 10.0;
    let coverage_plan = CoveragePlan::generate(&field, coverage_width_m);

    println!("\nCOVERAGE PLAN");
    println!("Coverage width: {} m", coverage_width_m);
    println!("Number of lanes: {}", coverage_plan.lanes.len());

    for (index, (start, end)) in coverage_plan.lanes.iter().enumerate() {
        println!(
            "Lane {}: ({}, {}) -> ({}, {})",
            index + 1,
            start.x,
            start.y,
            end.x,
            end.y,
        );
    }

    
    
}
