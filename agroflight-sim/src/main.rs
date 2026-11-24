mod coverage;
mod drone;
mod errors;
mod field;
mod geometry;
mod metrics;
mod mission;
mod trajectory;

use coverage::CoveragePlan;
use drone::Drone;
use field::Field;
use geometry::Point;
use metrics::MissionMetrics;
use mission::{Mission, MissionType};
use trajectory::Trajectory;

fn main() {
    let drone = Drone {
        model: String::from("AgroQuad-X1"),
        empty_mass_kg: 25.0,
        max_payload_kg: 15.0,
        payload_kg: 10.0,
        battery_capacity_ah: 20.0,
        battery_voltage_v: 48.0,
        max_speed_mps: 10.0,
        cruise_speed_mps: 5.0,
    };

    let field = Field {
        name: String::from("Field-001"),
        width_m: 100.0,
        height_m: 50.0,
        origin: Point { x: 10.0, y: 20.0 },
    };

    let mission = Mission {
        name: String::from("Mission-001"),
        mission_type: MissionType::Spraying,
        target_altitude_m: 30.0,
    };

    println!("AGROFLIGHT SIM");
    println!("==============");

    println!("\nDRONE");
    println!("Model: {}", drone.model);
    println!("Empty Mass: {}", drone.empty_mass_kg);
    println!("Battery: {}", drone.battery_capacity_ah);
    println!("Payload {} kg", drone.payload_kg);
    println!("Max Payload {} kg", drone.max_payload_kg);
    println!("Total mass {} kg", drone.total_mass_kg());
    println!("Battery energy {} Wh", drone.battery_energy_wh());
    println!("Max Speed: {}", drone.max_speed_mps);

    println!("\nFIELD");
    println!("Name: {}", field.name);
    println!("Dimensions: {} x {}", field.width_m, field.height_m);
    println!("Area: {} hectares", field.area_hectares());

    println!("\nMISSION");
    println!("Name: {}", mission.name);
    println!("Altitude: {}", mission.target_altitude_m);

    let start = Point { x: 0.0, y: 0.0 };
    let end = Point { x: 3.0, y: 4.0 };

    println!("\nGEOMETRY TEST");
    println!("Distance {} m", start.distance_to(&end));

    println!("\nFIELD GEOMETRY");

    let corners = field.corners();

    for (index, point) in corners.iter().enumerate() {
        println!("Corner {}: ({}, {})", index + 1, point.x, point.y);
    }

    let test_point = Point { x: 50.0, y: 40.0 };
    println!(
        "Point (50, 40) inside field: {}",
        field.contains(&test_point)
    );

    let coverage_width_m = 10.0;
    let coverage_plan =
        CoveragePlan::generate(&field, coverage_width_m).expect("Fail to generate coverage Plan");

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

    let trajectory = Trajectory::from_coverage_plan(&coverage_plan);

    println!("\nFLIGHT TRAJECTORY");
    println!("Number of waypoints: {}", trajectory.waypoints.len());
    for (index, point) in trajectory.waypoints.iter().enumerate() {
        println!("waypoint {}: {:.2}, {:.2}", index + 1, point.x, point.y);
    }
    println!(
        "Total flight distance: {:.2} m",
        trajectory.total_distance()
    );

    let metrics = MissionMetrics::calculate(&drone, &trajectory)
        .expect("Failed to calculate mission metrics");

    println!("\nMISSION PERFORMANCE");
    println!("Distance: {:.2} m", metrics.total_distance_m);
    println!("Duration: {:.2} s", metrics.flight_duration_s);
    println!("Average Speed: {:.2}", metrics.average_speed_mps);
}
