use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct IoElementDefinition {
    pub label: &'static str,
    pub dimension: Option<&'static str>,
    pub values: Option<HashMap<i64, &'static str>>,
}

pub fn get_io_element_definition(id: u16) -> Option<IoElementDefinition> {
    match id {
        1 => Some(IoElementDefinition {
            label: "Digital Input 1",
            dimension: None,
            values: Some(HashMap::from([(0, "0"), (1, "1")])),
        }),
        4 => Some(IoElementDefinition {
            label: "Pulse counter DIN1",
            dimension: None,
            values: None,
        }),
        9 => Some(IoElementDefinition {
            label: "Analog Input 1",
            dimension: Some("V"),
            values: None,
        }),
        10 => Some(IoElementDefinition {
            label: "SD Status",
            dimension: None,
            values: Some(HashMap::from([(0, "Not present"), (1, "Present")])),
        }),
        11 => Some(IoElementDefinition {
            label: "SIM ICCID1 number",
            dimension: None,
            values: None,
        }),
        12 => Some(IoElementDefinition {
            label: "Fuel Used GPS",
            dimension: None,
            values: None,
        }),
        13 => Some(IoElementDefinition {
            label: "Average Fuel Use",
            dimension: Some("L / 100 km"),
            values: None,
        }),
        14 => Some(IoElementDefinition {
            label: "SIM ICCID2 number",
            dimension: None,
            values: None,
        }),
        15 => Some(IoElementDefinition {
            label: "Eco Score",
            dimension: None,
            values: None,
        }),
        16 => Some(IoElementDefinition {
            label: "Total Odometer",
            dimension: Some("m"),
            values: None,
        }),
        17 => Some(IoElementDefinition {
            label: "Axis X",
            dimension: Some("mg"),
            values: None,
        }),
        18 => Some(IoElementDefinition {
            label: "Axis Y",
            dimension: Some("mg"),
            values: None,
        }),
        19 => Some(IoElementDefinition {
            label: "Axis Z",
            dimension: Some("mg"),
            values: None,
        }),
        20 => Some(IoElementDefinition {
            label: "BLE 2 Battery Voltage",
            dimension: Some("%"),
            values: None,
        }),
        21 => Some(IoElementDefinition {
            label: "GSM Signal",
            dimension: None,
            values: Some(HashMap::from([(1, "1"), (2, "2"), (3, "3"), (4, "4"), (5, "5")])),
        }),
        22 => Some(IoElementDefinition {
            label: "BLE 3 Battery Voltage",
            dimension: Some("%"),
            values: None,
        }),
        23 => Some(IoElementDefinition {
            label: "BLE 4 Battery Voltage",
            dimension: Some("%"),
            values: None,
        }),
        24 => Some(IoElementDefinition {
            label: "Speed",
            dimension: Some("km/h"),
            values: None,
        }),
        25 => Some(IoElementDefinition {
            label: "BLE 1 Temperature",
            dimension: Some("C"),
            values: None,
        }),
        26 => Some(IoElementDefinition {
            label: "BLE 2 Temperature",
            dimension: Some("C"),
            values: None,
        }),
        27 => Some(IoElementDefinition {
            label: "BLE 3 Temperature",
            dimension: Some("C"),
            values: None,
        }),
        28 => Some(IoElementDefinition {
            label: "BLE 4 Temperature",
            dimension: Some("C"),
            values: None,
        }),
        29 => Some(IoElementDefinition {
            label: "BLE 1 Battery Voltage",
            dimension: Some("%"),
            values: None,
        }),
        30 => Some(IoElementDefinition {
            label: "Number of DTC",
            dimension: None,
            values: None,
        }),
        31 => Some(IoElementDefinition {
            label: "Calculated engine load value",
            dimension: Some("%"),
            values: None,
        }),
        32 => Some(IoElementDefinition {
            label: "Engine coolant temperature",
            dimension: Some("C"),
            values: None,
        }),
        33 => Some(IoElementDefinition {
            label: "Short term fuel trim 1",
            dimension: Some("%"),
            values: None,
        }),
        34 => Some(IoElementDefinition {
            label: "Fuel pressure",
            dimension: Some("kPa"),
            values: None,
        }),
        35 => Some(IoElementDefinition {
            label: "Intake manifold absolute pressure",
            dimension: Some("kPa"),
            values: None,
        }),
        36 => Some(IoElementDefinition {
            label: "Engine RPM",
            dimension: Some("rpm"),
            values: None,
        }),
        37 => Some(IoElementDefinition {
            label: "Vehicle speed",
            dimension: Some("km/h"),
            values: None,
        }),
        38 => Some(IoElementDefinition {
            label: "Timing advance",
            dimension: Some("O"),
            values: None,
        }),
        39 => Some(IoElementDefinition {
            label: "Intake air temperature",
            dimension: Some("C"),
            values: None,
        }),
        40 => Some(IoElementDefinition {
            label: "MAF air flow rate",
            dimension: Some("g/sec, *0.01"),
            values: None,
        }),
        41 => Some(IoElementDefinition {
            label: "Throttle position",
            dimension: Some("%"),
            values: None,
        }),
        42 => Some(IoElementDefinition {
            label: "Run time since engine start",
            dimension: Some("s"),
            values: None,
        }),
        43 => Some(IoElementDefinition {
            label: "Distance traveled MIL on",
            dimension: Some("Km"),
            values: None,
        }),
        44 => Some(IoElementDefinition {
            label: "Relative fuel rail pressure",
            dimension: Some("kPa*0.1"),
            values: None,
        }),
        45 => Some(IoElementDefinition {
            label: "Direct fuel rail pressure",
            dimension: Some("kPa*0.1"),
            values: None,
        }),
        46 => Some(IoElementDefinition {
            label: "Commanded EGR",
            dimension: Some("%"),
            values: None,
        }),
        47 => Some(IoElementDefinition {
            label: "EGR error",
            dimension: Some("%"),
            values: None,
        }),
        48 => Some(IoElementDefinition {
            label: "Fuel level",
            dimension: Some("%"),
            values: None,
        }),
        49 => Some(IoElementDefinition {
            label: "Distance traveled since codes cleared",
            dimension: Some("Km"),
            values: None,
        }),
        50 => Some(IoElementDefinition {
            label: "Barometric pressure",
            dimension: Some("kPa"),
            values: None,
        }),
        51 => Some(IoElementDefinition {
            label: "Control module voltage",
            dimension: Some("mV"),
            values: None,
        }),
        52 => Some(IoElementDefinition {
            label: "Absolute load value",
            dimension: Some("%"),
            values: None,
        }),
        53 => Some(IoElementDefinition {
            label: "Ambient air temperature",
            dimension: Some("C"),
            values: None,
        }),
        54 => Some(IoElementDefinition {
            label: "Time run with MIL on",
            dimension: Some("min"),
            values: None,
        }),
        55 => Some(IoElementDefinition {
            label: "Time since trouble codes cleared",
            dimension: Some("min"),
            values: None,
        }),
        56 => Some(IoElementDefinition {
            label: "Absolute fuel rail pressure",
            dimension: Some("kPa*10"),
            values: None,
        }),
        57 => Some(IoElementDefinition {
            label: "Hybrid battery pack remaining life",
            dimension: Some("%"),
            values: None,
        }),
        58 => Some(IoElementDefinition {
            label: "Engine oil temperature",
            dimension: Some("C"),
            values: None,
        }),
        59 => Some(IoElementDefinition {
            label: "Fuel injection timing",
            dimension: Some("O, *0.01"),
            values: None,
        }),
        60 => Some(IoElementDefinition {
            label: "Engine fuel rate",
            dimension: Some("L/h, *100"),
            values: None,
        }),
        66 => Some(IoElementDefinition {
            label: "Ext Voltage",
            dimension: Some("mV"),
            values: None,
        }),
        67 => Some(IoElementDefinition {
            label: "Internal Battery Voltage",
            dimension: Some("mV"),
            values: None,
        }),
        68 => Some(IoElementDefinition {
            label: "Internal Battery Current",
            dimension: Some("mA"),
            values: None,
        }),
        69 => Some(IoElementDefinition {
            label: "GNSS Status",
            dimension: None,
            values: Some(HashMap::from([(0, "OFF"), (1, "ON with fix"), (2, "ON without fix"), (3, "In sleep state")])),
        }),
        70 => Some(IoElementDefinition {
            label: "PCB temperature",
            dimension: Some("deg C"),
            values: None,
        }),
        80 => Some(IoElementDefinition {
            label: "Data Mode",
            dimension: None,
            values: Some(HashMap::from([(0, "Home On Stop"), (1, "Home On Moving"), (2, "Roaming On Stop"), (3, "Roaming On Moving"), (4, "Unknown On Stop"), (5, "Unknown On Moving")])),
        }),
        86 => Some(IoElementDefinition {
            label: "BLE 1 Humidity",
            dimension: Some("%RH"),
            values: None,
        }),
        104 => Some(IoElementDefinition {
            label: "BLE 2 Humidity",
            dimension: Some("%RH"),
            values: None,
        }),
        106 => Some(IoElementDefinition {
            label: "BLE 3 Humidity",
            dimension: Some("%RH"),
            values: None,
        }),
        108 => Some(IoElementDefinition {
            label: "BLE 4 Humidity",
            dimension: Some("%RH"),
            values: None,
        }),
        113 => Some(IoElementDefinition {
            label: "Internal Battery level",
            dimension: Some("%"),
            values: None,
        }),
        175 => Some(IoElementDefinition {
            label: "Auto geofence",
            dimension: None,
            values: None,
        }),
        179 => Some(IoElementDefinition {
            label: "Digital Output",
            dimension: None,
            values: None,
        }),
        181 => Some(IoElementDefinition {
            label: "PDOP",
            dimension: Some("m"),
            values: None,
        }),
        182 => Some(IoElementDefinition {
            label: "HDOP",
            dimension: Some("m"),
            values: None,
        }),
        199 => Some(IoElementDefinition {
            label: "Trip Odometer",
            dimension: Some("m"),
            values: None,
        }),
        200 => Some(IoElementDefinition {
            label: "Sleep Mode",
            dimension: None,
            values: Some(HashMap::from([(0, "No Sleep"), (1, "GPS Sleep"), (2, "Deep Sleep")])),
        }),
        205 => Some(IoElementDefinition {
            label: "GSM Cell ID",
            dimension: None,
            values: None,
        }),
        206 => Some(IoElementDefinition {
            label: "GSM Area Code",
            dimension: None,
            values: None,
        }),
        237 => Some(IoElementDefinition {
            label: "Network Type",
            dimension: None,
            values: None,
        }),
        238 => Some(IoElementDefinition {
            label: "User ID",
            dimension: None,
            values: None,
        }),
        239 => Some(IoElementDefinition {
            label: "Ignition",
            dimension: None,
            values: Some(HashMap::from([(0, "No"), (1, "Yes")])),
        }),
        240 => Some(IoElementDefinition {
            label: "Movement",
            dimension: None,
            values: Some(HashMap::from([(0, "No"), (1, "Yes")])),
        }),
        241 => Some(IoElementDefinition {
            label: "GSM Operator",
            dimension: None,
            values: None,
        }),
        243 => Some(IoElementDefinition {
            label: "Green Driving Event Duration",
            dimension: Some("ms"),
            values: None,
        }),
        246 => Some(IoElementDefinition {
            label: "Towing Detection",
            dimension: None,
            values: Some(HashMap::from([(1, "Towing detected")])),
        }),
        247 => Some(IoElementDefinition {
            label: "Crash Detection",
            dimension: None,
            values: Some(HashMap::from([(1, "Crash Detected"), (2, "Crash Trace Record"), (3, "Crash trace record(calibrated)")])),
        }),
        249 => Some(IoElementDefinition {
            label: "Jamming Detection",
            dimension: None,
            values: Some(HashMap::from([(0, "Jamming Ended"), (1, "Jamming Detected")])),
        }),
        250 => Some(IoElementDefinition {
            label: "Trip Event",
            dimension: None,
            values: Some(HashMap::from([(0, "Trip Ended"), (1, "Trip Started"), (2, "Business Status"), (3, "Private Status"), (4, "Custom Statuses"), (5, "Custom Statuses"), (6, "Custom Statuses"), (7, "Custom Statuses"), (8, "Custom Statuses"), (9, "Custom Statuses")])),
        }),
        251 => Some(IoElementDefinition {
            label: "Idling Event",
            dimension: None,
            values: Some(HashMap::from([(0, "Idling ended event"), (1, "Idling started event")])),
        }),
        252 => Some(IoElementDefinition {
            label: "Unplug Event",
            dimension: None,
            values: Some(HashMap::from([(1, "Send when unplug event happens")])),
        }),
        253 => Some(IoElementDefinition {
            label: "Green Driving Type",
            dimension: None,
            values: Some(HashMap::from([(1, "Acceleration"), (2, "Braking"), (3, "Cornering")])),
        }),
        254 => Some(IoElementDefinition {
            label: "Eco driving value",
            dimension: Some("G/rad"),
            values: None,
        }),
        255 => Some(IoElementDefinition {
            label: "Overspeeding Event",
            dimension: Some("km/h"),
            values: None,
        }),
        256 => Some(IoElementDefinition {
            label: "VIN",
            dimension: None,
            values: None,
        }),
        281 => Some(IoElementDefinition {
            label: "fault codes",
            dimension: None,
            values: None,
        }),
        303 => Some(IoElementDefinition {
            label: "Instant Movement",
            dimension: None,
            values: None,
        }),
        358 => Some(IoElementDefinition {
            label: "Custom scenario 1",
            dimension: None,
            values: None,
        }),
        359 => Some(IoElementDefinition {
            label: "Custom scenario 2",
            dimension: None,
            values: None,
        }),
        360 => Some(IoElementDefinition {
            label: "Custom scenario 3",
            dimension: None,
            values: None,
        }),
        641 => Some(IoElementDefinition {
            label: "ICCID",
            dimension: None,
            values: None,
        }),
        800 => Some(IoElementDefinition {
            label: "External Voltage",
            dimension: Some("mV"),
            values: None,
        }),
        841 => Some(IoElementDefinition {
            label: "Digital Output 1 Overcurrent",
            dimension: None,
            values: None,
        }),
        1148 => Some(IoElementDefinition {
            label: "Connectivity quality",
            dimension: Some("dBm"),
            values: None,
        }),
        1429 => Some(IoElementDefinition {
            label: "Crash average vector",
            dimension: Some("mG"),
            values: None,
        }),
        1432 => Some(IoElementDefinition {
            label: "Crash max vector",
            dimension: Some("mG"),
            values: None,
        }),
        257 => Some(IoElementDefinition {
            label: "Crash trace data",
            dimension: None,
            values: None,
        }),
        383 => Some(IoElementDefinition {
            label: "Accel calibration",
            dimension: None,
            values: None,
        }),
        386 => Some(IoElementDefinition {
            label: "Time from last gnss fix",
            dimension: Some("seconds"),
            values: None,
        }),
        449 => Some(IoElementDefinition {
            label: "Ignition On Counter",
            dimension: Some("seconds"),
            values: None,
        }),
        13266 => Some(IoElementDefinition {
            label: "Current log file",
            dimension: None,
            values: None,
        }),
        13267 => Some(IoElementDefinition {
            label: "Max log file count",
            dimension: None,
            values: None,
        }),
        _ => None,
    }
}
