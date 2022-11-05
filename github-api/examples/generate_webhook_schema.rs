use std::{fs, path::Path, time::Instant};

use schemars::schema::Schema;
use typify::TypeSpace;

fn main() {
    let t = Instant::now();
    let elapsed = || {
        let millis = t.elapsed().as_millis();
        millis as f64 * 0.001
    };
    let content = std::fs::read_to_string("./github/swagger/webhooks_schema.json").unwrap_or_else(|_| {
        println!("Error loading schema file");
        std::process::exit(1);
    });
    let schema = serde_json::from_str::<schemars::schema::RootSchema>(&content).unwrap_or_else(|e| {
        println!("Error deserializing schema.\n{}", e);
        std::process::exit(2);
    });

    println!(
        "JSON schema loaded and deserialized. {} kB ({:.2}s)",
        content.len() / 1024,
        elapsed()
    );
    let mut type_space = TypeSpace::default();
    type_space.add_ref_types(schema.definitions).unwrap_or_else(|e| {
        println!("Error adding reference types.\n{}", e);
        std::process::exit(3);
    });

    println!("Reference types added. ({:.2} s)", elapsed());
    let base_type = &schema.schema;
    // Only convert the top-level type if it has a name
    if (|| base_type.metadata.as_ref()?.title.as_ref())().is_some() {
        let _ = type_space.add_type(&Schema::Object(schema.schema)).unwrap();
    }

    println!("Other types added. ({:.2} s)", elapsed());
    let content = format!("{}\n{}", "use serde::{Deserialize, Serialize};", type_space.to_string());
    println!("Converted to Rust. ({:.2} s)", elapsed());
    let mut out_file = Path::new("./github/swagger/").to_path_buf();
    out_file.push("webhooks_codegen.rs");
    fs::write(out_file, content).unwrap();
}
