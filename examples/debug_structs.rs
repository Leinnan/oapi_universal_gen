use std::fs;

fn main() {
    let json = fs::read_to_string("tests/fixtures/opencode.json").unwrap();
    let output = oapi_universal_gen::oapi_gen::generate_from_json(&json);

    // Find SessionStatus enum
    let lines: Vec<&str> = output.lines().collect();
    let mut in_enum = false;
    let mut brace_count = 0;
    for line in &lines {
        if line.contains("pub enum SessionStatus") {
            in_enum = true;
            brace_count = 0;
        }
        if in_enum {
            println!("{}", line);
            brace_count += line.matches('{').count();
            brace_count += line.matches('(').count();
            brace_count -= line.matches('}').count();
            brace_count -= line.matches(')').count();
            if brace_count == 0 && (line.contains('}') || line.contains(')')) {
                break;
            }
        }
    }
}
