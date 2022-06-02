use super::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Asset {
    /// The request URL
    url: String,
    /// Name of the asset 
    name: Option<String>,
    /// Description of the asset
    description: Option<String>,
    /// Headers to send with the request
    headers: Option<String>,
    /// Request type
    request_type: RequestType,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum RequestType {
    GET,
    POST,
    PATCH,
    DELETE,
}
