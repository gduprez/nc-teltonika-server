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
        // We'll handle this separately or change create_io_element to take a generic or enum.
        // IoElement struct has value: String.
        // So we can convert to string here.
        n8.push(create_io_element_from_string(id, value_f64.to_string(), value_f64));
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
        let value_hex = hex::encode(value_bytes);
        nx.push(create_io_element_from_string(id, value_hex.clone(), 0.0)); // 0.0 payload for now, mostly string matters
    }

    Ok(IoGroup { n1, n2, n4, n8, nx })
}

fn create_io_element(id: u16, value: i64, _byte_count: u8) -> IoElement {
     let def = get_io_element_definition(id);
     let (label, dimension, value_human) = match def {
         Some(d) => {
             let val_human = if let Some(values) = d.values {
                 values.get(&value).unwrap_or(&"").to_string()
             } else {
                 "".to_string()
             };
             (d.label.to_string(), d.dimension.unwrap_or("").to_string(), val_human)
         },
         None => (format!("Unknown-{}", id), "".to_string(), "".to_string()),
     };

     IoElement {
         id,
         value: value.to_string(),
         label,
         dimension,
         value_human,
     }
}

fn create_io_element_from_string(id: u16, value_str: String, value_num: f64) -> IoElement {
    // Similar to above but value is already string.
    // For n8 (double), we might need to look up values using the numeric value cast to i64?
    // JS `ReadDouble` returns a number. `values[value]` access suggests it checks exact match.
    // If double is 1.0, it matches 1.
    
    let def = get_io_element_definition(id);
     let (label, dimension, value_human) = match def {
         Some(d) => {
             let val_fixed = value_num as i64; // rough cast
             let val_human = if let Some(values) = d.values {
                 // Check if float is close to int?
                 values.get(&val_fixed).unwrap_or(&"").to_string()
             } else {
                 "".to_string()
             };
             (d.label.to_string(), d.dimension.unwrap_or("").to_string(), val_human)
         },
         None => (format!("Unknown-{}", id), "".to_string(), "".to_string()),
     };

     IoElement {
         id,
         value: value_str,
         label,
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
    }
}
