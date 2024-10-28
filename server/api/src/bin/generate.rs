use std::{
    io::{Read, Write},
    process::Stdio,
};

use api::models::{CreateUser, User};
use schemars::schema_for;

macro_rules! schemas {
    ($ty:ty) => {{
        let schema = schema_for!($ty);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }};
    ($($tys:ident),+) => {
        #[allow(non_snake_case)]
        #[derive(schemars::JsonSchema)]
        pub struct Root {
            $(pub $tys: $tys,)*
        }
        fn get_schema() -> String {
            let schema = schema_for!(Root);
            return format!("{}", serde_json::to_string_pretty(&schema).unwrap());
        }
    };
}

schemas!(User, CreateUser);

fn main() {
    let schema = get_schema();

    let process = std::process::Command::new("json2ts")
        .arg("--unreachableDefinitions")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    process
        .stdin
        .as_ref()
        .unwrap()
        .write_all(schema.as_bytes())
        .unwrap();

    let out = process.wait_with_output().unwrap().stdout;

    println!("{}", String::from_utf8_lossy(&out));
}
