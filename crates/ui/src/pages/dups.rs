//! Duplicate Finder page.

use crate::theme::ThemeColors;
use crate::widgets::Card;
use egui::Ui;
use smart_filee_common::{fmt_bytes, CancellationToken};
use smart_filee_duplicate::{find_duplicates, DuplicateGroup};
use smart_filee_file_engine::trash_files;
use smart_filee_localization::tr;
use std::collections::HashSet;
use std::path::PathBuf;

pub struct DuplicatesPage {
    pub folder: String,
    pub min_size_kb: u64,
    pub groups: Vec<DuplicateGroup>,
    pub selected_to_trash: HashSet<PathBuf>,
    pub status: String,
}

impl Default for DuplicatesPage {
    fn default() -> Self {
        Self {
            folder: String::new(),
            min_size_kb: 1,
            groups: Vec::new(),
            selected_to_trash: HashSet::new(),
            status: String::new(),
        }
    }
}

impl DuplicatesPage {
    pub fn show(&mut self, ui: &mut Ui, colors: &ThemeColors) {
        ui.heading(tr("Duplicate Finder"));
        ui.colored_label(
            colors.muted,
            "Identify identical files using staged SHA-256 detection and reclaim wasted storage",
        );
        ui.add_space(12.0);

        Card::show(ui, colors, |ui| {
            ui.horizontal(|ui| {
                ui.label(tr("Select folder"));
                ui.add(
                    egui::TextEdit::singleline(&mut self.folder)
                        .hint_text("Choose directory to scan...")
                        .desired_width(ui.available_width() - 280.0),
                );

                if ui.button(tr("Browse...")).clicked() {
                    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                        self.folder = folder.to_string_lossy().to_string();
                    }
                }

                if ui.button(tr("Find Duplicates")).clicked() && !self.folder.is_empty() {
                    let path = PathBuf::from(&self.folder);
                    if path.is_dir() {
                        let cancel = CancellationToken::new();
                        let res = find_duplicates(
                            &path,
                            self.min_size_kb * 1024,
                            None,
                            &cancel,
                            |_, _| {},
                        );
                        let total_wasted: u64 =
                            res.groups.iter().map(DuplicateGroup::wasted_bytes).sum();
                        self.status = format!(
                            "Found {} duplicate groups ({} recoverable)",
                            res.groups.len(),
                            fmt_bytes(total_wasted)
                        );
                        self.groups = res.groups;
                        self.selected_to_trash.clear();
                    }
                }
            });
        });

        if !self.status.is_empty() {
            ui.add_space(8.0);
            ui.colored_label(colors.accent_btn, &self.status);
        }

        ui.add_space(12.0);

        if !self.groups.is_empty() {
            ui.horizontal(|ui| {
                if ui
                    .button("Select All Duplicates (Keep Originals)")
                    .clicked()
                {
                    self.selected_to_trash.clear();
                    for g in &self.groups {
                        // Skip first (original/oldest), select all subsequent copies
                        for copy in g.files.iter().skip(1) {
                            self.selected_to_trash.insert(copy.path.clone());
                        }
                    }
                }

                let can_trash = !self.selected_to_trash.is_empty();
                if ui
                    .add_enabled(
                        can_trash,
                        egui::Button::new(format!(
                            "Safe Trash Selected ({})",
                            self.selected_to_trash.len()
                        )),
                    )
                    .clicked()
                {
                    let to_trash: Vec<PathBuf> = self.selected_to_trash.iter().cloned().collect();
                    let cancel = CancellationToken::new();
                    let (trashed, failed) = trash_files(&to_trash, &cancel, |_, _| {});
                    self.status = format!(
                        "Moved {} duplicate files to Recycle Bin ({} failed)",
                        trashed.len(),
                        failed.len()
                    );
                    self.selected_to_trash.clear();
                    // Refresh list
                    self.groups.retain_mut(|g| {
                        g.files.retain(|p| !trashed.contains(&p.path));
                        g.files.len() > 1
                    });
                }
            });

            ui.add_space(8.0);

            egui::ScrollArea::vertical()
                .max_height(350.0)
                .show(ui, |ui| {
                    for (g_idx, group) in self.groups.iter().enumerate() {
                        ui.label(format!(
                            "Group #{} — Each file: {}",
                            g_idx + 1,
                            fmt_bytes(group.size)
                        ));

                        for (c_idx, item) in group.files.iter().enumerate() {
                            ui.horizontal(|ui| {
                                if c_idx == 0 {
                                    ui.colored_label(colors.green, "[Original]");
                                    ui.label(item.path.display().to_string());
                                } else {
                                    let mut is_selected =
                                        self.selected_to_trash.contains(&item.path);
                                    if ui.checkbox(&mut is_selected, "[Copy]").changed() {
                                        if is_selected {
                                            self.selected_to_trash.insert(item.path.clone());
                                        } else {
                                            self.selected_to_trash.remove(&item.path);
                                        }
                                    }
                                    ui.colored_label(colors.muted, item.path.display().to_string());
                                }
                            });
                        }
                        ui.add_space(4.0);
                    }
                });
        }
    }
}
