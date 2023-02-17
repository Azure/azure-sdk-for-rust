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
        let aud = self.parameters.Audience.as_ref();
        match aud {
            Some(audience) => audience.Users.clone(),
            None => vec![],
        }
    }

    pub fn get_groups(&self) -> Vec<Group> {
        let aud = self.parameters.Audience.as_ref();
        match aud {
            Some(audience) => audience.Groups.clone(),
            None => vec![],
        }
    }

    pub fn get_default_rollout_percentage(&self) -> i64 {
        let aud = self.parameters.Audience.as_ref();
        match aud {
            Some(audience) => audience.DefaultRolloutPercentage,
            None => 0,
        }
    }

    pub fn get_value(&self) -> Option<i64> {
        self.parameters.Value
    }

    pub fn get_end(&self) -> Option<String> {
        self.parameters.End.clone()
    }

    pub fn get_start(&self) -> Option<String> {
        self.parameters.Start.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Parameter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    Audience: Option<Audience>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    Value: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    End: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    Start: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audience {
    Users: Vec<String>,
    Groups: Vec<Group>,
    DefaultRolloutPercentage: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub Name: String,
    pub RolloutPercentage: i64,
}
