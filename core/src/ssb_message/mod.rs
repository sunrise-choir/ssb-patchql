use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SsbValue {
    pub author: String,
    pub sequence: u32,
    pub timestamp: f64,
    pub content: Value,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SsbMessage {
    pub key: String,
    pub value: SsbValue,
    pub timestamp: f64,
}
