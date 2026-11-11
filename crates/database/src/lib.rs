//! SQLite persistence engine for Smart-Filee.
//!
//! Replicates `sfm.db` schema and operations:
//! - Meta key/value store
//! - Undo batches & moves journal with stale batch tracking
//! - Activity log with 5,000-line automatic pruning
//! - Custom organizer rules with ordering
//! - SHA-256 hash cache with 90-day pruning
//! - Scan history & aggregated statistics

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::SystemTime;

use chrono::Local;
use rusqlite::{params, Connection};
use smart_filee_duplicate::DuplicateHashCache;

pub const SCHEMA_VERSION: i32 = 1;
pub const UNDO_LIMIT: usize = 20;
pub const LOG_KEEP: i64 = 5000;
pub const CACHE_MAX_AGE_DAYS: u64 = 90;

const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS meta (
    key   TEXT PRIMARY KEY,
    value TEXT
);
CREATE TABLE IF NOT EXISTS undo_batches (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    label      TEXT    NOT NULL,
    created_at TEXT    NOT NULL,
    status     TEXT    NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'undone', 'stale'))
);
CREATE TABLE IF NOT EXISTS undo_moves (
    id       INTEGER PRIMARY KEY AUTOINCREMENT,
    batch_id INTEGER NOT NULL REFERENCES undo_batches(id) ON DELETE CASCADE,
    position INTEGER NOT NULL,
    src      TEXT    NOT NULL,
    dst      TEXT    NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_undo_moves_batch ON undo_moves(batch_id);
CREATE TABLE IF NOT EXISTS activity_log (
    id      INTEGER PRIMARY KEY AUTOINCREMENT,
    ts      TEXT NOT NULL,
    message TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS rules (
    id       INTEGER PRIMARY KEY AUTOINCREMENT,
    position INTEGER NOT NULL,
    pattern  TEXT    NOT NULL,
    folder   TEXT    NOT NULL
);
CREATE TABLE IF NOT EXISTS hash_cache (
    path    TEXT    NOT NULL,
    kind    TEXT    NOT NULL CHECK (kind IN ('head', 'full')),
    size    INTEGER NOT NULL,
    mtime   REAL    NOT NULL,
    hash    TEXT    NOT NULL,
    updated REAL    NOT NULL,
    PRIMARY KEY (path, kind)
);
CREATE TABLE IF NOT EXISTS scan_history (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    ts           TEXT    NOT NULL,
    kind         TEXT    NOT NULL,
    root         TEXT    NOT NULL,
    scanned      INTEGER NOT NULL DEFAULT 0,
    groups       INTEGER NOT NULL DEFAULT 0,
    dup_files    INTEGER NOT NULL DEFAULT 0,
    wasted_bytes INTEGER NOT NULL DEFAULT 0,
    cache_hits   INTEGER NOT NULL DEFAULT 0,
    cache_misses INTEGER NOT NULL DEFAULT 0,
    seconds      REAL    NOT NULL DEFAULT 0
);
"#;

fn now_str() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

fn now_epoch_secs() -> f64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_or(0.0, |d| d.as_secs_f64())
}

/// Information about an undo batch row in the database.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UndoBatchRow {
    pub id: i64,
    pub label: String,
    pub created_at: String,
    pub status: String,
    pub move_count: i64,
}

/// Statistics summary for the Activity Log page.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DatabaseStats {
    pub actions: i64,
    pub files_moved: i64,
    pub undone: i64,
    pub scans: i64,
    pub wasted_bytes: i64,
    pub cached_hashes: i64,
    pub log_lines: i64,
}

