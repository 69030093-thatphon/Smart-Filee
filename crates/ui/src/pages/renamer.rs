//! Bulk Renamer page.

use crate::theme::ThemeColors;
use crate::widgets::Card;
use egui::Ui;
use smart_filee_common::CancellationToken;
use smart_filee_localization::tr;
use smart_filee_renamer::{
    apply_rename, plan_rename, CaseTransform, RenameOptions, RenamePlanItem, RenameStatus,
    SpaceTransform,
};
use std::path::PathBuf;

#[derive(Default)]
pub struct RenamerPage {
    pub folder: String,
    pub find_text: String,
    pub replace_text: String,
    pub prefix: String,
    pub suffix: String,
    pub case_mode: usize, // 0: Keep, 1: Lower, 2: Upper, 3: Title
    pub add_numbering: bool,
    pub plan: Vec<RenamePlanItem>,
    pub status: String,
}

impl RenamerPage {
    pub fn show(&mut self, ui: &mut Ui, colors: &ThemeColors) {
        ui.heading(tr("Bulk Renamer"));
        ui.colored_label(
            colors.muted,
            "Transform filenames in bulk with text replacement, case formatting, and numbering",
        );
        ui.add_space(12.0);

        Card::show(ui, colors, |ui| {
            ui.horizontal(|ui| {
                ui.label(tr("Select folder"));
                ui.add(
                    egui::TextEdit::singleline(&mut self.folder)
                        .hint_text("Choose directory with files to rename...")
                        .desired_width(ui.available_width() - 280.0),
                );

                if ui.button(tr("Browse...")).clicked() {
                    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                        self.folder = folder.to_string_lossy().to_string();
                    }
                }
            });

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label("Find:");
                ui.add(egui::TextEdit::singleline(&mut self.find_text).desired_width(120.0));
                ui.label("Replace:");
                ui.add(egui::TextEdit::singleline(&mut self.replace_text).desired_width(120.0));
            });

            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label("Prefix:");
                ui.add(egui::TextEdit::singleline(&mut self.prefix).desired_width(100.0));
                ui.label("Suffix:");
                ui.add(egui::TextEdit::singleline(&mut self.suffix).desired_width(100.0));

                ui.label("Case:");
                ui.selectable_value(&mut self.case_mode, 0, "Original");
                ui.selectable_value(&mut self.case_mode, 1, "lower");
                ui.selectable_value(&mut self.case_mode, 2, "UPPER");
                ui.selectable_value(&mut self.case_mode, 3, "Title Case");
            });

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("Preview Rename Plan").clicked() && !self.folder.is_empty() {
                    let path = PathBuf::from(&self.folder);
                    if path.is_dir() {
                        let mut files = Vec::new();
                        if let Ok(entries) = std::fs::read_dir(&path) {
                            for e in entries.flatten() {
                                if e.path().is_file() {
                                    files.push(e.path());
                                }
                            }
                        }

                        let case_transform = match self.case_mode {
                            1 => CaseTransform::Lowercase,
                            2 => CaseTransform::Uppercase,
                            3 => CaseTransform::TitleCase,
                            _ => CaseTransform::Keep,
                        };

                        let opts = RenameOptions {
                            find: &self.find_text,
                            replace: &self.replace_text,
                            prefix: &self.prefix,
                            suffix: &self.suffix,
                            case: case_transform,
                            spaces: SpaceTransform::Keep,
                            number: self.add_numbering,
                            start: 1,
                            pad: 3,
                        };

                        let plan = plan_rename(&files, &opts);
                        let conflicts = plan
                            .iter()
                            .filter(|p| {
                                matches!(
                                    p.status,
                                    RenameStatus::Conflict
                                        | RenameStatus::Invalid
                                        | RenameStatus::Protected
                                )
                            })
                            .count();
                        self.status = format!(
                            "Planned {} renames ({} conflicts/invalid)",
                            plan.len(),
                            conflicts
                        );
                        self.plan = plan;
                    }
                }

                let has_errors = self.plan.iter().any(|p| {
                    matches!(
                        p.status,
                        RenameStatus::Conflict | RenameStatus::Invalid | RenameStatus::Protected
                    )
                });
                let can_apply = !self.plan.is_empty() && !has_errors;
                if ui
                    .add_enabled(can_apply, egui::Button::new("Apply Rename"))
                    .clicked()
                {
                    let cancel = CancellationToken::new();
                    let (applied, failed) = apply_rename(&self.plan, &cancel, |_, _| {});
                    self.status = format!(
                        "Renamed {} files successfully! ({} failed)",
                        applied.len(),
                        failed.len()
                    );
                    self.plan.clear();
                }
            });
        });

        if !self.status.is_empty() {
            ui.add_space(8.0);
            ui.colored_label(colors.accent_btn, &self.status);
        }

        ui.add_space(12.0);

        if !self.plan.is_empty() {
            ui.heading("Rename Plan Preview");
            ui.add_space(4.0);

            egui::ScrollArea::vertical()
                .max_height(350.0)
                .show(ui, |ui| {
                    for item in &self.plan {
                        ui.horizontal(|ui| {
                            let old_name = item
                                .original
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy();
                            ui.label(old_name);
                            ui.label("➔");
                            let tag_color = match &item.status {
                                RenameStatus::OK => colors.accent_btn,
                                RenameStatus::Unchanged => colors.muted,
                                RenameStatus::Conflict
                                | RenameStatus::Invalid
                                | RenameStatus::Protected => colors.red,
                            };
                            ui.colored_label(tag_color, &item.new_name);
                        });
                    }
                });
        }
    }
}
