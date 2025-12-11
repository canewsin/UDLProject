use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    pub id: String,
    pub description: Option<String>,
    pub immutable: Option<bool>,
    pub error: Option<String>,
    pub properties: HashMap<String, Property>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Property {
    Type(String),
    Map(HashMap<PropertyKey, String>),
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PropertyKey {
    Description,
    Type,
    Format,
    Limit,
    Min,
    Max,
    Private,
    Default,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_parse() {
        let test_1 = r#"
            id: User
            description: "User profile data"
            immutable: true
            properties:
                id:
                    type: string
                    format: uuid
                email:
                    type: string
                    format: email
                name:
                    type: string
                    limit: 1...100
                phone:
                    type: string?
                    format: phone
                company: string?
                created_at: datetime
                updated_at: datetime^  # Short Syntax for Required Property equals false
                login_status: $enum::LoginStatus
            "#;
        let class: Class = serde_yaml::from_str(test_1).unwrap();
        assert_eq!(class.id, "User");
        assert_eq!(class.description.unwrap(), "User profile data");
        assert_eq!(class.immutable.unwrap(), true);
        assert_eq!(class.error, None);
        assert_eq!(
            class.properties.get("id").unwrap(),
            &Property::Map(HashMap::from([
                (PropertyKey::Type, "string".to_string()),
                (PropertyKey::Format, "uuid".to_string()),
            ]))
        );
        assert_eq!(
            class.properties.get("email").unwrap(),
            &Property::Map(HashMap::from([
                (PropertyKey::Type, "string".to_string()),
                (PropertyKey::Format, "email".to_string()),
            ]))
        );
        assert_eq!(
            class.properties.get("name").unwrap(),
            &Property::Map(HashMap::from([
                (PropertyKey::Type, "string".to_string()),
                (PropertyKey::Limit, "1...100".to_string()),
            ]))
        );
        assert_eq!(
            class.properties.get("phone").unwrap(),
            &Property::Map(HashMap::from([
                (PropertyKey::Type, "string?".to_string()),
                (PropertyKey::Format, "phone".to_string()),
            ]))
        );
        assert_eq!(
            class.properties.get("company").unwrap(),
            &Property::Type("string?".to_string()),
        );
        assert_eq!(
            class.properties.get("created_at").unwrap(),
            &Property::Type("datetime".to_string()),
        );
        assert_eq!(
            class.properties.get("updated_at").unwrap(),
            &Property::Type("datetime^".to_string()),
        );
        assert_eq!(
            class.properties.get("login_status").unwrap(),
            &Property::Type("$enum::LoginStatus".to_string()),
        );

        let test_2 = r#"
            id: LoginRequest
            description: "User login request"
            error: LoginError
            properties:
              email:
                type: string
                format: email
                description: "User email address"
              password:
                type: string
                limit: 8...32
              remember_me: bool
            "#;

        let class: Class = serde_yaml::from_str(test_2).unwrap();

        assert_eq!(class.id, "LoginRequest");
        assert_eq!(&class.description.unwrap(), "User login request");
        assert_eq!(&class.error.unwrap(), "LoginError");
        assert_eq!(class.properties.len(), 3);
        assert_eq!(
            class.properties.get("email").unwrap(),
            &Property::Map(HashMap::from([
                (PropertyKey::Type, "string".to_string()),
                (PropertyKey::Format, "email".to_string()),
                (PropertyKey::Description, "User email address".to_string()),
            ]))
        );
        assert_eq!(
            class.properties.get("password").unwrap(),
            &Property::Map(HashMap::from([
                (PropertyKey::Type, "string".to_string()),
                (PropertyKey::Limit, "8...32".to_string()),
            ]))
        );
        assert_eq!(
            class.properties.get("remember_me").unwrap(),
            &Property::Type("bool".to_string())
        );
    }
}
