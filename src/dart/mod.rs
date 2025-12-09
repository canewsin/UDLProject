use core::panic;
use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

use convert_case::ccase;

use crate::udl::{
    LangGenerator, UDL,
    class::{Class, Property, PropertyKey},
    enums::{Enum, EnumKind, EnumVariantValue},
    utils::{extract_enum_variant, is_nullable_type, parse_length_validator},
};

pub(crate) struct DartGenerator();

static MAPPINGS: LazyLock<HashMap<&str, &str>> = std::sync::LazyLock::new(|| {
    HashMap::from([
        ("bool", "bool"),
        ("int", "int"),
        ("float", "double"),
        ("string", "String"),
        ("isize", "int"),
        ("int8", "int"),
        ("int16", "int"),
        ("int32", "int"),
        ("int64", "int"),
        ("usize", "int"),
        ("uint8", "int"),
        ("uint16", "int"),
        ("uint32", "int"),
        ("uint64", "int"),
        ("float32", "double"),
        ("float64", "double"),
        ("datetime", "DateTime"),
        ("date", "DateTime"),
        ("object", "Object"),
    ])
});

const VALIDATORS: [PropertyKey; 5] = [
    PropertyKey::Length,
    PropertyKey::Format,
    PropertyKey::Default,
    PropertyKey::Min,
    PropertyKey::Max,
];

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
    let prefix = if is_nullable { "" } else { "" };
    let suffix = if is_nullable { "?" } else { "" };
    format!(
        "{}{}{}",
        prefix,
        mapped_type.unwrap_or(&name.as_str()),
        suffix
    )
}

impl LangGenerator for DartGenerator {
    fn extension(&self) -> &str {
        "dart"
    }

    fn gen_enum(&self, enumm: &Enum) -> String {
        let mut code = String::new();
        if let Some(desc) = enumm.description.as_ref() {
            code.push_str(&format!("/// {}\n", desc));
        }
        code.push_str("enum ");
        code.push_str(&enumm.id);
        code.push_str(" {");
        let is_complex = enumm
            .variants
            .iter()
            .any(|variant| matches!(variant, EnumKind::Complex(_)));
        for variant in &enumm.variants {
            //TODO!: Handle default value
            let is_last = variant == enumm.variants.last().unwrap();
            match variant {
                EnumKind::Simple(name) => {
                    code.push_str(&format!("{},", ccase!(camel, name)));
                }
                EnumKind::Complex(variant) => {
                    if let Some(desc) = variant.description.as_ref() {
                        code.push_str(&format!("/// {}\n", desc));
                    }
                    code.push_str(&format!("{}", ccase!(camel, variant.id.as_str())));

                    match &variant.value {
                        EnumVariantValue::Single(str) => {
                            code.push_str(&format!("(\"{str}\")"));
                        }
                        EnumVariantValue::Multiple(_map) => {}
                    }

                    if is_last {
                        code.push_str(";");
                    } else {
                        code.push_str(",");
                    }
                }
            }
        }

        if is_complex {
            code.push_str("final String value;\n\n");
            code.push_str(&format!("const {}(this.value);", enumm.id));
        }

        code.push_str("}");
        code
    }

