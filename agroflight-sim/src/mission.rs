pub enum MissionType {
    Mapping,
    Spraying,
    Inspection,
}

pub struct Mission {
    pub name: String,
    pub mission_type: MissionType,
    pub target_altitude_m: f64,
}
