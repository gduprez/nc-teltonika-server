use bytes::{Buf, Bytes};
use chrono::{TimeZone, Utc};
use super::models::{AvlRecord, TeltonikaGps, IoGroup, IoElement};
use super::io_elements::get_io_element_definition;

const GPS_PRECISION: f64 = 10000000.0;

// Teltonika Protocol: Timestamp is in milliseconds since 1970-01-01 00:00:00 UTC.

pub fn parse(buf: &mut Bytes, number_of_data: u8) -> Result<Vec<AvlRecord>, Box<dyn std::error::Error>> {
    let mut records = Vec::new();
    for _ in 0..number_of_data {
        records.push(parse_record(buf)?);
    }
    Ok(records)
}

fn parse_record(buf: &mut Bytes) -> Result<AvlRecord, Box<dyn std::error::Error>> {
    // Timestamp: 8 bytes
    if buf.remaining() < 8 { return Err("Not enough bytes for timestamp".into()); }
    let timestamp_ms = buf.get_i64();
    let timestamp = Utc.timestamp_millis_opt(timestamp_ms).single().ok_or("Invalid timestamp")?;

    // Priority: 1 byte
    if buf.remaining() < 1 { return Err("Not enough bytes for priority".into()); }
    let priority = buf.get_u8();

    // GPS: 15 bytes
    // Longitude: 4 bytes (i32)
    // Latitude: 4 bytes (i32)
    // Altitude: 2 bytes (i16)
    // Angle: 2 bytes (i16)
    // Satellites: 1 byte (u8)
    // Speed: 2 bytes (i16)
    if buf.remaining() < 15 { return Err("Not enough bytes for GPS".into()); }
    let longitude_raw = buf.get_i32();
    let latitude_raw = buf.get_i32();
    let altitude = buf.get_i16();
    let angle = buf.get_i16();
    let satellites = buf.get_u8();
    let speed = buf.get_i16();

    let longitude = longitude_raw as f64 / GPS_PRECISION;
    let latitude = latitude_raw as f64 / GPS_PRECISION;

    // Event ID: 2 bytes (Wait, JS says ReadBytes(2) then `toInt`. `toInt` is hex parse.
    // binutils ReadBytes returns a buffer.
    // Codec8e.ts: event_id: this.toInt(this.reader.ReadBytes(2))
    // Typically Event ID is u16? Or u8?
    // Codec8 protocol says Event IO ID is 1 byte in Codec8, but maybe 2 in Codec8 Extended (142)?
    // Codec8 Extended: Event IO ID is 2 bytes. correct.
    if buf.remaining() < 2 { return Err("Not enough bytes for event id".into()); }
    let event_id = buf.get_u16(); // assuming BE

    // Properties count: 2 bytes (Codec 8 Extended)
    if buf.remaining() < 2 { return Err("Not enough bytes for properties count".into()); }
    let properties_count = buf.get_u16();

    // IO Elements
    let io_groups = parse_io_elements(buf)?;

    Ok(AvlRecord {
        timestamp,
        priority,
        gps: TeltonikaGps {
            longitude,
            latitude,
            altitude,
            angle,
            satellites,
            speed,
        },
        event_id,
        properties_count,
        io_groups,
        io_elements: vec![],
    })
}

