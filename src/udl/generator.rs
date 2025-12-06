use crate::udl::{UDL, class::Class, enums::Enum};

pub trait LangGenerator {
    fn gen_enum(&self, enumm: &Enum) -> String;

    fn gen_class(&self, class: &Class) -> String;

    fn generate(&self, udl: &UDL) -> String;

    fn format(&self, path: &str);
}
