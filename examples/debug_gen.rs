use std::fs;

fn main() {
    let json = fs::read_to_string("tests/fixtures/comprehensive_api.json").unwrap();
    let output = oapi_universal_gen::oapi_gen::generate_from_json(&json);

    // Find items_get function
    let lines: Vec<&str> = output.lines().collect();
    let mut in_items = false;
    let mut brace_count = 0;
    for line in &lines {
        if line.contains("fn items_get") {
            in_items = true;
            brace_count = 0;
        }
        if in_items {
            println!("{}", line);
            brace_count += line.chars().filter(|c| *c == '{').count();
            brace_count -= line.chars().filter(|c| *c == '}').count();
            if brace_count == 0 && line.trim() == "}" {
                break;
            }
        }
    }
}
