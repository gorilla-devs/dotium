use std::collections::HashMap;

use super::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Asset {
    /// The request URL
    pub url: String,
    /// Name of the asset
    pub name: Option<String>,
    /// Description of the asset
    pub description: Option<String>,
    /// Additional headers to send with the request
    pub headers: HashMap<String, String>,
    /// Request type
    pub request_type: RequestType,
    /// Hashes to check
    pub hash: Option<AssetHash>,
    /// Optional size of file in bytes
    pub size: Option<usize>,
}

impl Asset {
    pub fn by_description(url: &str, name: &str, description: &str) -> Self {
        Self {
            url: url.to_string(),
            name: Some(name.to_string()),
            description: Some(description.to_string()),
            headers: HashMap::new(),
            request_type: RequestType::GET,
            hash: None,
            size: None,
        }
    }

    pub fn by_name(url: &str, name: &str) -> Self {
        Self {
            url: url.to_string(),
            name: Some(name.to_string()),
            description: None,
            headers: HashMap::new(),
            request_type: RequestType::GET,
            hash: None,
            size: None,
        }
    }

    pub fn by_url(url: &str) -> Self {
        Self {
            url: url.to_string(),
            name: None,
            description: None,
            headers: HashMap::new(),
            request_type: RequestType::GET,
            hash: None,
            size: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum RequestType {
    GET,
    POST,
    PATCH,
    DELETE,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AssetHash {
    pub hash: String,
    pub algorithm: HashAlgorithm,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum HashAlgorithm {
    Sha1,
    Sha512,
    Md5,
}
