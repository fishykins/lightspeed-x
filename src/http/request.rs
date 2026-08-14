use reqwest::Method;

#[derive(Clone, Debug)]
pub struct Request {
    pub method: Method,
    pub endpoint: String,
    pub query: Vec<(String, String)>,
    pub payload: Option<serde_json::Value>,
}

impl Request {
    pub fn new(method: Method, endpoint: impl Into<String>) -> Self {
        Self {
            method,
            endpoint: endpoint.into(),
            query: Vec::new(),
            payload: None,
        }
    }

    pub fn get(endpoint: impl Into<String>) -> Self {
        Self::new(Method::GET, endpoint)
    }

    pub fn post(endpoint: impl Into<String>) -> Self {
        Self::new(Method::POST, endpoint)
    }

    pub fn delete(endpoint: impl Into<String>) -> Self {
        Self::new(Method::DELETE, endpoint)
    }

    pub fn put(endpoint: impl Into<String>) -> Self {
        Self::new(Method::PUT, endpoint)
    }

    pub fn with_query(mut self, form: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.push((form.into(), value.into()));
        self
    }

    pub fn with_payload(mut self, payload: impl Into<serde_json::Value>) -> Self {
        self.payload = Some(payload.into());
        self
    }
}