/// Thread-safe SQLite persistence wrapper for Smart-Filee.
#[derive(Debug)]
pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    /// Opens or creates an SQLite database at `path`. Accepts `":memory:"` for tests.
    pub fn open(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let p = path.as_ref();
        let conn = if p.as_os_str() == ":memory:" {
            Connection::open_in_memory()?
        } else {
            if let Some(parent) = p.parent() {
                fs::create_dir_all(parent)?;
            }
            Connection::open(p)?
        };

        let db = Self {
            conn: Mutex::new(conn),
        };
        db.init()?;
        Ok(db)
    }

    fn init(&self) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;
        let _ = conn.execute_batch("PRAGMA journal_mode = WAL;");
        conn.execute_batch(SCHEMA)?;

        conn.execute(
            "INSERT OR REPLACE INTO meta(key, value) VALUES ('schema_version', ?)",
            params![SCHEMA_VERSION.to_string()],
        )?;

        drop(conn);
        self.maintenance();
        Ok(())
    }

    /// Trims old logs exceeding 5,000 entries and cache entries older than 90 days.
    pub fn maintenance(&self) {
        let conn = self.conn.lock().unwrap();
        let _ = conn.execute(
            "DELETE FROM activity_log WHERE id <= (SELECT COALESCE(MAX(id), 0) FROM activity_log) - ?",
            params![LOG_KEEP],
        );
        let cutoff = now_epoch_secs() - (CACHE_MAX_AGE_DAYS as f64 * 86_400.0);
        let _ = conn.execute("DELETE FROM hash_cache WHERE updated < ?", params![cutoff]);
    }

    // ------------------------------------------------------------------ meta
    pub fn get_meta(&self, key: &str) -> Option<String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT value FROM meta WHERE key = ?").ok()?;
        stmt.query_row(params![key], |row| row.get(0)).ok()
    }

    pub fn set_meta(&self, key: &str, value: &str) {
        let conn = self.conn.lock().unwrap();
        let _ = conn.execute(
            "INSERT OR REPLACE INTO meta(key, value) VALUES (?, ?)",
            params![key, value],
        );
    }

    // ------------------------------------------------------------------ undo
    pub fn push_undo(&self, label: &str, moves: &[(PathBuf, PathBuf)]) -> i64 {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction().unwrap();

        tx.execute(
            "INSERT INTO undo_batches(label, created_at) VALUES (?, ?)",
            params![label, now_str()],
        )
        .unwrap();
        let batch_id = tx.last_insert_rowid();

        {
            let mut stmt = tx
                .prepare("INSERT INTO undo_moves(batch_id, position, src, dst) VALUES (?, ?, ?, ?)")
                .unwrap();
            for (i, (s, d)) in moves.iter().enumerate() {
                stmt.execute(params![
                    batch_id,
                    i as i64,
                    s.to_string_lossy(),
                    d.to_string_lossy()
                ])
                .unwrap();
            }
        }

        tx.commit().unwrap();
        batch_id
    }

    pub fn pop_undo(&self) {
        let conn = self.conn.lock().unwrap();
        let _ = conn.execute(
            "UPDATE undo_batches SET status='undone' WHERE id = (SELECT MAX(id) FROM undo_batches WHERE status='active')",
            [],
        );
    }

    /// Loads active batches with existing destination files. Marks vanished batches as stale.
    pub fn load_undo_stack(&self, limit: usize) -> Vec<(String, Vec<(PathBuf, PathBuf)>)> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT id, label FROM undo_batches WHERE status='active' ORDER BY id DESC LIMIT ?",
            )
            .unwrap();

        let batch_rows: Vec<(i64, String)> = stmt
            .query_map(params![limit as i64], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .flatten()
            .collect();

        let mut out = Vec::new();

        // Process in chronological order (oldest first)
        for (bid, label) in batch_rows.into_iter().rev() {
            let mut m_stmt = conn
                .prepare("SELECT src, dst FROM undo_moves WHERE batch_id = ? ORDER BY position")
                .unwrap();

            let move_rows: Vec<(String, String)> = m_stmt
                .query_map(params![bid], |row| Ok((row.get(0)?, row.get(1)?)))
                .unwrap()
                .flatten()
                .collect();

            let surviving_moves: Vec<(PathBuf, PathBuf)> = move_rows
                .into_iter()
                .map(|(s, d)| (PathBuf::from(s), PathBuf::from(d)))
                .filter(|(_, d)| d.exists())
                .collect();

            if surviving_moves.is_empty() {
                let _ = conn.execute(
                    "UPDATE undo_batches SET status='stale' WHERE id = ?",
                    params![bid],
                );
            } else {
                out.push((label, surviving_moves));
            }
        }

        out
    }

    pub fn undo_history(&self, limit: usize) -> Vec<UndoBatchRow> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                r#"SELECT b.id, b.label, b.created_at, b.status, COUNT(m.id)
                   FROM undo_batches b LEFT JOIN undo_moves m ON m.batch_id = b.id
                   GROUP BY b.id ORDER BY b.id DESC LIMIT ?"#,
            )
            .unwrap();

        stmt.query_map(params![limit as i64], |row| {
            Ok(UndoBatchRow {
                id: row.get(0)?,
                label: row.get(1)?,
                created_at: row.get(2)?,
                status: row.get(3)?,
                move_count: row.get(4)?,
            })
        })
        .unwrap()
        .flatten()
        .collect()
    }

    // ------------------------------------------------------------------- log
    pub fn add_log(&self, message: &str) {
        let conn = self.conn.lock().unwrap();
        let _ = conn.execute(
            "INSERT INTO activity_log(ts, message) VALUES (?, ?)",
            params![now_str(), message],
        );
    }

    pub fn recent_logs(&self, limit: usize, query: Option<&str>) -> Vec<String> {
        let conn = self.conn.lock().unwrap();
        let mut sql = "SELECT ts, message FROM activity_log".to_string();
        let mut params_vec: Vec<rusqlite::types::Value> = Vec::new();

        if let Some(q) = query {
            if !q.trim().is_empty() {
                sql.push_str(" WHERE message LIKE ? ESCAPE '\\'");
                let escaped = format!(
                    "%{}%",
                    q.replace('\\', "\\\\")
                        .replace('%', "\\%")
                        .replace('_', "\\_")
                );
                params_vec.push(escaped.into());
            }
        }

        sql.push_str(" ORDER BY id DESC LIMIT ?");
        params_vec.push((limit as i64).into());

        let mut stmt = conn.prepare(&sql).unwrap();
        let rusqlite_params = rusqlite::params_from_iter(params_vec);

        stmt.query_map(rusqlite_params, |row| {
            let ts: String = row.get(0)?;
            let msg: String = row.get(1)?;
            Ok(format!("[{ts}] {msg}"))
        })
        .unwrap()
        .flatten()
        .collect()
    }

    pub fn clear_logs(&self) {
        let conn = self.conn.lock().unwrap();
        let _ = conn.execute("DELETE FROM activity_log", []);
    }

    // ----------------------------------------------------------------- rules
    pub fn get_rules(&self) -> Vec<(String, String)> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT pattern, folder FROM rules ORDER BY position")
            .unwrap();

        stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .flatten()
            .collect()
    }

    pub fn set_rules(&self, rules: &[(String, String)]) {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction().unwrap();

        tx.execute("DELETE FROM rules", []).unwrap();
        {
            let mut stmt = tx
                .prepare("INSERT INTO rules(position, pattern, folder) VALUES (?, ?, ?)")
                .unwrap();
            for (i, (pat, folder)) in rules.iter().enumerate() {
                stmt.execute(params![i as i64, pat, folder]).unwrap();
            }
        }
        tx.commit().unwrap();
    }

    // --------------------------------------------------------- scan history
    #[allow(clippy::too_many_arguments)]
    pub fn add_scan(
        &self,
        kind: &str,
        root: &Path,
        scanned: usize,
        groups: usize,
        dup_files: usize,
        wasted_bytes: u64,
        cache_hits: usize,
        cache_misses: usize,
        seconds: f64,
    ) {
        let conn = self.conn.lock().unwrap();
        let _ = conn.execute(
            r#"INSERT INTO scan_history(ts, kind, root, scanned, groups, dup_files, wasted_bytes,
                                cache_hits, cache_misses, seconds) VALUES (?,?,?,?,?,?,?,?,?,?)"#,
            params![
                now_str(),
                kind,
                root.to_string_lossy(),
                scanned as i64,
                groups as i64,
                dup_files as i64,
                wasted_bytes as i64,
                cache_hits as i64,
                cache_misses as i64,
                seconds
            ],
        );
    }

    pub fn stats(&self) -> DatabaseStats {
        let conn = self.conn.lock().unwrap();
        let q = |sql: &str| -> i64 { conn.query_row(sql, [], |r| r.get(0)).unwrap_or(0) };

        DatabaseStats {
            actions: q("SELECT COUNT(*) FROM undo_batches"),
            files_moved: q(
                "SELECT COUNT(*) FROM undo_moves m JOIN undo_batches b ON b.id = m.batch_id",
            ),
            undone: q("SELECT COUNT(*) FROM undo_batches WHERE status = 'undone'"),
            scans: q("SELECT COUNT(*) FROM scan_history"),
            wasted_bytes: q("SELECT COALESCE(MAX(wasted_bytes), 0) FROM scan_history"),
            cached_hashes: q("SELECT COUNT(*) FROM hash_cache"),
            log_lines: q("SELECT COUNT(*) FROM activity_log"),
        }
    }
}

