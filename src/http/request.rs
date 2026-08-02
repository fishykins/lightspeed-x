use reqwest::Method;

#[derive(Clone, Debug)]
pub struct Request {
    pub method: Method,
    pub endpoint: String,
    pub query: Vec<(String, String)>,
}

impl Request {
    pub fn get(endpoint: impl Into<String>) -> Self {
        Self {
            method: Method::GET,
            endpoint: endpoint.into(),
            query: Vec::new(),
        }
    }

    pub fn with_query(mut self, form: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.push((form.into(), value.into()));
        self
    }
}
