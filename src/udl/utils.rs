// pub fn is_optional_type(type_name: &str) -> bool {
//     type_name.contains("^")
// }

use crate::udl::enums::{Enum, EnumKind, EnumVariant};

pub fn is_nullable_type(type_name: &str) -> bool {
    type_name.contains("?")
}

// pub fn is_enum_type(type_name: &str) -> bool {
//     type_name.starts_with("$enum::")
// }

pub fn parse_length_validator(value: &str) -> (isize, isize, isize) {
    if value.contains("...") {
        let parts: Vec<&str> = value.split("...").collect();
        let min = parts[0].parse().unwrap_or(-1);
        let max = parts[1].parse().unwrap_or(-1);
        let default = -1;
        (min, max, default)
    } else {
        let parts: Vec<&str> = value.split("..").collect();
        #[cfg(debug_assertions)]
        println!("{:?}", parts);
        if parts.len() < 3 {
            return if value.starts_with("..") {
                let max = parts[1].parse().unwrap_or(-1);
                let default = max;
                (-1, max, default)
            } else if value.ends_with("..") {
                let min = parts[0].parse().unwrap_or(-1);
                let default = min;
                (min, -1, default)
            } else {
                panic!("Invalid length validator format");
            };
        }
        let min = parts[0].parse().unwrap_or(0);
        let default = parts[1].parse().unwrap();
        let max = parts[2].parse().unwrap_or(-1);
        (min, max, default)
    }
}

pub fn extract_enum_variant<'a>(
    enumm: &'a Enum,
    filter: &str,
) -> Vec<(&'a String, &'a String, &'a Option<String>)> {
    let a = enumm.variants.iter().filter_map(|a| {
        if let EnumKind::Complex(EnumVariant {
            target: Some(str),
            target_field,
            id,
            ..
        }) = a
        {
            if str == filter {
                Some((id, str, target_field))
            } else {
                None
            }
        } else {
            None
        }
    });
    a.into_iter().collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_length_validator() {
        assert_eq!(parse_length_validator("1...10"), (1, 10, -1));
        assert_eq!(parse_length_validator("...10"), (-1, 10, -1));
        assert_eq!(parse_length_validator("10..."), (10, -1, -1));
        assert_eq!(parse_length_validator("1..10..5"), (1, 5, 10));
        assert_eq!(parse_length_validator("1.."), (1, -1, 1));
        assert_eq!(parse_length_validator("..10"), (-1, 10, 10));
    }
}
