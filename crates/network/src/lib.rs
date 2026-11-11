//! Network inspection and diagnostic tools for Smart-Filee.
//!
//! Provides local IPv4 detection, host validation, ping diagnostics, and TCP port checks.

use std::net::{SocketAddr, TcpStream, ToSocketAddrs, UdpSocket};
use std::process::Command;
use std::time::{Duration, Instant};

/// Determines the primary local IPv4 address by probing an outbound routing table.
/// Does not send any network packets. Falls back to "127.0.0.1".
pub fn local_ip() -> String {
    let Ok(socket) = UdpSocket::bind("0.0.0.0:0") else {
        return "127.0.0.1".to_string();
    };

    if socket.connect("10.255.255.255:1").is_ok() {
        if let Ok(addr) = socket.local_addr() {
            return addr.ip().to_string();
        }
    }

    "127.0.0.1".to_string()
}

/// Validates whether `host` is a safe, well-formed hostname or IP string.
/// Rejects empty strings, strings starting with `-`, and illegal characters.
pub fn valid_host(host: &str) -> bool {
    let trimmed = host.trim();
    if trimmed.is_empty() || trimmed.starts_with('-') {
        return false;
    }

    trimmed
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '.' || ch == '_' || ch == ':' || ch == '-')
}

/// Diagnoses network reachability for `host` by resolving DNS and issuing a system ICMP ping.
pub fn ping_host(host: &str) -> String {
    if !valid_host(host) {
        return "Invalid host name.".to_string();
    }

    let dns_start = Instant::now();
    let addrs_res: Result<Vec<SocketAddr>, _> =
        format!("{host}:0").to_socket_addrs().map(Iterator::collect);

    let (addrs, dns_ms) = match addrs_res {
        Ok(ips) if !ips.is_empty() => {
            let elapsed = dns_start.elapsed().as_secs_f64() * 1000.0;
            let mut unique_ips: Vec<String> = ips.iter().map(|s| s.ip().to_string()).collect();
            unique_ips.sort();
            unique_ips.dedup();
            (unique_ips, elapsed)
        }
        Ok(_) | Err(_) => {
            return format!("DNS lookup failed for {host}: unable to resolve address");
        }
    };

    let addr_display = addrs.iter().take(4).cloned().collect::<Vec<_>>().join(", ");
    let head = format!("{host} → {addr_display}\nDNS resolution: {dns_ms:.1} ms\n");

    let cmd_res = if cfg!(target_os = "windows") {
        Command::new("ping")
            .args(["-n", "2", "-w", "3000", host])
            .output()
    } else {
        Command::new("ping")
            .args(["-c", "2", "-W", "3", host])
            .output()
    };

    match cmd_res {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            let combined = format!("{stdout}\n{stderr}").trim().to_string();

            if output.status.success() {
                format!("Reachable.\n{head}\n{combined}")
            } else {
                format!("DNS resolved, but ping got no reply.\n{head}\n{combined}")
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            "The 'ping' command is not available on this system.".to_string()
        }
        Err(e) => {
            format!("Network check failed: {e}")
        }
    }
}

/// Tests whether a specific TCP port on `host` is open and accepting connections.
pub fn check_port(host: &str, port: u16, timeout: Duration) -> String {
    if !valid_host(host) {
        return "Invalid host name.".to_string();
    }
    if port == 0 {
        return "Port must be a number between 1 and 65535.".to_string();
    }

    let addr_str = format!("{host}:{port}");
    let addrs: Vec<SocketAddr> = match addr_str.to_socket_addrs() {
        Ok(iter) => iter.collect(),
        Err(e) => return format!("DNS lookup failed for {host}: {e}"),
    };

    let start = Instant::now();
    let mut last_err = None;

    for addr in addrs {
        match TcpStream::connect_timeout(&addr, timeout) {
            Ok(_) => {
                let ms = start.elapsed().as_secs_f64() * 1000.0;
                return format!("{host}:{port} is OPEN (connected in {ms:.0} ms)");
            }
            Err(e) => {
                last_err = Some(e);
            }
        }
    }

    if let Some(err) = last_err {
        if err.kind() == std::io::ErrorKind::TimedOut {
            format!("{host}:{port} timed out (filtered or unreachable)")
        } else {
            format!("{host}:{port} is CLOSED or unreachable ({err})")
        }
    } else {
        format!("{host}:{port} is CLOSED or unreachable")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_host_rules() {
        assert!(valid_host("google.com"));
        assert!(valid_host("127.0.0.1"));
        assert!(valid_host("localhost"));
        assert!(valid_host("my-server.local"));

        assert!(!valid_host(""));
        assert!(!valid_host("-flag"));
        assert!(!valid_host("bad host; rm -rf /"));
        assert!(!valid_host("host with spaces"));
    }

    #[test]
    fn test_local_ip_returns_valid_ip() {
        let ip = local_ip();
        assert!(!ip.is_empty());
        assert!(ip.contains('.') || ip.contains(':'));
    }

    #[test]
    fn test_check_port_loopback() {
        // Attempt connect to an unlistened port on localhost (should fail immediately)
        let res = check_port("127.0.0.1", 59999, Duration::from_millis(100));
        assert!(res.contains("CLOSED") || res.contains("unreachable") || res.contains("timed out"));
    }
}
