fn main() {
    let json_content = include_str!("../fixtures/opencode.json");

    let generated_code = oapi_universal_gen::oapi_gen::generate_from_json(json_content);

    let out_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("src/generated_api.rs");
    std::fs::write(&dest_path, &generated_code).expect("Failed to write generated_api.rs");
    println!("cargo:warning=Code saved to {}", dest_path.display());

    println!("cargo:rerun-if-changed=../opencode.json");
}
