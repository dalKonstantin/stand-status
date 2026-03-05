use std::{net::Ipv4Addr, process::Command};

pub fn ping_host(host: Ipv4Addr) -> bool {
    let host_string = host.to_string();
    let mut child = Command::new("ping")
        .args(["-c", "1", "-W", "1", &host_string])
        .status()
        .expect("Failed to execute ping");
    if child.success() { true } else { false }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_eights() {
        let res = ping_host(Ipv4Addr::new(8, 8, 8, 8));
        assert_eq!(res, true);
    }

    #[test]
    fn ping_wrong_ip() {
        let res = ping_host(Ipv4Addr::new(88, 88, 8, 88));
        assert_eq!(res, false);
    }
}
