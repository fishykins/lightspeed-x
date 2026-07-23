use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxComponent {
    pub rate_id: String,
    pub total_tax: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxSummary {
    pub id: String,
    pub tax: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tax {
    pub id: String,
    pub amount: f64,
    pub total: f64,
}
