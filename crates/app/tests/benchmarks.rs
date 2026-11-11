//! Phase 9: Performance Benchmarks & Rust vs Python Validation.

use smart_filee_common::CancellationToken;
use smart_filee_database::Database;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

fn make_temp_dir(tag: &str) -> PathBuf {
    let p = std::env::temp_dir().join(format!("sfm_bench_{}_{}", tag, std::process::id()));
    let _ = fs::remove_dir_all(&p);
    fs::create_dir_all(&p).expect("create_dir");
    p
}

#[test]
fn bench_hashing_throughput() {
    let temp_dir = make_temp_dir("hash");
    let test_file = temp_dir.join("50mb_test.bin");

    // Create 50MB synthetic binary file
    {
        let mut f = File::create(&test_file).expect("Failed to create file");
        let chunk = vec![0xAB_u8; 1024 * 1024]; // 1MB chunk
        for _ in 0..50 {
            f.write_all(&chunk).expect("Write failed");
        }
    }

    let cancel = CancellationToken::new();
    let start = Instant::now();
    let hash = smart_filee_duplicate::hash_file(&test_file, None, &cancel).expect("Hashing failed");
    let elapsed = start.elapsed();

    let secs = elapsed.as_secs_f64();
    let throughput_mb_s = 50.0 / secs;

    println!("\n=== Hashing Throughput Benchmark ===");
    println!("File size: 50 MB");
    println!("SHA-256: {hash}");
    println!("Time: {secs:.4} s");
    println!("Throughput: {throughput_mb_s:.1} MB/s");

    assert!(!hash.is_empty());
    let min_expected = if cfg!(debug_assertions) { 15.0 } else { 100.0 };
    assert!(
        throughput_mb_s > min_expected,
        "Hashing throughput ({throughput_mb_s:.1} MB/s) should exceed {min_expected} MB/s"
    );
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn bench_traversal_speed() {
    let temp_dir = make_temp_dir("traversal");
    let root = temp_dir.join("dataset");
    fs::create_dir_all(&root).expect("create_dir");

    // Generate 1,000 files in 20 subdirectories
    for d in 0..20 {
        let sub = root.join(format!("sub_{d}"));
        fs::create_dir_all(&sub).expect("create_dir");
        for f in 0..50 {
            let file_path = sub.join(format!("file_{f}.txt"));
            fs::write(file_path, "sample content").expect("write");
        }
    }

    let cancel = CancellationToken::new();
    let mut errs = Vec::new();

    let start = Instant::now();
    let files = smart_filee_file_engine::walk_files(&root, &cancel, None, &mut errs, |_| {});
    let elapsed = start.elapsed();

    let count = files.len();
    let secs = elapsed.as_secs_f64();
    let rate = count as f64 / secs;

    println!("\n=== Traversal Throughput Benchmark ===");
    println!("Files scanned: {count}");
    println!("Time: {secs:.4} s");
    println!("Rate: {rate:.1} files/s");

    assert_eq!(count, 1000);
    assert!(rate > 1000.0, "Traversal should exceed 1000 files/s");
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn bench_database_batch_write_and_read() {
    let temp_dir = make_temp_dir("db");
    let db_path = temp_dir.join("bench.db");
    let db = Database::open(&db_path).expect("Failed to open db");

    let pairs: Vec<(PathBuf, PathBuf)> = (0..500)
        .map(|i| {
            (
                PathBuf::from(format!("/tmp/src_{i}.txt")),
                PathBuf::from(format!("/tmp/dest_{i}.txt")),
            )
        })
        .collect();

    let start = Instant::now();
    for _ in 0..10 {
        db.push_undo("Bench Batch", &pairs);
    }
    let elapsed = start.elapsed();

    let history = db.undo_history(100);
    println!("\n=== SQLite WAL Performance Benchmark ===");
    println!("Total Batches in History: {}", history.len());
    println!("Elapsed time: {:.4} s", elapsed.as_secs_f64());

    assert_eq!(history.len(), 10);
    let _ = fs::remove_dir_all(&temp_dir);
}
