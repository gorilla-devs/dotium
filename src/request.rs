use super::*;
use std::collections::HashMap;
use url::Url;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Asset {
    /// The request Url
    pub url: Url,
    /// Name of the asset, usually the filename
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
    pub fn by_description(url: Url, name: Option<String>, description: Option<String>) -> Self {
        Self {
            url,
            name,
            description,
            headers: HashMap::new(),
            request_type: RequestType::GET,
            hash: None,
            size: None,
        }
    }

    pub fn by_name(url: Url, name: Option<String>) -> Self {
        Self {
            url,
            name,
            description: None,
            headers: HashMap::new(),
            request_type: RequestType::GET,
            hash: None,
            size: None,
        }
    }

    pub fn by_url(url: Url) -> Self {
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
    SHA1,
    SHA512,
    MD5,
}