/// Thread-safe caching bridge implementing `smart_filee_duplicate::DuplicateHashCache`.
#[derive(Debug)]
pub struct DbHashCache<'a> {
    db: &'a Database,
    pub hits: usize,
    pub misses: usize,
}

impl<'a> DbHashCache<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self {
            db,
            hits: 0,
            misses: 0,
        }
    }
}

impl DuplicateHashCache for DbHashCache<'_> {
    fn get(&mut self, path: &Path, size: u64, mtime: f64, kind: &str) -> Option<String> {
        let conn = self.db.conn.lock().unwrap();
        let p_str = path.to_string_lossy();
        let mut stmt = conn
            .prepare("SELECT hash FROM hash_cache WHERE path = ? AND kind = ? AND size = ? AND mtime = ?")
            .ok()?;

        let hash_opt: Option<String> = stmt
            .query_row(params![p_str, kind, size as i64, mtime], |r| r.get(0))
            .ok();

        if let Some(h) = hash_opt {
            self.hits += 1;
            Some(h)
        } else {
            self.misses += 1;
            None
        }
    }

    fn put(&mut self, path: &Path, size: u64, mtime: f64, kind: &str, digest: &str) {
        let conn = self.db.conn.lock().unwrap();
        let p_str = path.to_string_lossy();
        let _ = conn.execute(
            r#"INSERT OR REPLACE INTO hash_cache(path, kind, size, mtime, hash, updated)
               VALUES (?, ?, ?, ?, ?, ?)"#,
            params![p_str, kind, size as i64, mtime, digest, now_epoch_secs()],
        );
    }

    fn flush(&mut self) {
        // Transactions are managed per operation or autocommitted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_undo_push_pop_and_reload() {
        let db = Database::open(":memory:").unwrap();
        let f1 = PathBuf::from("/tmp/f1.txt");
        let f2 = PathBuf::from("/tmp/f2.txt");

        let bid = db.push_undo("Test action", &[(f1.clone(), f2.clone())]);
        assert!(bid > 0);

        let history = db.undo_history(10);
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].label, "Test action");
        assert_eq!(history[0].status, "active");

        db.pop_undo();
        let history2 = db.undo_history(10);
        assert_eq!(history2[0].status, "undone");
    }

    #[test]
    fn test_db_logs_filter_and_clear() {
        let db = Database::open(":memory:").unwrap();
        db.add_log("Starting engine");
        db.add_log("Moved file A");
        db.add_log("Finished work");

        let all = db.recent_logs(10, None);
        assert_eq!(all.len(), 3);

        let filtered = db.recent_logs(10, Some("Moved"));
        assert_eq!(filtered.len(), 1);
        assert!(filtered[0].contains("Moved file A"));

        db.clear_logs();
        assert!(db.recent_logs(10, None).is_empty());
    }

    #[test]
    fn test_db_rules_keep_order() {
        let db = Database::open(":memory:").unwrap();
        let rules = vec![
            ("*.jpg".to_string(), "Photos".to_string()),
            ("*.rs".to_string(), "RustCode".to_string()),
        ];
        db.set_rules(&rules);

        let fetched = db.get_rules();
        assert_eq!(fetched, rules);
    }

    #[test]
    fn test_db_hash_cache() {
        let db = Database::open(":memory:").unwrap();
        let mut cache = DbHashCache::new(&db);
        let p = Path::new("/test/image.png");

        assert_eq!(cache.get(p, 1024, 123.456, "head"), None);
        assert_eq!(cache.misses, 1);
        assert_eq!(cache.hits, 0);

        cache.put(p, 1024, 123.456, "head", "sha256_mock_hash");
        assert_eq!(
            cache.get(p, 1024, 123.456, "head"),
            Some("sha256_mock_hash".to_string())
        );
        assert_eq!(cache.hits, 1);
    }
}
