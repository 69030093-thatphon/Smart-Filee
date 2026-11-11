//! Similar Photos page.

use crate::theme::ThemeColors;
use crate::widgets::Card;
use egui::Ui;
use smart_filee_common::CancellationToken;
use smart_filee_file_engine::trash_files;
use smart_filee_image_engine::{find_similar_images, SimilarGroup};
use smart_filee_localization::tr;
use std::collections::HashSet;
use std::path::PathBuf;

pub struct SimilarPhotosPage {
    pub folder: String,
    pub max_dist: u32,
    pub groups: Vec<SimilarGroup>,
    pub selected_to_trash: HashSet<PathBuf>,
    pub status: String,
}

impl Default for SimilarPhotosPage {
    fn default() -> Self {
        Self {
            folder: String::new(),
            max_dist: 6,
            groups: Vec::new(),
            selected_to_trash: HashSet::new(),
            status: String::new(),
        }
    }
}

impl SimilarPhotosPage {
    pub fn show(&mut self, ui: &mut Ui, colors: &ThemeColors) {
        ui.heading(tr("Similar Photos"));
        ui.colored_label(
            colors.muted,
            "Detect visually similar photos and burst shots using perceptual hashing",
        );
        ui.add_space(12.0);

        Card::show(ui, colors, |ui| {
            ui.horizontal(|ui| {
                ui.label(tr("Select folder"));
                ui.add(
                    egui::TextEdit::singleline(&mut self.folder)
                        .hint_text("Choose photos folder...")
                        .desired_width(ui.available_width() - 280.0),
                );

                if ui.button(tr("Browse...")).clicked() {
                    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                        self.folder = folder.to_string_lossy().to_string();
                    }
                }

                if ui.button("Find Similar Photos").clicked() && !self.folder.is_empty() {
                    let path = PathBuf::from(&self.folder);
                    if path.is_dir() {
                        let cancel = CancellationToken::new();
                        let res = find_similar_images(&path, self.max_dist, &cancel, |_| {});
                        self.status =
                            format!("Found {} visually similar clusters", res.groups.len());
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
                        "Moved {} photos to Recycle Bin ({} failed)",
                        trashed.len(),
                        failed.len()
                    );
                    self.selected_to_trash.clear();
                    self.groups.retain_mut(|g| {
                        g.items.retain(|p| !trashed.contains(&p.path));
                        g.items.len() > 1
                    });
                }
            });

            ui.add_space(8.0);

            egui::ScrollArea::vertical()
                .max_height(350.0)
                .show(ui, |ui| {
                    for (g_idx, group) in self.groups.iter().enumerate() {
                        ui.label(format!(
                            "Cluster #{} ({} photos)",
                            g_idx + 1,
                            group.items.len()
                        ));

                        for (c_idx, photo) in group.items.iter().enumerate() {
                            ui.horizontal(|ui| {
                                if c_idx == 0 {
                                    ui.colored_label(colors.green, "[Best]");
                                    ui.label(
                                        photo
                                            .path
                                            .file_name()
                                            .unwrap_or_default()
                                            .to_string_lossy(),
                                    );
                                    ui.colored_label(
                                        colors.muted,
                                        photo.path.display().to_string(),
                                    );
                                } else {
                                    let mut is_selected =
                                        self.selected_to_trash.contains(&photo.path);
                                    if ui.checkbox(&mut is_selected, "[Similar]").changed() {
                                        if is_selected {
                                            self.selected_to_trash.insert(photo.path.clone());
                                        } else {
                                            self.selected_to_trash.remove(&photo.path);
                                        }
                                    }
                                    ui.label(
                                        photo
                                            .path
                                            .file_name()
                                            .unwrap_or_default()
                                            .to_string_lossy(),
                                    );
                                    ui.colored_label(
                                        colors.muted,
                                        photo.path.display().to_string(),
                                    );
                                }
                            });
                        }
                        ui.add_space(6.0);
                    }
                });
        }
    }
}
