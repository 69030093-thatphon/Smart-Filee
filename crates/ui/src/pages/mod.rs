//! Feature pages module for all 14 views.

pub mod backup;
pub mod cleanup;
pub mod dashboard;
pub mod disk;
pub mod dups;
pub mod log;
pub mod network;
pub mod organizer;
pub mod procs;
pub mod renamer;
pub mod search;
pub mod settings;
pub mod similar;
pub mod system;

pub use backup::BackupPage;
pub use cleanup::CleanupPage;
pub use dashboard::DashboardPage;
pub use disk::DiskPage;
pub use dups::DuplicatesPage;
pub use log::ActivityLogPage;
pub use network::NetworkPage;
pub use organizer::OrganizerPage;
pub use procs::ProcessesPage;
pub use renamer::RenamerPage;
pub use search::SearchPage;
pub use settings::SettingsPage;
pub use similar::SimilarPhotosPage;
pub use system::SystemInfoPage;
