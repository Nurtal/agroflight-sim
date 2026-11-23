#[derive(Debug, PartialEq)]
pub enum SimulationError {
    InvalidFieldDimensions,
    InvalidCoverageWidth,
    InvalidDroneSpeed,
    InvalidTrajectory,
}
