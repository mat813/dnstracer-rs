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

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_optname_eq() {
        let opt1 = OptName {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            name: Some("ns1.example.com".to_string()),
            zone: Some("example.com.".to_string()),
        };

        let opt2 = OptName {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            name: Some("ns1.example.com".to_string()),
            zone: Some("example.com.".to_string()),
        };

        assert_eq!(
            opt1, opt2,
            "OptNames with the same IP and name should be equal"
        );
    }

    #[test]
    fn test_optname_neq() {
        let opt1 = OptName {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            name: Some("ns1.example.com".to_string()),
            zone: Some("example.com.".to_string()),
        };

        let opt2 = OptName {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2)),
            name: Some("ns2.example.com".to_string()),
            zone: Some("example.com.".to_string()),
        };

        assert_ne!(
            opt1, opt2,
            "OptNames with different IPs or names should not be equal"
        );
    }

    #[test]
    fn test_optname_ordering() {
        let opt1 = OptName {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            name: Some("ns1.example.com".to_string()),
            zone: Some("example.com.".to_string()),
        };

        let opt2 = OptName {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2)),
            name: Some("ns2.example.com".to_string()),
            zone: Some("example.com.".to_string()),
        };

        assert!(opt1 < opt2, "OptName should be ordered by name and then IP");
    }

    #[test]
    fn test_optname_display() {
        let opt = OptName {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            name: None,
            zone: None,
        };

        let expected = "192.168.1.1 (192.168.1.1)";
        assert_eq!(
            format!("{}", opt),
            expected,
            "Display without zone is incorrect"
        );
    }

    #[test]
    fn test_optname_display_with_name() {
        let opt = OptName {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            name: Some("ns1.example.com".to_string()),
            zone: None,
        };

        let expected = "ns1.example.com (192.168.1.1)";
        assert_eq!(
            format!("{}", opt),
            expected,
            "Display without zone is incorrect"
        );
    }

    #[test]
    fn test_optname_full_display() {
        let opt = OptName {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            name: Some("ns1.example.com".to_string()),
            zone: Some("example.com.".to_string()),
        };

        let expected = "ns1.example.com [example.com] (192.168.1.1)";
        assert_eq!(
            format!("{}", opt),
            expected,
            "Display implementation is incorrect"
        );
    }

    #[test]
    fn test_optname_full_display_root_zone() {
        let opt = OptName {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            name: Some("ns1.example.com".to_string()),
            zone: Some(".".to_string()),
        };

        let expected = "ns1.example.com [.] (192.168.1.1)";
        assert_eq!(
            format!("{}", opt),
            expected,
            "Display implementation is incorrect"
        );
    }

    #[test]
    fn test_optname_into_socketaddr_v4() {
        let opt = OptName {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            name: None,
            zone: None,
        };

        let socket_addr: SocketAddr = opt.into();
        assert_eq!(socket_addr, "192.168.1.1:53".parse().unwrap());
    }

    #[test]
    fn test_optname_into_socketaddr_v6() {
        let opt = OptName {
            ip: IpAddr::V6(Ipv6Addr::new(
                0xfe80, 0, 0, 0, 0x0202, 0xb3ff, 0xfe1e, 0x8329,
            )),
            name: None,
            zone: None,
        };

        let socket_addr: SocketAddr = opt.into();
        assert_eq!(
            socket_addr,
            "[fe80::202:b3ff:fe1e:8329]:53".parse().unwrap()
        );
    }

    #[test]
    fn test_optname_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let opt1 = OptName {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            name: Some("ns1.example.com".to_string()),
            zone: Some("example.com.".to_string()),
        };

        let opt2 = OptName {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            name: Some("ns1.example.com".to_string()),
            zone: Some("example.com.".to_string()),
        };

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        opt1.hash(&mut hasher1);
        opt2.hash(&mut hasher2);

        assert_eq!(
            hasher1.finish(),
            hasher2.finish(),
            "Hash should be the same for identical OptNames"
        );
    }
}
