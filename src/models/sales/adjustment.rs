use serde::{Deserialize, Serialize};

use crate::models::sales::TaxComponent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleAdjustment {
    pub adjustment_type: String,

    pub name: String,

    pub total: f64,

    pub tax_components: Vec<TaxComponent>,
}
