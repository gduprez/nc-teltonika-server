use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TeltonikaGps {
    pub longitude: f64,
    pub latitude: f64,
    pub altitude: i16,
    pub angle: i16,
    pub satellites: u8,
    pub speed: i16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IoElement {
    pub id: u16,
    pub value: String, 
    pub label: String,
    pub dimension: String,
    pub value_human: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IoGroup {
    pub n1: Vec<IoElement>,
    pub n2: Vec<IoElement>,
    pub n4: Vec<IoElement>,
    pub n8: Vec<IoElement>,
    pub nx: Vec<IoElement>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AvlRecord {
    pub timestamp: DateTime<Utc>,
    pub priority: u8,
    pub gps: TeltonikaGps,
    pub event_id: u16,
    pub properties_count: u16,
    pub io_groups: IoGroup,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AvlData {
    pub codec_id: u8,
    pub number_of_data: u8,
    pub records: Vec<AvlRecord>,
}


