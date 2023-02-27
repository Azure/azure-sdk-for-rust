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

    pub fn get_users(&self) -> Vec<String> {
        let aud = self.parameters.audience.as_ref();
        match aud {
            Some(audience) => audience.users.clone(),
            None => vec![],
        }
    }

    pub fn get_groups(&self) -> Vec<Group> {
        let aud = self.parameters.audience.as_ref();
        match aud {
            Some(audience) => audience.groups.clone(),
            None => vec![],
        }
    }

    pub fn get_default_rollout_percentage(&self) -> i64 {
        let aud = self.parameters.audience.as_ref();
        match aud {
            Some(audience) => audience.default_rollout_percentage,
            None => 0,
        }
    }

    pub fn get_value(&self) -> Option<i64> {
        self.parameters.value
    }

    pub fn get_end(&self) -> Option<String> {
        self.parameters.end.clone()
    }

    pub fn get_start(&self) -> Option<String> {
        self.parameters.start.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Parameter {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Audience")]
    audience: Option<Audience>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Value")]
    value: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "End")]
    end: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Start")]
    start: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audience {
    #[serde(rename = "Users")]
    users: Vec<String>,
    #[serde(rename = "Groups")]
    groups: Vec<Group>,
    #[serde(rename = "DefaultRolloutPercentage")]
    default_rollout_percentage: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "RolloutPercentage")]
    pub rollout_percentage: i64,
}