fn parse_io_elements(buf: &mut Bytes) -> Result<IoGroup, Box<dyn std::error::Error>> {
    let mut n1 = Vec::new();
    let mut n2 = Vec::new();
    let mut n4 = Vec::new();
    let mut n8 = Vec::new();
    let mut nx = Vec::new();

    // 1 byte IOs
    if buf.remaining() < 2 { return Err("Not enough bytes for 1-byte IO count".into()); }
    let count_n1 = buf.get_u16();
    for _ in 0..count_n1 {
        if buf.remaining() < 3 { return Err("Not enough bytes for 1-byte IO".into()); }
        let id = buf.get_u16();
        let value = buf.get_u8() as i64; // Value is 1 byte
        n1.push(create_io_element(id, value, 1));
    }

    // 2 byte IOs
    if buf.remaining() < 2 { return Err("Not enough bytes for 2-byte IO count".into()); }
    let count_n2 = buf.get_u16();
    for _ in 0..count_n2 {
        if buf.remaining() < 4 { return Err("Not enough bytes for 2-byte IO".into()); }
        let id = buf.get_u16();
        let value = buf.get_i16() as i64; 
        n2.push(create_io_element(id, value, 2));
    }

    // 4 byte IOs
    if buf.remaining() < 2 { return Err("Not enough bytes for 4-byte IO count".into()); }
    let count_n4 = buf.get_u16();
    for _ in 0..count_n4 {
        if buf.remaining() < 6 { return Err("Not enough bytes for 4-byte IO".into()); }
        let id = buf.get_u16();
        let value = buf.get_i32() as i64;
        n4.push(create_io_element(id, value, 4));
    }

    // 8 byte IOs
    if buf.remaining() < 2 { return Err("Not enough bytes for 8-byte IO count".into()); }
    let count_n8 = buf.get_u16();
    for _ in 0..count_n8 {
        if buf.remaining() < 10 { return Err("Not enough bytes for 8-byte IO".into()); }
        let id = buf.get_u16();
        // JS: this.reader.ReadDouble(). It's likely f64.
        // But here we might store as string or handled differently.
        // create_io_element takes i64, so might need adaptation for Double.
        // For now let's assume it fits into something or adapt create_io_element.
        // The JS implementation creates { value: ... } where value is the Double.
        let value_f64 = buf.get_f64();
        let value_i64 = value_f64 as i64;
        let (label, dimension, value_human) = resolve_io_meta(id, value_i64);
        n8.push(IoElement {
            id,
            label,
            value: serde_json::json!(value_f64),
            dimension,
            value_human,
        });
    }

    // X byte IOs
    if buf.remaining() < 2 { return Err("Not enough bytes for X-byte IO count".into()); }
    let count_nx = buf.get_u16();
    for _ in 0..count_nx {
        if buf.remaining() < 4 { return Err("Not enough bytes for X-byte IO header".into()); }
        let id = buf.get_u16();
        let len = buf.get_u16() as usize;
        if buf.remaining() < len { return Err("Not enough bytes for X-byte IO data".into()); }
        let value_bytes = buf.copy_to_bytes(len);
        // Decode as UTF-8 string (ICCID, VIN, etc.), fallback to hex
        let value_str = String::from_utf8(value_bytes.to_vec())
            .unwrap_or_else(|_| hex::encode(&value_bytes));
        nx.push(create_io_element_string(id, value_str));
    }

    Ok(IoGroup { n1, n2, n4, n8, nx })
}

fn resolve_io_meta(id: u16, value: i64) -> (String, Option<String>, Option<String>) {
    let def = get_io_element_definition(id);
    match def {
        Some(d) => {
            let dimension = d.dimension.map(|s| s.to_string());
            let value_human = if let Some(values) = d.values {
                // Only include valueHuman if the key is found in the map
                values.get(&value).map(|v| v.to_string())
            } else {
                // No values map → always include as empty string
                Some("".to_string())
            };
            (d.label.to_string(), dimension, value_human)
        }
        None => (format!("Unknown-{}", id), None, Some("".to_string())),
    }
}

fn create_io_element(id: u16, value: i64, _byte_count: u8) -> IoElement {
    let (label, dimension, value_human) = resolve_io_meta(id, value);
    IoElement {
        id,
        label,
        value: serde_json::Value::Number(serde_json::Number::from(value)),
        dimension,
        value_human,
    }
}

