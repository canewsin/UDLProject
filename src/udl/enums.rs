use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enum {
    pub id: String,
    pub description: Option<String>,
    pub variants: Vec<EnumKind>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum EnumKind {
    Simple(String),
    Complex(EnumVariant),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnumVariant {
    pub id: String,
    pub description: Option<String>,
    pub value: EnumVariantValue,
    pub target: Option<String>,
    pub target_field: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum EnumVariantValue {
    Single(String),
    Multiple(HashMap<String, String>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum() {
        let test_1 = r#"
            id: SortBy
            variants:
              - K_ID
              - K_NAME
              - K_CREATED_AT
        "#;
        let enumm: Enum = serde_yaml::from_str(test_1).unwrap();

        assert_eq!(enumm.id, "SortBy");
        assert_eq!(enumm.variants.len(), 3);
        let mut variant = enumm.variants.get(0).unwrap();
        assert_eq!(*variant, EnumKind::Simple("K_ID".into()));
        variant = enumm.variants.get(1).unwrap();
        assert_eq!(*variant, EnumKind::Simple("K_NAME".into()));
        variant = enumm.variants.get(2).unwrap();
        assert_eq!(*variant, EnumKind::Simple("K_CREATED_AT".into()));

        let test_2 = r#"
            id: InvoiceStatus
            description: "Status of an invoice"
            default: K_NOT_FOUND
            variants:
                - id: K_NOT_FOUND
                  value: "Invoice not found"
                  description: "Invoice not found"
                - id: K_FORBIDDEN
                  value: "User does not have access to this invoice"
        "#;

        let enumm: Enum = serde_yaml::from_str(test_2).unwrap();

        assert_eq!(enumm.id, "InvoiceStatus");
        assert_eq!(enumm.description.unwrap(), "Status of an invoice");
        assert_eq!(enumm.variants.len(), 2);

        let variant = enumm.variants.get(0).unwrap();
        assert_eq!(
            EnumKind::Complex(EnumVariant {
                id: "K_NOT_FOUND".to_string(),
                description: Some("Invoice not found".to_string()),
                value: EnumVariantValue::Single("Invoice not found".to_string()),
                target: None,
                target_field: None,
            }),
            *variant
        );

        let variant = enumm.variants.get(1).unwrap();
        assert_eq!(
            EnumKind::Complex(EnumVariant {
                id: "K_FORBIDDEN".to_string(),
                description: None,
                value: EnumVariantValue::Single(
                    "User does not have access to this invoice".to_string()
                ),
                target: None,
                target_field: None,
            }),
            *variant
        );
    }
}
