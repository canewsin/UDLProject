use crate::udl::enums::{Enum, EnumKind, EnumVariant};

// pub fn is_optional_type(type_name: &str) -> bool {
//     type_name.contains("^")
// }

pub fn is_nullable_type(type_name: &str) -> bool {
    type_name.contains("?")
}

// pub fn is_enum_type(type_name: &str) -> bool {
//     type_name.starts_with("$enum::")
// }

pub fn parse_limit_validator(value: &str) -> (isize, isize, isize) {
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
                panic!("Invalid limit validator format");
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
    let f = enumm.variants.iter().filter_map(|v| {
        if let EnumKind::Complex(EnumVariant {
            target: Some(str),
            target_field,
            id,
            ..
        }) = v
        {
            if str == filter {
                return Some((id, str, target_field));
            }
            return None;
        };
        None
    });
    f.into_iter().collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_nullable() {
        assert!(is_nullable_type("String?"));
        assert!(!is_nullable_type("String"));
        assert!(is_nullable_type("int?^"));
        assert!(!is_nullable_type("bool^"));
    }

    #[test]
    fn test_parse_limit_validator() {
        assert_eq!(parse_limit_validator("1...10"), (1, 10, -1));
        assert_eq!(parse_limit_validator("...10"), (-1, 10, -1));
        assert_eq!(parse_limit_validator("10..."), (10, -1, -1));
        assert_eq!(parse_limit_validator("1..10..5"), (1, 5, 10));
        assert_eq!(parse_limit_validator("1.."), (1, -1, 1));
        assert_eq!(parse_limit_validator("..10"), (-1, 10, 10));
    }
}
