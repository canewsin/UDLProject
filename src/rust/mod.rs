use std::{collections::HashMap, process::Stdio, sync::LazyLock};

use crate::udl::{
    LangGenerator, UDL,
    core::{Enum, EnumKind, EnumVariantValue},
};

pub(crate) struct RustGenerator();

static MAPPINGS: LazyLock<HashMap<&str, &str>> = std::sync::LazyLock::new(|| {
    HashMap::from([
        ("bool", "bool"),
        ("int8", "i8"),
        ("int16", "i16"),
        ("int32", "i32"),
        ("int64", "i64"),
        ("uint8", "u8"),
        ("uint16", "u16"),
        ("uint32", "u32"),
        ("uint64", "u64"),
        ("float32", "f32"),
        ("float64", "f64"),
        ("string", "String"),
    ])
});

impl LangGenerator for RustGenerator {
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

    fn generate(&self, udl: &UDL) -> String {
        let mut code = String::new();
        code.push_str(&self.gen_enum(&udl.enums.iter().skip(1).next().unwrap()));
        code
    }

    fn format(&self, path: &str) {
        //Run rustfmt on the code
        std::process::Command::new("rustfmt")
            .arg(path)
            .stdout(Stdio::piped())
            .spawn()
            .expect("");
    }
}
