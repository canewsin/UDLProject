use std::{
    collections::{HashMap, HashSet},
    process::Stdio,
    sync::LazyLock,
};

use crate::udl::{
    LangGenerator, UDL,
    class::{Class, Property},
    enums::{Enum, EnumKind, EnumVariantValue},
    utils::is_nullable_type,
};

pub(crate) struct RustGenerator();

static MAPPINGS: LazyLock<HashMap<&str, &str>> = std::sync::LazyLock::new(|| {
    HashMap::from([
        ("bool", "bool"),
        ("int", "i32"),
        ("float", "f64"),
        ("string", "String"),
        ("isize", "isize"),
        ("int8", "i8"),
        ("int16", "i16"),
        ("int32", "i32"),
        ("int64", "i64"),
        ("usize", "usize"),
        ("uint8", "u8"),
        ("uint16", "u16"),
        ("uint32", "u32"),
        ("uint64", "u64"),
        ("float32", "f32"),
        ("float64", "f64"),
    ])
});

fn clean_type_name(type_name: &str) -> String {
    type_name
        .replace("?", "")
        .replace("^", "")
        .replace("$enum::", "")
}

fn process_type(type_name: &str) -> String {
    let is_nullable = is_nullable_type(type_name);
    let name = clean_type_name(type_name);
    let mapped_type = MAPPINGS.get(name.as_str());
    let prefix = if is_nullable { "Option<" } else { "" };
    let suffix = if is_nullable { ">" } else { "" };
    format!(
        "{}{}{}",
        prefix,
        mapped_type.unwrap_or(&name.as_str()),
        suffix
    )
}

impl LangGenerator for RustGenerator {
    fn extension(&self) -> &str {
        "rs"
    }

    fn gen_enum(&self, enumm: &Enum) -> String {
        let mut code = String::new();
        if let Some(desc) = enumm.description.as_ref() {
            code.push_str(&format!("/// {}\n", desc));
        }
        code.push_str("enum ");
        code.push_str(&enumm.id);
        code.push_str(" {");
        for variant in &enumm.variants {
            //Handle default value
            match variant {
                EnumKind::Simple(name) => {
                    code.push_str(&format!("{},", name));
                }
                EnumKind::Complex(variant) => {
                    if let Some(desc) = variant.description.as_ref() {
                        code.push_str(&format!("/// {}\n", desc));
                    }
                    code.push_str(&format!("{}", variant.id));

                    match &variant.value {
                        EnumVariantValue::Single(_) => {}
                        EnumVariantValue::Multiple(map) => {
                            code.push_str(" {");
                            for (key, value) in map {
                                if MAPPINGS.contains_key(value.as_str()) {
                                    code.push_str(&format!(
                                        "{}: {},",
                                        key,
                                        MAPPINGS[value.as_str()]
                                    ));
                                } else {
                                    code.push_str(&format!("{}: {},", key, value));
                                }
                            }
                            code.push_str("}");
                        }
                    }
                    code.push_str(",");
                }
            }
        }
        code.push_str("}");
        code
    }

    fn gen_class(&self, class: &Class, _error_enum: Option<&Enum>) -> (String, HashSet<String>) {
        let mut code = String::new();
        if let Some(desc) = &class.description {
            code.push_str(&format!("/// {}\n", desc));
        }
        code.push_str(&format!("pub struct {} {{", class.id));
        // code.push_str("\n");
        for (name, ty) in &class.properties {
            match ty {
                Property::Type(ty) => {
                    code.push_str(&format!("    pub {}: {},", name, process_type(ty)));
                }
                Property::Map(map) => {
                    if let Some(desc) = &map.get("description") {
                        code.push_str(&format!("/// {}\n", desc));
                    }
                    let private = map.get("private") == Some(&String::from("true"));
                    code.push_str(&format!(
                        "    {} {}: {},",
                        if private { "" } else { "pub" },
                        name,
                        process_type(&map["type"])
                    ));
                }
            }
        }
        code.push_str(&format!("}}\n"));
        (code, HashSet::new())
    }

    fn generate(&self, udl: &UDL) -> String {
        let mut code = String::new();
        for enum_def in &udl.enums {
            code.push_str(&self.gen_enum(enum_def));
            code.push_str("\n\n");
        }

        for class in &udl.models {
            let error_enum = udl
                .enums
                .iter()
                .find(|e| e.id == class.clone().error.unwrap_or_default());

            code.push_str(&self.gen_class(class, error_enum).0);
            code.push_str("\n\n");
        }

        code
    }

    fn format(&self, path: &str) {
        std::process::Command::new("rustfmt")
            .arg(path)
            .stdout(Stdio::piped())
            .spawn()
            .expect("");
    }
}
