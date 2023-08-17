pub fn do_vecs_match<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    let matching = a.iter().zip(b).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}

pub fn get_full_type(input: &str) -> &str {
    input.split_whitespace().collect::<Vec<&str>>()[0]
}

pub fn parse_array_size(input: &str) -> (String, u32, u32) {
    let mut parts = input.splitn(2, '[');
    let type_name = parts.next().unwrap().to_string();
    let remainder = parts.next();

    let (type_name, type_size, array_len) = match remainder {
        Some(rem) => {
            let array_size: u32 = rem.trim_end_matches(']').parse().unwrap();

            let (type_name, type_size) = parse_type_size(&type_name);
            (type_name, type_size, array_size)
        }
        None => {
            let (type_name, type_size) = parse_type_size(&type_name);
            (type_name, type_size, 0)
        }
    };

    (type_name, type_size, array_len)
}

pub fn parse_type_size(input: &str) -> (String, u32) {
    let (type_name, type_size) = split_type_and_size(input);
    match type_name {
        "bool" => ("bool".to_string(), 1),
        "address" => ("address".to_string(), 20),
        "bytes" => {
            if type_size.is_none() {
                ("bytes".to_string(), 32)
            } else {
                ("bytes".to_string(), type_size.unwrap())
            }
        }
        "string" => ("string".to_string(), 32),
        "uint" => {
            if type_size.is_none() {
                ("uint".to_string(), 32)
            } else {
                ("uint".to_string(), type_size.unwrap() / 8)
            }
        }
        "int" => {
            if type_size.is_none() {
                ("int".to_string(), 32)
            } else {
                ("int".to_string(), type_size.unwrap() / 8)
            }
        }
        _ => (type_name.to_string(), 32),
    }
}

pub fn split_type_and_size(input: &str) -> (&str, Option<u32>) {
    let letter_end = input.chars().position(|c| c.is_digit(10));

    if let Some(letter_end) = letter_end {
        let (letters, numbers) = input.split_at(letter_end);
        let number: u32 = numbers.parse().unwrap();
        (letters, Some(number))
    } else {
        (input, None)
    }
}
