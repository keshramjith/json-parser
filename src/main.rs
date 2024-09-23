fn main() {
    let input_json_string = "{}";

    println!("{}", validate_json_string(input_json_string.to_string()))
}

fn validate_json_string(input: String) -> bool {
    let mut stck = Vec::new();

    let mut is_valid_json = false;

    for c in input.chars() {
        match c {
            '{' => stck.push(c),
            '}' => {
                let last_value = stck.pop();
                match last_value {
                    None => {}
                    Some(x) => {
                        if x == '{' {
                            is_valid_json = true
                        }
                    }
                }
            }
            _ => {}
        }
    }

    is_valid_json
}
