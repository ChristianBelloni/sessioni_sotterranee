use api::ws::models::*;
use schemars::schema_for;

fn main() {
    let schema = schema_for!(WebSocketMessage);
    let data = serde_json::to_string_pretty(&schema).unwrap();
    println!("{data}")
}
