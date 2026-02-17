use bytes::{Buf, Bytes};
use self::models::AvlData;
use tracing::{warn, error};

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
        
        let zeros = buf.slice(0..4);
        if zeros.as_ref() == [0, 0, 0, 0] {
             let _preamble = buf.get_u32(); // consume 0000
             if buf.len() < 4 {
                 return TeltonikaParser { is_imei: false, imei: None, avl_data: None, invalid: true };
             }
        }
        
        let _data_length = buf.get_u32(); // advance 4
        let codec_id = buf.get_u8();
        let number_of_data = buf.get_u8();
        
        if codec_id != 142 {
             // Maybe it skipped 0s and we need to retry?
             // But following JS explicitly:
             warn!("Unsupported codec: {}", codec_id);
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
                error!("Parser error: {}", e);
                TeltonikaParser { is_imei: false, imei: None, avl_data: None, invalid: true }
            }
        }
    }
}

