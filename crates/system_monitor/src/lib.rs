//! System monitoring and inspection engine for Smart-Filee.
//!
//! Provides read-only metrics for CPU usage, memory, disk partitions, and active processes.

use serde::{Deserialize, Serialize};
use std::path::Path;
use sysinfo::{
    CpuRefreshKind, Disks, MemoryRefreshKind, ProcessRefreshKind, ProcessesToUpdate, RefreshKind,
    System,
};

/// Information about a mounted disk partition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiskPartitionInfo {
    pub name: String,
    pub mount_point: String,
    pub total_bytes: u64,
    pub free_bytes: u64,
}

/// Information about a running process (read-only).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessItem {
    pub pid: u32,
    pub name: String,
    pub cpu_percent: f32,
    pub memory_rss_bytes: u64,
}

/// Sorting order for the process list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProcessSort {
    #[default]
    Memory,
    Cpu,
    Name,
    Pid,
}

/// Comprehensive hardware and operating system snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSnapshot {
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub arch: String,
    pub cpu_brand: String,
    pub physical_cores: usize,
    pub logical_cores: usize,
    pub cpu_percent: f32,
    pub memory_used_bytes: u64,
    pub memory_total_bytes: u64,
    pub boot_time_secs: u64,
    pub disks: Vec<DiskPartitionInfo>,
}

/// System monitor holding reusable sysinfo structures.
#[derive(Debug)]
pub struct SystemMonitor {
    sys: System,
    disks: Disks,
}

impl Default for SystemMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemMonitor {
    /// Creates a new `SystemMonitor` with minimal initial refresh.
    pub fn new() -> Self {
        let refresh = RefreshKind::new()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything());

        let mut sys = System::new_with_specifics(refresh);
        sys.refresh_all();
        let disks = Disks::new_with_refreshed_list();

        Self { sys, disks }
    }

    /// Refreshes CPU, memory, and disk statistics.
    pub fn refresh(&mut self) {
        self.sys.refresh_cpu_usage();
        self.sys.refresh_memory();
        self.disks.refresh();
    }

    /// Captures a point-in-time system metrics snapshot.
    pub fn snapshot(&mut self) -> SystemSnapshot {
        self.refresh();

        let cpus = self.sys.cpus();
        let cpu_percent = self.sys.global_cpu_usage();
        let cpu_brand = cpus
            .first()
            .map_or_else(|| "Unknown CPU".to_string(), |c| c.brand().to_string());

        let logical_cores = cpus.len();
        let physical_cores = self.sys.physical_core_count().unwrap_or(logical_cores);

        let mut disk_list = Vec::new();
        for disk in &self.disks {
            disk_list.push(DiskPartitionInfo {
                name: disk.name().to_string_lossy().to_string(),
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                total_bytes: disk.total_space(),
                free_bytes: disk.available_space(),
            });
        }

        SystemSnapshot {
            os_name: System::name().unwrap_or_else(|| "Unknown OS".to_string()),
            os_version: System::os_version().unwrap_or_default(),
            kernel_version: System::kernel_version().unwrap_or_default(),
            arch: System::cpu_arch().unwrap_or_default(),
            cpu_brand,
            physical_cores,
            logical_cores,
            cpu_percent,
            memory_used_bytes: self.sys.used_memory(),
            memory_total_bytes: self.sys.total_memory(),
            boot_time_secs: System::boot_time(),
            disks: disk_list,
        }
    }

    /// Returns disk usage for a specific path (e.g. user home anchor).
    pub fn disk_usage_for_path(&self, target_path: &Path) -> Option<DiskPartitionInfo> {
        let canonical = target_path.canonicalize().ok()?;
        let mut best_match: Option<&sysinfo::Disk> = None;
        let mut best_len = 0;

        for disk in &self.disks {
            let mp = disk.mount_point();
            if canonical.starts_with(mp) {
                let len = mp.as_os_str().len();
                if len >= best_len {
                    best_len = len;
                    best_match = Some(disk);
                }
            }
        }

        best_match.map(|disk| DiskPartitionInfo {
            name: disk.name().to_string_lossy().to_string(),
            mount_point: disk.mount_point().to_string_lossy().to_string(),
            total_bytes: disk.total_space(),
            free_bytes: disk.available_space(),
        })
    }

    /// Lists running processes with optional case-insensitive name filter and sort order.
    /// Strictly read-only; no kill or termination capability.
    pub fn list_processes(&mut self, query: Option<&str>, sort: ProcessSort) -> Vec<ProcessItem> {
        self.sys
            .refresh_processes_specifics(ProcessesToUpdate::All, ProcessRefreshKind::everything());

        let q_lower = query.map(str::to_ascii_lowercase);
        let mut items = Vec::new();

        for (&pid, proc_info) in self.sys.processes() {
            let name = proc_info.name().to_string_lossy().to_string();
            if let Some(ref q) = q_lower {
                if !name.to_ascii_lowercase().contains(q) {
                    continue;
                }
            }

            items.push(ProcessItem {
                pid: pid.as_u32(),
                name,
                cpu_percent: proc_info.cpu_usage(),
                memory_rss_bytes: proc_info.memory(),
            });
        }

        match sort {
            ProcessSort::Memory => {
                items.sort_by_key(|p| std::cmp::Reverse(p.memory_rss_bytes));
            }
            ProcessSort::Cpu => {
                items.sort_by(|a, b| {
                    b.cpu_percent
                        .partial_cmp(&a.cpu_percent)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            ProcessSort::Name => {
                items.sort_by(|a, b| {
                    a.name
                        .to_ascii_lowercase()
                        .cmp(&b.name.to_ascii_lowercase())
                });
            }
            ProcessSort::Pid => {
                items.sort_by_key(|p| p.pid);
            }
        }

        items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_snapshot_and_metrics() {
        let mut monitor = SystemMonitor::new();
        let snap = monitor.snapshot();

        assert!(!snap.os_name.is_empty());
        assert!(snap.memory_total_bytes > 0);
        assert!(snap.logical_cores > 0);
    }

    #[test]
    fn test_list_processes_read_only() {
        let mut monitor = SystemMonitor::new();
        let procs = monitor.list_processes(None, ProcessSort::Memory);

        // There should be at least 1 running process (our own test process)
        assert!(!procs.is_empty());
        let current_pid = std::process::id();
        assert!(procs.iter().any(|p| p.pid == current_pid));
    }
}
