use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturesFilter {
    id: String,
    description: String,
    enabled: bool,
    conditions: Conditions,
}
impl FeaturesFilter {
    pub fn get_filters(&self) -> &[ClientFilter] {
        self.conditions.client_filters.as_slice()
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Conditions {
    client_filters: Vec<ClientFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientFilter {
    name: String,
    parameters: Parameter, // todo!
}

impl ClientFilter {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_users(&self) -> &[String] {
        self.parameters.Audience.Users.as_slice()
    }

    pub fn get_groups(&self) -> &[String] {
        self.parameters.Audience.Groups.as_slice()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Parameter {
    Audience: Audience,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audience {
    Users: Vec<String>,
    Groups: Vec<String>,
    DefaultRolloutPercentage: i64,
}
