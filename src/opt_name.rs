use derive_more::PartialEq;
use std::{
    cmp::Ordering,
    fmt, hash,
    net::{IpAddr, SocketAddr},
};

/// `OptName` is a struct that represents an nameserver and the zone it is supposed to be authoritative for.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OptName {
    /// The name server IP address
    pub ip: IpAddr,
    /// Optionnally, the name server's FQDN
    pub name: Option<String>,
    /// The zone it is supposed to be authoritative for
    #[partial_eq(skip)]
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
        Self::new(opt_name.ip, 53)
    }
}

impl From<&OptName> for SocketAddr {
    fn from(opt_name: &OptName) -> Self {
        Self::new(opt_name.ip, 53)
    }
}

impl fmt::Display for OptName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self {
                ref ip, name: None, ..
            } => write!(f, "{ip} ({ip})"),
            Self {
                ref ip,
                name: Some(ref name),
                zone: None,
            } => write!(f, "{name} ({ip})"),
            Self {
                ref ip,
                name: Some(ref name),
                zone: Some(ref zone),
            } => write!(f, "{name} [{zone}] ({ip})"),
        }
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
    #![allow(clippy::expect_used, clippy::unwrap_used, reason = "test")]

    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn optname_eq() {
        let opt1 = OptName {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            name: Some("ns1.example.com".to_owned()),
            zone: Some("example.com.".to_owned()),
        };

        let opt2 = OptName {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            name: Some("ns1.example.com".to_owned()),
            zone: Some("example.com.".to_owned()),
        };

        assert_eq!(
            opt1, opt2,
            "OptNames with the same IP and name should be equal"
        );
    }

    #[test]
    fn optname_neq() {
        let opt1 = OptName {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            name: Some("ns1.example.com".to_owned()),
            zone: Some("example.com.".to_owned()),
        };

        let opt2 = OptName {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2)),
            name: Some("ns2.example.com".to_owned()),
            zone: Some("example.com.".to_owned()),
        };

        assert_ne!(
            opt1, opt2,
            "OptNames with different IPs or names should not be equal"
        );
    }

    #[test]
    fn optname_ordering() {
        let opt1 = OptName {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            name: Some("ns1.example.com".to_owned()),
            zone: Some("example.com.".to_owned()),
        };

        let opt2 = OptName {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2)),
            name: Some("ns2.example.com".to_owned()),
            zone: Some("example.com.".to_owned()),
        };

        assert!(opt1 < opt2, "OptName should be ordered by name and then IP");
    }

    #[test]
    fn optname_display() {
        let opt = OptName {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            name: None,
            zone: None,
        };

        let expected = "192.168.1.1 (192.168.1.1)";
        assert_eq!(
            format!("{opt}"),
            expected,
            "Display without zone is incorrect"
        );
    }

    #[test]
    fn optname_display_with_name() {
        let opt = OptName {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            name: Some("ns1.example.com".to_owned()),
            zone: None,
        };

        let expected = "ns1.example.com (192.168.1.1)";
        assert_eq!(
            format!("{opt}"),
            expected,
            "Display without zone is incorrect"
        );
    }

    #[test]
    fn optname_full_display() {
        let opt = OptName {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            name: Some("ns1.example.com".to_owned()),
            zone: Some("example.com.".to_owned()),
        };

        let expected = "ns1.example.com [example.com.] (192.168.1.1)";
        assert_eq!(
            format!("{opt}"),
            expected,
            "Display implementation is incorrect"
        );
    }

    #[test]
    fn optname_full_display_root_zone() {
        let opt = OptName {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            name: Some("ns1.example.com".to_owned()),
            zone: Some(".".to_owned()),
        };

        let expected = "ns1.example.com [.] (192.168.1.1)";
        assert_eq!(
            format!("{opt}"),
            expected,
            "Display implementation is incorrect"
        );
    }

    #[test]
    fn optname_into_socketaddr_v4() {
        let opt = OptName {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            name: None,
            zone: None,
        };

        let socket_addr: SocketAddr = opt.into();
        assert_eq!(socket_addr, "192.168.1.1:53".parse().unwrap());
    }

    #[test]
    fn optname_into_socketaddr_v6() {
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
    fn optname_hash() {
        use std::{
            collections::hash_map::DefaultHasher,
            hash::{Hash as _, Hasher as _},
        };

        let opt1 = OptName {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            name: Some("ns1.example.com".to_owned()),
            zone: Some("example.com.".to_owned()),
        };

        let opt2 = OptName {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            name: Some("ns1.example.com".to_owned()),
            zone: Some("example.com.".to_owned()),
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
