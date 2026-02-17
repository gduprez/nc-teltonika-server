use super::parser::models::AvlRecord;

pub fn format_record(record: &AvlRecord) -> String {
    let mut log = format!(
		"📍 Record Timestamp: {}\n",
		record.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
	);
    log += &format!("  Priority: {}\n", record.priority);
    log += &format!(
        "  GPS: lat={:.6}, lon={:.6}, alt={}m, speed={}km/h, sats={}\n",
        record.gps.latitude, record.gps.longitude, record.gps.altitude, record.gps.speed, record.gps.satellites
    );
    log += &format!("  Event IO ID: {}\n", record.event_id);
    
    let total = record.io_groups.n1.len() + record.io_groups.n2.len() + record.io_groups.n4.len() + record.io_groups.n8.len() + record.io_groups.nx.len();
    log += &format!("  IO count total: {}\n", total);
    
    log += &format!("  N1 count: {}\n", record.io_groups.n1.len());
    for io in &record.io_groups.n1 {
        let val = if !io.dimension.is_empty() { format!("{} {}", io.value, io.dimension) } else { format!("{} -", io.value) };
        log += &format!("    {} (ID={}) -> {}\n", io.label, io.id, val);
    }
    
    log += &format!("  N2 count: {}\n", record.io_groups.n2.len());
    for io in &record.io_groups.n2 {
        let val = if !io.dimension.is_empty() { format!("{} {}", io.value, io.dimension) } else { io.value.clone() };
        log += &format!("    {} (ID={}) -> {}\n", io.label, io.id, val);
    }
    
    log += &format!("  N4 count: {}\n", record.io_groups.n4.len());
    for io in &record.io_groups.n4 {
        let val = if !io.dimension.is_empty() { format!("{} {}", io.value, io.dimension) } else { io.value.clone() };
        log += &format!("    {} (ID={}) -> {}\n", io.label, io.id, val);
    }
    
    log += &format!("  N8 count: {}\n", record.io_groups.n8.len());
    for io in &record.io_groups.n8 {
        let val = if !io.dimension.is_empty() { format!("{} {}", io.value, io.dimension) } else { io.value.clone() };
        log += &format!("    {} (ID={}) -> {}\n", io.label, io.id, val);
    }
    
    log += &format!("  NX count: {}\n", record.io_groups.nx.len());
    for io in &record.io_groups.nx {
        log += &format!("    {} (NX ID={}): {}\n", io.label, io.id, io.value);
    }
    
    log += "  ---\n";
    log
}
