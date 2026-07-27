use crate::{
    LsError, LsResult,
    models::{customers::WebhookCustomer, products::WebhookProduct, sales::WebhookSale},
    webhooks::{WebhookForm, WebhookKind},
};

pub enum WebhookEvent {
    Sale(WebhookSale),
    Product(WebhookProduct),
    Customer(WebhookCustomer),
}

impl TryFrom<WebhookForm> for WebhookEvent {
    type Error = LsError;

    fn try_from(form: WebhookForm) -> LsResult<Self> {
        match form.kind() {
            WebhookKind::Sale => {
                let sale: WebhookSale = form.parse()?;
                Ok(WebhookEvent::Sale(sale))
            }
            WebhookKind::Product => {
                let product: WebhookProduct = form.parse()?;
                Ok(WebhookEvent::Product(product))
            }
            WebhookKind::Customer => {
                let customer: WebhookCustomer = form.parse()?;
                Ok(WebhookEvent::Customer(customer))
            }
            value => Result::Err(LsError::Other(format!(
                "Unknown WebhookForm.kind(): {:?}",
                value
            ))),
        }
    }
}
