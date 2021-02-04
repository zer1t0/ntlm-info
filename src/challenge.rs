use ntlm::ChallengeMsg;
use std::fmt;
use std::net::IpAddr;

pub struct Challenge {
    pub target: ChallengeTarget,
    pub challenge: ChallengeMsg,
}

impl Challenge {
    pub fn new(target: ChallengeTarget, challenge: ChallengeMsg) -> Self {
        return Self { target, challenge };
    }
}

pub enum ChallengeTarget {
    Host(Host),
    Url(String),
}

impl fmt::Display for ChallengeTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::Host(h) => write!(f, "{}", h),
            Self::Url(u) => write!(f, "{}", u),
        }
    }
}

impl From<Host> for ChallengeTarget {
    fn from(h: Host) -> Self {
        return Self::Host(h);
    }
}

impl From<&str> for ChallengeTarget {
    fn from(s: &str) -> Self {
        return Self::Url(s.to_string());
    }
}

pub struct Host {
    pub ip: IpAddr,
    pub name: Option<String>,
}

impl Host {
    pub fn new(ip: IpAddr, name: Option<String>) -> Self {
        return Self { ip, name };
    }
}

impl fmt::Display for Host {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.name {
            Some(name) => write!(f, "{}/{}", self.ip, name),
            None => write!(f, "{}", self.ip),
        }
    }
}
