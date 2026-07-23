use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Promotion {
    pub id: String,

    pub name: String,

    pub amount: f64,

    pub promo_code: Option<String>,

    pub promo_code_id: Option<String>,
}
