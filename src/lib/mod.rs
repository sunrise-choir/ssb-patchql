use serde_json::Value;

pub fn find_values_in_object_by_key<'a>(
    obj: &'a serde_json::Value,
    key: &str,
    values: &mut Vec<&'a serde_json::Value>,
) {
    if let Some(val) = obj.get(key) {
        values.push(val)
    }

    match obj {
        Value::Array(arr) => {
            for val in arr {
                find_values_in_object_by_key(val, key, values);
            }
        }
        Value::Object(kv) => {
            for val in kv.values() {
                match val {
                    Value::Object(_) => find_values_in_object_by_key(val, key, values),
                    Value::Array(_) => find_values_in_object_by_key(val, key, values),
                    _ => (),
                }
            }
        }
        _ => (),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SsbValue {
    pub author: String,
    pub sequence: u32,
    pub timestamp: f64,
    pub content: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SsbMessage {
    pub key: String,
    pub value: SsbValue,
    pub timestamp: f64,
}
