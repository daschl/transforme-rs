use serde_json::Value;

pub fn transform(doc: Value, from: Value, to: Value) -> Value {
    json!({"hello": ["world"]})
}