use serde::{Serialize, Serializer, Deserialize};
use chrono::{DateTime, Utc, SecondsFormat};

fn serialize_gps_coord<S>(value: &f64, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if *value == value.trunc() && value.is_finite() {
        s.serialize_i64(*value as i64)
    } else {
        s.serialize_f64(*value)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TeltonikaGps {
    #[serde(serialize_with = "serialize_gps_coord")]
    pub longitude: f64,
    #[serde(serialize_with = "serialize_gps_coord")]
    pub latitude: f64,
    pub altitude: i16,
    pub angle: i16,
    pub satellites: u8,
    pub speed: i16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IoElement {
    pub id: u16,
    pub label: String,
    pub value: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension: Option<String>,
    #[serde(rename = "valueHuman")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_human: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IoGroup {
    pub n1: Vec<IoElement>,
    pub n2: Vec<IoElement>,
    pub n4: Vec<IoElement>,
    pub n8: Vec<IoElement>,
    pub nx: Vec<IoElement>,
}

fn serialize_timestamp_millis<S>(dt: &DateTime<Utc>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&dt.to_rfc3339_opts(SecondsFormat::Millis, true))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AvlRecord {
    #[serde(serialize_with = "serialize_timestamp_millis")]
    pub timestamp: DateTime<Utc>,
    pub priority: u8,
    pub gps: TeltonikaGps,
    pub event_id: u16,
    #[serde(rename = "ioGroups")]
    pub io_groups: IoGroup,
    #[serde(rename = "ioElements")]
    pub io_elements: Vec<serde_json::Value>,
    pub properties_count: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AvlData {
    pub codec_id: u8,
    pub number_of_data: u8,
    pub records: Vec<AvlRecord>,
}


