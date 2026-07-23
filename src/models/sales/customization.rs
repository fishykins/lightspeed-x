use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customization {
    pub id: String,

    pub name: String,

    pub value: String,

    pub field_type: String,

    pub files: Vec<CustomizationFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomizationFile {
    pub id: String,

    pub filename: String,

    pub provider: String,

    pub size: u64,
}
