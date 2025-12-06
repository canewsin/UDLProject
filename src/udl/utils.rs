// pub fn is_optional_type(type_name: &str) -> bool {
//     type_name.contains("^")
// }

pub fn is_nullable_type(type_name: &str) -> bool {
    type_name.contains("?")
}

// pub fn is_enum_type(type_name: &str) -> bool {
//     type_name.starts_with("$enum::")
// }