    fn gen_class(&self, class: &Class, error_enum: Option<&Enum>) -> (String, HashSet<String>) {
        let mut imports = HashSet::new();
        let mut code = String::new();
        if let Some(desc) = &class.description {
            code.push_str(&format!("/// {}\n", desc));
        }
        code.push_str(&format!("class {} {{", class.id));
        // code.push_str("\n");
        let mut need_priv_constructor = false;
        let mut props = HashMap::new();
        let mut pub_props = HashMap::new();
        let mut priv_props = HashMap::new();
        for (name, ty) in &class.properties {
            let mut private = false;
            #[allow(unused)]
            let mut type_str = String::new();
            match ty {
                Property::Type(ty) => {
                    code.push_str(&format!(
                        "final {} {};",
                        process_type(ty),
                        ccase!(camel, name)
                    ));
                    type_str = process_type(ty);
                }
                Property::Map(map) => {
                    if let Some(desc) = &map.get(&PropertyKey::Description) {
                        code.push_str(&format!("/// {}\n", desc));
                    }
                    private = map.get(&PropertyKey::Private) == Some(&String::from("true"));
                    need_priv_constructor =
                        map.keys().any(|k| VALIDATORS.contains(&k)) && class.error.is_some();
                    let ty = process_type(&map[&PropertyKey::Type]);
                    type_str = ty.clone();
                    code.push_str(&format!(
                        "final {} {}{};",
                        type_str,
                        if private { "_" } else { "" },
                        ccase!(camel, name)
                    ));
                    if private {
                        priv_props.insert(name.clone(), type_str.clone());
                    }
                }
            }
            let prefix = if !private {
                "required this."
            } else {
                &format!("{} ", type_str)
            };
            props.insert(
                format!("{}{}", prefix, ccase!(camel, name)),
                type_str.clone(),
            );
            pub_props.insert(ccase!(camel, name), type_str);
        }
        code.push_str("\n\n");
        let priv_props_list = priv_props
            .keys()
            // .into_iter()
            .map(|name| format!("_{} = {}", name, name))
            .collect::<Vec<String>>();
        let suffix = if priv_props_list.is_empty() {
            "".to_string()
        } else {
            format!(": {}", priv_props_list.join(", "))
        };
        code.push_str(&format!(
            "    const {}{}({{ {} }}){};",
            class.id,
            if need_priv_constructor { "._" } else { "" },
            props
                .keys()
                .into_iter()
                .cloned()
                .collect::<Vec<String>>()
                .join(", "),
            suffix
        ));

        code.push_str("\n\n");
        // code.push_str(&format!(";\n"));

        if need_priv_constructor {
            if error_enum.is_none() {
                println!("{}", class.id);
                panic!("Error enum is required for private constructor");
            }
            let err_enum_name = error_enum.unwrap().id.clone();
            code.push_str(&format!(
                "    static ResultDart<{}, {}> build({{ {} }}) {{",
                class.id,
                class.error.clone().unwrap_or_default(),
                pub_props
                    .iter()
                    .map(|(name, ty)| format!("required {} {}", ty, name))
                    .collect::<Vec<String>>()
                    .join(", "),
            ));
            imports.insert("import 'package:result_dart/result_dart.dart';\n".to_string());

            for (name, prop) in &class.properties {
                if let Property::Map(map) = prop {
                    for (key, value) in map {
                        if key == &PropertyKey::Default {
                            continue;
                        }
                        if VALIDATORS.contains(&key) {
                            if key == &PropertyKey::Default {
                                #[cfg(debug_assertions)]
                                code.push_str(&format!(
                                    "// {:?} Validator found for {}\n",
                                    key, name
                                ));
                                continue;
                            }
                            #[cfg(debug_assertions)]
                            code.push_str(&format!("// {:?} Validator found for {}\n", key, name));
                            let (min, max, _) = if key == &PropertyKey::Length {
                                parse_length_validator(value)
                            } else if key == &PropertyKey::Min {
                                (value.parse().unwrap(), -1, -1)
                            } else if key == &PropertyKey::Max {
                                (0, value.parse().unwrap(), -1)
                            } else {
                                (-1, -1, -1)
                            };
                            if (min, max) == (-1, -1) {
                                continue;
                            }
                            if min != -1 {
                                let variant =
                                    extract_enum_variant(error_enum.unwrap(), "length:min");
                                let variant = if variant.len() == 1 {
                                    let v = variant.get(0).unwrap();
                                    Some((&v.0, &v.1))
                                } else {
                                    variant
                                        .iter()
                                        .filter_map(|(id, str, field)| {
                                            if let Some(name_) = field
                                                && name_ == name
                                            {
                                                Some((id, str))
                                            } else {
                                                None
                                            }
                                        })
                                        .next()
                                };
                                if let Some((variant, _)) = variant {
                                    code.push_str(&format!("if ({}.length < {}) {{\n", name, min));
                                    code.push_str(&format!(
                                        "    return Failure({}.{});\n",
                                        err_enum_name,
                                        ccase!(camel, variant)
                                    ));
                                    code.push_str("}\n");
                                }
                            }
                            if max != -1 {
                                let variant =
                                    extract_enum_variant(error_enum.unwrap(), "length:max");

                                let variant = if variant.len() == 1 {
                                    let v = variant.get(0).unwrap();
                                    Some((&v.0, &v.1))
                                } else {
                                    variant
                                        .iter()
                                        .filter_map(|(id, str, field)| {
                                            if let Some(name_) = field
                                                && name_ == name
                                            {
                                                Some((id, str))
                                            } else {
                                                None
                                            }
                                        })
                                        .next()
                                };
                                if let Some((variant, _)) = variant {
                                    code.push_str(&format!("if ({}.length > {}) {{\n", name, max));
                                    code.push_str(&format!(
                                        "    return Failure({}.{});\n",
                                        err_enum_name,
                                        ccase!(camel, variant)
                                    ));
                                    code.push_str("}\n");
                                }
                            }
                        }
                    }
                }
            }

            code.push_str(&format!(
                "return Success({}._({}));",
                class.id.as_str(),
                class
                    .properties
                    .keys()
                    .fold(String::new(), |acc, k| acc
                        + &format!("{}:{},", ccase!(camel, k), ccase!(camel, k)))
                    .trim_end_matches(", ")
            ));

            code.push_str("}");
        }

        for (name, ty) in priv_props {
            code.push_str("\n\n");
            code.push_str(&format!("    {} get {} => _{};", ty, name, name));
        }

        code.push_str(&format!("}}\n"));
        (code, imports)
    }

    fn generate(&self, udl: &UDL) -> String {
        let mut imports = HashSet::new();
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
            let (gen_code, imports_) = self.gen_class(class, error_enum);
            code.push_str(&gen_code);
            code.push_str("\n\n");
            imports.extend(imports_);
        }

        let final_str = format!(
            "{}\n\n{}",
            imports.into_iter().collect::<Vec<_>>().join("\n"),
            code
        );

        final_str
    }

    fn format(&self, _path: &str) {
        // TODO!: Implement formatting logic for Dart code
        // std::process::Command::new("dart format")
        //     .arg(path)
        //     .stdout(Stdio::piped())
        //     .spawn()
        //     .expect("");
    }
}
