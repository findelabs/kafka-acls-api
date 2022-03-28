use serde::{Serialize, Deserialize};
use validator::{ValidationError, Validate};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AclDefinition {
    #[validate(length(min = 1), custom = "ensure_not_empty")]
    resource_type: String,
    #[validate(length(min = 1), custom = "ensure_not_empty")]
    resource_name: String,
    #[validate(length(min = 1), custom = "ensure_not_empty")]
    pattern_type: String,
    #[validate(length(min = 1), custom = "ensure_not_empty")]
    principal: String,
    #[validate(length(min = 1), custom = "ensure_not_empty")]
    host: String,
    #[validate(length(min = 1), custom = "ensure_not_empty")]
    operation: String,
    #[validate(length(min = 1), custom = "ensure_not_empty")]
    permission: String
}

fn ensure_not_empty(value: &str) -> Result<(), ValidationError> {
    if value == "" {
        return Err(ValidationError::new("Missing value"));
    }
    Ok(())
}


impl AclDefinition {
    pub fn query_pairs(&self) -> String {
        let query_pairs = format!("resource_type={}&resource_name={}&pattern_type={}&principal={}&host={}&operation={}&permission={}", self.resource_type, self.resource_name, self.pattern_type, self.principal, self.host, self.operation, self.permission);
        query_pairs
    }
}
