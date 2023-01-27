pub mod validate {
    use regex::Regex;

    pub fn user_name(name: String) -> bool {
        if name.len() < 1 { return false; };
        return true;
    }

    pub fn ip(input: String) -> bool {
        let ip_re = Regex::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:\d+").unwrap();
        if !ip_re.is_match(input.as_str()) { return false; };
        return true;
    }
}

pub mod convert {
    use std::net::{SocketAddr, ToSocketAddrs};

    pub fn to_ip(input: String) -> SocketAddr {
        return input.to_socket_addrs().unwrap().as_slice()[0];
    }
}
