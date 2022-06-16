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
    pub fn by_description(url: String, name: String, description: String) -> Self {
        Self {
            url,
            name: Some(name),
            description: Some(description),
            headers: HashMap::new(),
            request_type: RequestType::GET,
            hash: None,
            size: None,
        }
    }

    pub fn by_name(url: String, name: String) -> Self {
        Self {
            url,
            name: Some(name),
            description: None,
            headers: HashMap::new(),
            request_type: RequestType::GET,
            hash: None,
            size: None,
        }
    }

    pub fn by_url(url: String) -> Self {
        Self {
            url,
            name: None,
            description: None,
            headers: HashMap::new(),
            request_type: RequestType::GET,
            hash: None,
            size: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum HashAlgorithm {
    Sha1,
    Sha512,
    Md5,
}
