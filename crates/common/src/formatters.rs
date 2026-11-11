/// Format bytes into human-readable string (B, KB, MB, GB, TB)
/// matching Python `core.fmt_bytes`.
pub fn fmt_bytes(bytes: u64) -> String {
    let mut n = bytes as f64;
    for unit in ["B", "KB", "MB", "GB", "TB"] {
        if n < 1024.0 || unit == "TB" {
            return format!("{n:.1} {unit}");
        }
        n /= 1024.0;
    }
    format!("{n:.1} TB")
}

/// Format duration in seconds into human-readable string ("1d 2h 3m" or "2h 3m")
/// matching Python `core.fmt_duration`.
pub fn fmt_duration(seconds: u64, units: (&str, &str, &str)) -> String {
    let d = seconds / 86400;
    let r1 = seconds % 86400;
    let h = r1 / 3600;
    let r2 = r1 % 3600;
    let m = r2 / 60;
    let (ud, uh, um) = units;
    if d > 0 {
        format!("{d}{ud} {h}{uh} {m}{um}")
    } else {
        format!("{h}{uh} {m}{um}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt_bytes() {
        assert_eq!(fmt_bytes(0), "0.0 B");
        assert_eq!(fmt_bytes(500), "500.0 B");
        assert_eq!(fmt_bytes(1024), "1.0 KB");
        assert_eq!(fmt_bytes(1536), "1.5 KB");
        assert_eq!(fmt_bytes(1024 * 1024), "1.0 MB");
        assert_eq!(fmt_bytes(1024 * 1024 * 1024), "1.0 GB");
        assert_eq!(fmt_bytes(1024 * 1024 * 1024 * 1024), "1.0 TB");
    }

    #[test]
    fn test_fmt_duration() {
        assert_eq!(fmt_duration(0, ("d", "h", "m")), "0h 0m");
        assert_eq!(fmt_duration(65, ("d", "h", "m")), "0h 1m");
        assert_eq!(fmt_duration(3665, ("d", "h", "m")), "1h 1m");
        assert_eq!(fmt_duration(90060, ("d", "h", "m")), "1d 1h 1m");
        assert_eq!(fmt_duration(90060, ("วัน", "ชม.", "น.")), "1วัน 1ชม. 1น.");
    }
}
