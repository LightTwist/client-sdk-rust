use std::io::Result;
use chrono::Utc;
use uuid::Uuid;

fn main() {
    let build_uuid = Uuid::new_v4().to_string();
    let utc: chrono::prelude::DateTime<Utc> = Utc::now();
    let formatted_string = utc.format("%Y-%m-%d-%H-%M-%S").to_string();
    println!("cargo:rustc-env=LT_BUILD_UUID={}-{}", build_uuid, formatted_string);
}