use std::collections::HashMap;

use serde_json::Value;

use crate::errors::Result;
use crate::request::{get, put};
use crate::{Client, QueryMeta, QueryOptions, WriteMeta, WriteOptions};

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(default)]
#[allow(clippy::upper_case_acronyms)]
pub struct CAConfig {
	#[serde(rename = "Provider")]
    provider: String,
	#[serde(rename = "Config")]
    config: Value,
	#[serde(rename = "CreateIndex")]
    create_index: u64,
	#[serde(rename = "ModifyIndex")]
    modify_index: u64,
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(default)]
#[allow(clippy::upper_case_acronyms)]
pub struct CARootList {
	#[serde(rename = "ActiveRootID")]
    active_root_id: String,
	#[serde(rename = "TrustDomain")]
    trust_domain: String,
	#[serde(rename = "Roots")]
    roots: Vec<CARoot>,
}

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
#[allow(clippy::upper_case_acronyms)]
pub struct CARoot {
	#[serde(rename = "ID")]
    id: String,
	#[serde(rename = "Name")]
    name: String,
	#[serde(rename = "RootCert")]
    root_cert: String,
	#[serde(rename = "Active")]
    active: bool,
	#[serde(rename = "CreateIndex")]
    create_index: u64,
	#[serde(rename = "ModifyIndex")]
    modify_index: u64,
}

#[allow(clippy::upper_case_acronyms)]
pub trait ConnectCA {
    fn ca_roots(&self, q: Option<&QueryOptions>) -> Result<(CARootList, QueryMeta)>;
    fn ca_get_config(&self, q: Option<&QueryOptions>) -> Result<(CAConfig, QueryMeta)>;
    fn ca_set_config(&self, conf: &CAConfig, q: Option<&WriteOptions>) -> Result<((), WriteMeta)>;
}

impl ConnectCA for Client {
    /// https://www.consul.io/api/connect/ca.html#list-ca-root-certificates
    fn ca_roots(&self, q: Option<&QueryOptions>) -> Result<(CARootList, QueryMeta)> {
        get("/v1/connect/ca/roots", &self.config, HashMap::new(), q)
    }

    /// https://www.consul.io/api/connect/ca.html#get-ca-configuration
    fn ca_get_config(&self, q: Option<&QueryOptions>) -> Result<(CAConfig, QueryMeta)> {
        get(
            "/v1/connect/ca/configuration",
            &self.config,
            HashMap::new(),
            q,
        )
    }

    /// https://www.consul.io/api/connect/ca.html#update-ca-configuration
    fn ca_set_config(&self, conf: &CAConfig, q: Option<&WriteOptions>) -> Result<((), WriteMeta)> {
        put(
            "/v1/connect/ca/configuration",
            Some(conf),
            &self.config,
            HashMap::new(),
            q,
        )
    }
}
