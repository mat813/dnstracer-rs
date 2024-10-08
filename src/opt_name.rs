use std::{
    cmp::Ordering,
    fmt, hash,
    net::{IpAddr, SocketAddr},
};

/// OptName is a struct that represents an nameserver and the zone it is supposed to be authoritative for.
#[derive(Debug, Clone, Eq)]
pub struct OptName {
    /// The name server IP address
    pub ip: IpAddr,
    /// Optionnally, the name server's FQDN
    pub name: Option<String>,
    /// The zone it is supposed to be authoritative for
    pub zone: Option<String>,
}

impl hash::Hash for OptName {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.ip.hash(state);
        self.name.hash(state);
    }
}

impl From<OptName> for SocketAddr {
    fn from(opt_name: OptName) -> Self {
        if opt_name.ip.is_ipv4() {
            format!("{}:53", opt_name.ip)
        } else {
            format!("[{}]:53", opt_name.ip)
        }
        .parse()
        .unwrap() // We can simply unwrap because the IpAddr is always valid
    }
}

impl fmt::Display for OptName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ip = self.ip;
        match (&self.name, &self.zone) {
            (None, _) => write!(f, "{ip} ({ip})"),
            (Some(name), None) => write!(f, "{name} ({ip})"),
            (Some(name), Some(zone)) => write!(
                f,
                "{name} [{z}] ({ip})",
                z = match zone.len() {
                    0 | 1 => zone,
                    _ => zone.trim_end_matches("."),
                },
            ),
        }
    }
}

impl PartialEq for OptName {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.ip == other.ip
    }
}

impl PartialOrd for OptName {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other)) // Delegate to the full cmp function
    }
}

impl Ord for OptName {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.name.as_ref(), &self.ip).cmp(&(other.name.as_ref(), &other.ip))
    }
}
