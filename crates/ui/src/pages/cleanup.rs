//! System and storage cleanup page.

use crate::theme::ThemeColors;
use crate::widgets::Card;
use egui::Ui;
use smart_filee_cleanup::{find_cleanup, CleanupItem, CleanupKind};
use smart_filee_common::{fmt_bytes, CancellationToken};
use smart_filee_file_engine::trash_files;
use smart_filee_localization::tr;
use std::collections::HashSet;
use std::path::PathBuf;

pub struct CleanupPage {
    pub folder: String,
    pub min_mb: u64,
    pub old_days: u32,
    pub items: Vec<CleanupItem>,
    pub selected: HashSet<PathBuf>,
    pub status: String,
}

impl Default for CleanupPage {
    fn default() -> Self {
        Self {
            folder: String::new(),
            min_mb: 50,
            old_days: 180,
            items: Vec::new(),
            selected: HashSet::new(),
            status: String::new(),
        }
    }
}

impl CleanupPage {
    pub fn show(&mut self, ui: &mut Ui, colors: &ThemeColors) {
        ui.heading(tr("Cleanup"));
        ui.colored_label(
            colors.muted,
            "Detect temporary junk files, old unused large files, and empty directory trees",
        );
        ui.add_space(12.0);

        Card::show(ui, colors, |ui| {
            ui.horizontal(|ui| {
                ui.label(tr("Select folder"));
                ui.add(
                    egui::TextEdit::singleline(&mut self.folder)
                        .hint_text("Choose directory to clean...")
                        .desired_width(ui.available_width() - 280.0),
                );

                if ui.button(tr("Browse...")).clicked() {
                    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                        self.folder = folder.to_string_lossy().to_string();
                    }
                }

                if ui.button("Scan for Junk").clicked() && !self.folder.is_empty() {
                    let path = PathBuf::from(&self.folder);
                    if path.is_dir() {
                        let cancel = CancellationToken::new();

                        let res =
                            find_cleanup(&path, self.min_mb, self.old_days as f64, &cancel, |_| {});
                        let total_bytes: u64 = res.items.iter().map(|i| i.size).sum();
                        self.status = format!(
                            "Found {} cleanup targets ({})",
                            res.items.len(),
                            fmt_bytes(total_bytes)
                        );
                        self.items = res.items;
                        self.selected.clear();
                    }
                }
            });

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label("Min Large File (MB):");
                let mut mb_str = self.min_mb.to_string();
                if ui
                    .add(egui::TextEdit::singleline(&mut mb_str).desired_width(60.0))
                    .changed()
                {
                    if let Ok(v) = mb_str.parse::<u64>() {
                        self.min_mb = v;
                    }
                }

                ui.label("Age Threshold (Days):");
                let mut days_str = self.old_days.to_string();
                if ui
                    .add(egui::TextEdit::singleline(&mut days_str).desired_width(60.0))
                    .changed()
                {
                    if let Ok(v) = days_str.parse::<u32>() {
                        self.old_days = v;
                    }
                }
            });
        });

        if !self.status.is_empty() {
            ui.add_space(8.0);
            ui.colored_label(colors.accent_btn, &self.status);
        }

        ui.add_space(12.0);

        if !self.items.is_empty() {
            ui.horizontal(|ui| {
                if ui.button("Select All").clicked() {
                    for it in &self.items {
                        self.selected.insert(it.path.clone());
                    }
                }
                if ui.button("Deselect All").clicked() {
                    self.selected.clear();
                }

                let can_trash = !self.selected.is_empty();
                if ui
                    .add_enabled(
                        can_trash,
                        egui::Button::new(format!("Safe Clean Selected ({})", self.selected.len())),
                    )
                    .clicked()
                {
                    let to_trash: Vec<PathBuf> = self.selected.iter().cloned().collect();
                    let cancel = CancellationToken::new();
                    let (trashed, failed) = trash_files(&to_trash, &cancel, |_, _| {});
                    self.status = format!(
                        "Cleaned {} items successfully! ({} failed)",
                        trashed.len(),
                        failed.len()
                    );
                    self.selected.clear();
                    self.items.retain(|i| !trashed.contains(&i.path));
                }
            });

            ui.add_space(8.0);

            egui::ScrollArea::vertical()
                .max_height(350.0)
                .show(ui, |ui| {
                    for item in &self.items {
                        ui.horizontal(|ui| {
                            let mut is_selected = self.selected.contains(&item.path);
                            if ui.checkbox(&mut is_selected, "").changed() {
                                if is_selected {
                                    self.selected.insert(item.path.clone());
                                } else {
                                    self.selected.remove(&item.path);
                                }
                            }

                            let tag = match &item.kind {
                                CleanupKind::Temp => "[Temp]",
                                CleanupKind::OldLarge => "[Old Large]",
                                CleanupKind::EmptyDir => "[Empty Dir]",
                            };
                            ui.colored_label(colors.accent_btn, tag);
                            ui.label(item.path.file_name().unwrap_or_default().to_string_lossy());
                            if item.size > 0 {
                                ui.colored_label(colors.muted, fmt_bytes(item.size));
                            }
                            ui.colored_label(colors.muted, item.path.display().to_string());
                        });
                    }
                });
        }
    }
}
