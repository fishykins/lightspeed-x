use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pricing {
    pub cost: f64,
    pub cost_total: f64,

    pub discount: f64,
    pub discount_total: f64,

    pub loyalty_amount: f64,
    pub loyalty_amount_total: f64,

    pub price: f64,

    pub total: f64,
}
