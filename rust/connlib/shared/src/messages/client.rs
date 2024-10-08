//! Client related messages that are needed within connlib

use std::{collections::BTreeSet, fmt, str::FromStr};

use ip_network::IpNetwork;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::callbacks::Status;

use super::ResourceId;
use itertools::Itertools;

/// Description of a resource that maps to a DNS record.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceDescriptionDns {
    /// Resource's id.
    pub id: ResourceId,
    /// Internal resource's domain name.
    pub address: String,
    /// Name of the resource.
    ///
    /// Used only for display.
    pub name: String,

    pub address_description: Option<String>,
    #[serde(rename = "gateway_groups")]
    pub sites: Vec<Site>,
}

impl ResourceDescriptionDns {
    pub fn with_status(self, status: Status) -> crate::callbacks::ResourceDescriptionDns {
        crate::callbacks::ResourceDescriptionDns {
            id: self.id,
            address: self.address,
            name: self.name,
            address_description: self.address_description,
            sites: self.sites,
            status,
        }
    }
}

/// Description of a resource that maps to a CIDR.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceDescriptionCidr {
    /// Resource's id.
    pub id: ResourceId,
    /// CIDR that this resource points to.
    pub address: IpNetwork,
    /// Name of the resource.
    ///
    /// Used only for display.
    pub name: String,

    pub address_description: Option<String>,
    #[serde(rename = "gateway_groups")]
    pub sites: Vec<Site>,
}

impl ResourceDescriptionCidr {
    pub fn with_status(self, status: Status) -> crate::callbacks::ResourceDescriptionCidr {
        crate::callbacks::ResourceDescriptionCidr {
            id: self.id,
            address: self.address,
            name: self.name,
            address_description: self.address_description,
            sites: self.sites,
            status,
        }
    }
}

fn internet_resource_name() -> String {
    "Internet Resource".to_string()
}

/// Description of an internet resource.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceDescriptionInternet {
    /// Name of the resource.
    ///
    /// Used only for display.
    #[serde(default = "internet_resource_name")]
    pub name: String,
    /// Resource's id.
    pub id: ResourceId,
    /// Sites for the internet resource
    #[serde(rename = "gateway_groups")]
    pub sites: Vec<Site>,
}

impl ResourceDescriptionInternet {
    pub fn with_status(self, status: Status) -> crate::callbacks::ResourceDescriptionInternet {
        crate::callbacks::ResourceDescriptionInternet {
            name: self.name,
            id: self.id,
            sites: self.sites,
            status,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialOrd, Ord)]
pub struct Site {
    pub id: SiteId,
    pub name: String,
}

impl std::hash::Hash for Site {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Site {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SiteId(Uuid);

impl FromStr for SiteId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SiteId(Uuid::parse_str(s)?))
    }
}

impl SiteId {
    pub fn from_u128(v: u128) -> Self {
        Self(Uuid::from_u128(v))
    }
}

impl fmt::Display for SiteId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for SiteId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl ResourceDescription {
    pub fn address_string(&self) -> Option<String> {
        match self {
            ResourceDescription::Dns(d) => Some(d.address.clone()),
            ResourceDescription::Cidr(c) => Some(c.address.to_string()),
            ResourceDescription::Internet(_) => None,
        }
    }

    pub fn sites_string(&self) -> String {
        self.sites().iter().map(|s| &s.name).join("|")
    }

    pub fn id(&self) -> ResourceId {
        match self {
            ResourceDescription::Dns(r) => r.id,
            ResourceDescription::Cidr(r) => r.id,
            ResourceDescription::Internet(r) => r.id,
        }
    }

    pub fn sites(&self) -> BTreeSet<&Site> {
        match self {
            ResourceDescription::Dns(r) => BTreeSet::from_iter(r.sites.iter()),
            ResourceDescription::Cidr(r) => BTreeSet::from_iter(r.sites.iter()),
            ResourceDescription::Internet(r) => BTreeSet::from_iter(r.sites.iter()),
        }
    }

    pub fn sites_mut(&mut self) -> &mut Vec<Site> {
        match self {
            ResourceDescription::Dns(r) => &mut r.sites,
            ResourceDescription::Cidr(r) => &mut r.sites,
            ResourceDescription::Internet(r) => &mut r.sites,
        }
    }

    /// What the GUI clients should show as the user-friendly display name, e.g. `Firezone GitHub`
    pub fn name(&self) -> &str {
        match self {
            ResourceDescription::Dns(r) => &r.name,
            ResourceDescription::Cidr(r) => &r.name,
            ResourceDescription::Internet(_) => "Internet",
        }
    }

    pub fn has_different_address(&self, other: &ResourceDescription) -> bool {
        match (self, other) {
            (ResourceDescription::Dns(dns_a), ResourceDescription::Dns(dns_b)) => {
                dns_a.address != dns_b.address
            }
            (ResourceDescription::Cidr(cidr_a), ResourceDescription::Cidr(cidr_b)) => {
                cidr_a.address != cidr_b.address
            }
            (ResourceDescription::Internet(_), ResourceDescription::Internet(_)) => false,
            _ => true,
        }
    }

    pub fn with_status(self, status: Status) -> crate::callbacks::ResourceDescription {
        match self {
            ResourceDescription::Dns(r) => {
                crate::callbacks::ResourceDescription::Dns(r.with_status(status))
            }
            ResourceDescription::Cidr(r) => {
                crate::callbacks::ResourceDescription::Cidr(r.with_status(status))
            }
            ResourceDescription::Internet(r) => {
                crate::callbacks::ResourceDescription::Internet(r.with_status(status))
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResourceDescription {
    Dns(ResourceDescriptionDns),
    Cidr(ResourceDescriptionCidr),
    Internet(ResourceDescriptionInternet),
}

impl ResourceDescription {
    pub fn into_dns(self) -> Option<ResourceDescriptionDns> {
        match self {
            ResourceDescription::Dns(d) => Some(d),
            ResourceDescription::Cidr(_) | ResourceDescription::Internet(_) => None,
        }
    }

    pub fn into_cidr(self) -> Option<ResourceDescriptionCidr> {
        match self {
            ResourceDescription::Cidr(c) => Some(c),
            ResourceDescription::Dns(_) | ResourceDescription::Internet(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_deserialize_internet_resource() {
        let resources = r#"[
            {
                "id": "73037362-715d-4a83-a749-f18eadd970e6",
                "type": "cidr",
                "name": "172.172.0.0/16",
                "address": "172.172.0.0/16",
                "address_description": "cidr resource",
                "gateway_groups": [{"name": "test", "id": "bf56f32d-7b2c-4f5d-a784-788977d014a4"}]
            },
            {
                "id": "03000143-e25e-45c7-aafb-144990e57dcd",
                "type": "dns",
                "name": "gitlab.mycorp.com",
                "address": "gitlab.mycorp.com",
                "address_description": "dns resource",
                "gateway_groups": [{"name": "test", "id": "bf56f32d-7b2c-4f5d-a784-788977d014a4"}]
            },
            {
                "id": "1106047c-cd5d-4151-b679-96b93da7383b",
                "type": "internet",
                "name": "Internet Resource",
                "gateway_groups": [{"name": "test", "id": "eb94482a-94f4-47cb-8127-14fb3afa5516"}],
                "not": "relevant",
                "some_other": [
                    "field"
                ]
            }
        ]"#;

        serde_json::from_str::<Vec<ResourceDescription>>(resources).unwrap();
    }
}