fn create_io_element_string(id: u16, value_str: String) -> IoElement {
    let (label, dimension, value_human) = resolve_io_meta(id, 0);
    IoElement {
        id,
        label,
        value: serde_json::Value::String(value_str),
        dimension,
        value_human,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    #[test]
    fn test_parse_simple_record() {
        let mut data = Vec::new();

        // Timestamp
        data.extend_from_slice(&1700000000000i64.to_be_bytes()); // valid timestamp

        // Priority
        data.push(0);

        // GPS
        // Longitude: 23388880 -> 2.338888
        data.extend_from_slice(&23388880i32.to_be_bytes());
        // Latitude: 48000000 -> 4.800000
        data.extend_from_slice(&48000000i32.to_be_bytes());
        // Altitude: 100
        data.extend_from_slice(&100i16.to_be_bytes());
        // Angle: 0
        data.extend_from_slice(&0i16.to_be_bytes());
        // Satellites: 5
        data.push(5);
        // Speed: 0
        data.extend_from_slice(&0i16.to_be_bytes());

        // Event ID
        data.extend_from_slice(&1u16.to_be_bytes());

        // Properties Count
        data.extend_from_slice(&0u16.to_be_bytes());

        // IO Elements (Counts for N1..NX)
        data.extend_from_slice(&0u16.to_be_bytes()); // N1 count
        data.extend_from_slice(&0u16.to_be_bytes()); // N2 count
        data.extend_from_slice(&0u16.to_be_bytes()); // N4 count
        data.extend_from_slice(&0u16.to_be_bytes()); // N8 count
        data.extend_from_slice(&0u16.to_be_bytes()); // NX count

        let mut buf = Bytes::from(data);
        // We simulate 1 record
        let records = parse(&mut buf, 1).expect("Failed to parse");

        assert_eq!(records.len(), 1);
        let r = &records[0];
        assert_eq!(r.priority, 0);
        assert!((r.gps.longitude - 2.338888).abs() < 1e-6);
        assert!((r.gps.latitude - 4.8).abs() < 1e-6);
        assert_eq!(r.gps.altitude, 100);
        assert_eq!(r.gps.satellites, 5);
        assert_eq!(r.event_id, 1);
        assert!(r.io_elements.is_empty());
    }

    #[test]
    fn test_bot_payload_match() {
        // Full payload from bot.js (after preamble, data_length, codec_id, number_of_data)
        // Original hex: 00000000000000978e01<record_data>0100001e6c
        // We need to extract just the record portion for codec8e::parse
        let full_hex = "0000019c6d352b580000000000000000000000000000000000000018000c00010000150500450000711e00b30000c80300ed0200ef0000f000017f0033d20333d30a0008001100100012ffe00013ffe900430e03004600c700b5000000b6000001820000000300090000003b01c100015040032000000000000000010281001438393838333033303030303038363639393833390100001e6c";
        // Remove trailing: 01 (number_of_data_2) 0000 1e6c (CRC)
        // Actually the record data ends before the trailing number_of_data and CRC
        // Trailing bytes: 01 00001e6c = 5 bytes
        let record_hex = &full_hex[..full_hex.len() - 10]; // remove "0100001e6c"
        let record_bytes = hex::decode(record_hex).expect("Invalid hex");
        let mut buf = Bytes::from(record_bytes);

        let records = parse(&mut buf, 1).expect("Failed to parse");
        assert_eq!(records.len(), 1);

        let json_output = serde_json::to_value(&records).expect("Serialization failed");
        let expected_json: serde_json::Value = serde_json::from_str(r#"[{"gps": {"angle": 0, "speed": 0, "altitude": 0, "latitude": 0, "longitude": 0, "satellites": 0}, "event_id": 0, "ioGroups": {"n1": [{"id": 1, "label": "Digital Input 1", "value": 0, "valueHuman": "0"}, {"id": 21, "label": "GSM Signal", "value": 5, "valueHuman": "5"}, {"id": 69, "label": "GNSS Status", "value": 0, "valueHuman": "OFF"}, {"id": 113, "label": "Internal Battery level", "value": 30, "dimension": "%", "valueHuman": ""}, {"id": 179, "label": "Digital Output", "value": 0, "valueHuman": ""}, {"id": 200, "label": "Sleep Mode", "value": 3}, {"id": 237, "label": "Network Type", "value": 2, "valueHuman": ""}, {"id": 239, "label": "Ignition", "value": 0, "valueHuman": "No"}, {"id": 240, "label": "Movement", "value": 0, "valueHuman": "No"}, {"id": 383, "label": "Accel calibration", "value": 0, "valueHuman": ""}, {"id": 13266, "label": "Current log file", "value": 3, "valueHuman": ""}, {"id": 13267, "label": "Max log file count", "value": 10, "valueHuman": ""}], "n2": [{"id": 17, "label": "Axis X", "value": 16, "dimension": "mg", "valueHuman": ""}, {"id": 18, "label": "Axis Y", "value": -32, "dimension": "mg", "valueHuman": ""}, {"id": 19, "label": "Axis Z", "value": -23, "dimension": "mg", "valueHuman": ""}, {"id": 67, "label": "Internal Battery Voltage", "value": 3587, "dimension": "mV", "valueHuman": ""}, {"id": 70, "label": "PCB temperature", "value": 199, "dimension": "°C", "valueHuman": ""}, {"id": 181, "label": "PDOP", "value": 0, "dimension": "m", "valueHuman": ""}, {"id": 182, "label": "HDOP", "value": 0, "dimension": "m", "valueHuman": ""}, {"id": 386, "label": "Time from last gnss fix", "value": 0, "dimension": "seconds", "valueHuman": ""}], "n4": [{"id": 9, "label": "Analog Input 1", "value": 59, "dimension": "V", "valueHuman": ""}, {"id": 449, "label": "Ignition On Counter", "value": 86080, "dimension": "seconds", "valueHuman": ""}, {"id": 800, "label": "External Voltage", "value": 0, "dimension": "mV", "valueHuman": ""}], "n8": [], "nx": [{"id": 641, "label": "ICCID", "value": "89883030000086699839", "valueHuman": ""}]}, "priority": 0, "timestamp": "2026-02-17T20:05:27.000Z", "ioElements": [], "properties_count": 24}]"#).expect("Invalid expected JSON");

        assert_eq!(json_output, expected_json, "JSON mismatch!\nGot:\n{}\n\nExpected:\n{}",
            serde_json::to_string_pretty(&json_output).unwrap(),
            serde_json::to_string_pretty(&expected_json).unwrap());
    }
}
