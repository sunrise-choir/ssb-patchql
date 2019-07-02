use bytes::{ByteOrder, LittleEndian};

pub fn decode_cursor(encoded: &str) -> Result<i64, String> {
    match base64::decode(encoded) {
        Ok(ref bytes) if bytes.len() < 8 => {
            Err("Error decoding cursor. Is it a valid base64 encoded i64?".to_string())
        }
        Ok(bytes) => Ok(LittleEndian::read_i64(bytes.as_slice())),
        Err(err) => Err(err.to_string()),
    }
}

pub fn encode_cursor(cursor: i64) -> String {
    base64::encode(&(cursor as u64).to_le_bytes())
}
