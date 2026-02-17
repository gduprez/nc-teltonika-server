use bytes::{Buf, Bytes};
use self::models::AvlData;

pub mod codec8e;
pub mod io_elements;
pub mod models;

pub struct TeltonikaParser {
    pub is_imei: bool,
    pub imei: Option<String>,
    pub avl_data: Option<AvlData>,
    pub invalid: bool,
}

impl TeltonikaParser {
    pub fn new(mut buf: Bytes) -> Self {
        // Check for IMEI
        // IMEI length is first 2 bytes (u16)
        if buf.len() >= 2 {
            let imei_len = u16::from_be_bytes([buf[0], buf[1]]) as usize;
            if imei_len > 0 && buf.len() == imei_len + 2 {
                let imei_bytes = buf.slice(2..2+imei_len);
                let imei = String::from_utf8(imei_bytes.to_vec()).ok();
                if let Some(imei_str) = imei {
                     return TeltonikaParser {
                        is_imei: true,
                        imei: Some(imei_str),
                        avl_data: None,
                        invalid: false,
                    };
                }
            }
        }
        
        // Parse Header
        // Data Length: 4 bytes (i32)
        // Codec ID: 1 byte
        // Number of Data: 1 byte
        if buf.len() < 8 { // Min header size
             return TeltonikaParser { is_imei: false, imei: None, avl_data: None, invalid: true };
        }
        
        // Check zero bytes (4 bytes zeros) at start usually for Data packet?
        // Teltonika Protocol:
        // Preamble (4 bytes of 0)
        // Data Field Length (4 bytes)
        // Codec ID (1 byte)
        // Number of Data 1 (1 byte)
        
        // JS Parser:
        // `parseHeader` reads `ReadInt32()` -> data_length.
        // `ReadBytes(1)` -> codec_id.
        // `ReadBytes(1)` -> number_of_data.
        
        // Wait, does it skip preamble?
        // JS `TeltonikaParser.ts`:
        // `constructor(buffer)` -> `new binutils.BinaryReader(buffer)`.
        // `parseHeader()`: `data_length = this._reader.ReadInt32()`.
        
        // If the packet starts with 0s, ReadInt32 reads 0.
        // But usually data length is not 0.
        // Protocol documentation says: TCP packet starts with 4 zero bytes for AVL data.
        // But the JS parser seems to expect Data Length as the first Int32?
        // Let's assume the buffer PASSED to `TeltonikaParser` in `index.ts` is the full TCP payload.
        // If it starts with 4 zeros, `ReadInt32` will match 0.
        // `data_length` 0?
        
        // Let's check `index.ts`. `socket.on('data', ...)` -> `new TeltonikaParser(buffer)`.
        // If the device sends 0000 000F ... (15 bytes), `ReadInt32` reads 15.
        // So the first 4 bytes ARE the length field if we trust `ReadInt32`.
        // BUT, Teltonika TCP/IP protocol says:
        // "First 4 bytes are 0x00000000".
        // Then "Data Field Length" (4 bytes).
        // Then Codec ID, Count, Data, Count, CRC.
        
        // However, the JS code:
        // `this._avlObj = { data_length: this._reader.ReadInt32(), ... }`
        // If the first 4 bytes were 0, `data_length` would be 0.
        // If `data_length` is 0, logic proceeds?
        // Maybe the device sends WITHOUT preamble if formatted specifically, or binutils handles it?
        // Or `binutils` `ReadInt32` reads 4 bytes.
        // If the packet starts with 0s, then data_length=0.
        // Then codec_id gets read.
        
        // If the JS works, maybe the `buffer` passed in `index.ts` has already stripped 0s?
        // No, `index.ts` uses `net.createServer` and passes `data` directly.
        
        // Wait. Maybe the JS `TeltonikaParser` implies logic I don't see in `binutils`.
        // Or the device IS sending data length as first bytes (UDP packet format?).
        // TCP/IP packet: 00000000 [Length] [Codec] [Count] ...
        // If JS just reads Int32, it reads 0.
        // `codec_id` reading...
        // Let's look at `TeltonikaParser.ts` again.
        // `this._avlObj.data_length = this._reader.ReadInt32()`
        // `this._avlObj.codec_id = this._toInt(this._reader.ReadBytes(1))`
        
        // If first 4 bytes are 0, then codec_id is read from 5th byte.
        // But 5th byte is part of Data Length (MSB).
        // This implies the JS parser expects NO preamble?
        // Or the `buffer` starts at Data Length?
        // "Parser type is unknown; cast to any" in `index.ts`.
        
        // If I assume the JS code works for the user's devices:
        // Then the first 4 bytes must be Data Length.
        // This means the devices are NOT sending the 4 zero bytes preamble?
        // Or they are sending UDP-like packet over TCP?
        // Or maybe `binutils64` `ReadInt32` does something odd.
        
        // I will implement exactly what the JS does:
        // Read 4 bytes -> data_length.
        // Read 1 byte -> codec_id.
        // Read 1 byte -> number_of_data.
        
        let _data_length = buf.get_i32(); // advance 4
        let codec_id = buf.get_u8();
        let number_of_data = buf.get_u8();
        
        if codec_id != 142 {
             // Maybe it skipped 0s and we need to retry?
             // But following JS explicitly:
             println!("Unsupported codec: {}", codec_id);
             return TeltonikaParser { is_imei: false, imei: None, avl_data: None, invalid: true };
        }
        
        let records_res = codec8e::parse(&mut buf, number_of_data);
        match records_res {
            Ok(records) => {
                 TeltonikaParser {
                     is_imei: false,
                     imei: None,
                     avl_data: Some(AvlData {
                         codec_id,
                         number_of_data,
                         records,
                     }),
                     invalid: false,
                 }
            },
            Err(e) => {
                println!("Parser error: {}", e);
                TeltonikaParser { is_imei: false, imei: None, avl_data: None, invalid: true }
            }
        }
    }
}
