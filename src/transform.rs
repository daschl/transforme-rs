use serde_json::Value;

pub fn transform(mut document: Value, new_schema: Value) -> Value {
    let doc = document.as_object_mut().expect("Expected doc as object");
    let schema = new_schema.as_object().expect("Expected schema as object");

    for (field, spec) in schema {

        // Trans 1: If the schema does contain a field but the doc does not, add it
        // with a default value of null
        if !doc.contains_key(field) {
            doc.insert(field.clone(), Value::Null);
        }
    }

    document
}