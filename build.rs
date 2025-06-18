use std::fs::File;
use std::io::Write;

fn main() {
    // Ensure codegen only triggers when relevant files change
    println!("cargo:rerun-if-changed=src/openapi.rs");

    // Use your project's OpenAPI struct here
    let openapi = jsonld_signer::openapi::ApiDoc::openapi(); // <--- UPDATE with your crate name
    let yaml = serde_yaml::to_string(&openapi).expect("Failed to serialize OpenAPI spec");

    let mut file = File::create("doc/openapi.yaml").expect("Could not create openapi.yaml");
    file.write_all(yaml.as_bytes()).expect("Failed to write OpenAPI YAML");

    println!("✅ OpenAPI YAML written to ./doc/openapi.yaml");
}
