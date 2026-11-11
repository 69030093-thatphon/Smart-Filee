//! Tidy Score metric computation (0..=100).

/// Computes the 0..=100 "how tidy are your files" score based on clutter metrics.
/// Starts at 100 and applies capped penalties:
/// - loose files: max -35 (0.25 pts per file)
/// - duplicates: max -25 (2.5 pts per dup file + 5.0 pts per GB)
/// - temp files: max -20 (temp_bytes / (50 MB))
/// - old large files: max -10 (2.0 pts per GB)
/// - empty folders: max -10 (2.0 pts per folder)
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn tidy_score(
    loose: u64,
    dup_files: u64,
    dup_bytes: u64,
    temp_bytes: u64,
    old_large_bytes: u64,
    empty_dirs: u64,
) -> u32 {
    let mb = 1_048_576.0_f64;
    let gb = 1_073_741_824.0_f64;

    let mut s: f64 = 100.0;
    s -= 35.0f64.min((loose as f64) * 0.25);
    s -= 25.0f64.min((dup_files as f64) * 2.5 + ((dup_bytes as f64) / gb) * 5.0);
    s -= 20.0f64.min((temp_bytes as f64) / (50.0 * mb));
    s -= 10.0f64.min(((old_large_bytes as f64) / gb) * 2.0);
    s -= 10.0f64.min((empty_dirs as f64) * 2.0);

    let clamped = s.clamp(0.0, 100.0);
    clamped.round() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_folder_scores_100_and_clutter_lowers_it() {
        assert_eq!(tidy_score(0, 0, 0, 0, 0, 0), 100);
        assert!(tidy_score(200, 0, 0, 0, 0, 0) < 100);
        assert!(
            tidy_score(200, 10, 0, 2 * 1024 * 1024 * 1024, 0, 0) < tidy_score(200, 0, 0, 0, 0, 0)
        );
        assert_eq!(
            tidy_score(
                1_000_000,
                1_000_000,
                1_000_000_000_000,
                1_000_000_000_000,
                1_000_000_000_000,
                999
            ),
            0
        );
    }
}
