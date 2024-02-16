pub fn numberify(query: String, parameters: Vec<String>) -> Result<String, String> {
    let mut input = query.chars().peekable();
    let mut output = String::new();

    loop {
        match input.next() {
            None => return Ok(output),
            Some('@') => {
                match input.peek() {
                    None => return Err("Input SQL should not end with '@'. If a single literal '@' is desired, it should be doubled ('@@')".to_owned()),
                    Some('@') => {
                        // escaped '@' (i.e. '@@' -> '@')
                        output.push(input.next().unwrap())
                    },
                    Some(_) => {
                        let mut field_name = String::new();

                        while let Some(ch) = input.peek() {
                            if ch.is_alphanumeric() || *ch == '_' {
                                field_name.push(input.next().unwrap());
                            } else {
                                break;
                            }
                        }

                        match parameters.iter().position(|field| field_name == *field) {
                            Some(raw_index) => {
                                let sql_index = raw_index + 1;
                                output.push_str(&format!("${}", sql_index));
                            },
                            None => return Err(format!(
                                r#"The provided SQL contains "@{}", but there is no matching field in the struct with the name "{}""#,
                                field_name,
                                field_name,
                            ))
                        }
                    }
                }
            },

            // regular characters not in a field name pass through to the output
            Some(ch) => output.push(ch)
        }
    }
}