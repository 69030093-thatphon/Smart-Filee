//! Differential Backup page.

use crate::theme::ThemeColors;
use crate::widgets::Card;
use egui::Ui;
use smart_filee_backup::{plan_backup, run_backup, BackupPlanResult};
use smart_filee_common::{fmt_bytes, CancellationToken};
use smart_filee_localization::tr;
use std::path::PathBuf;

#[derive(Default)]
pub struct BackupPage {
    pub source: String,
    pub destination: String,
    pub plan: Option<BackupPlanResult>,
    pub status: String,
}

impl BackupPage {
    pub fn show(&mut self, ui: &mut Ui, colors: &ThemeColors) {
        ui.heading(tr("Backup"));
        ui.colored_label(
            colors.muted,
            "One-way differential backup copying only new and modified files with timestamp preservation",
        );
        ui.add_space(12.0);

        Card::show(ui, colors, |ui| {
            ui.horizontal(|ui| {
                ui.label("Source folder:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.source)
                        .hint_text("Source directory to backup...")
                        .desired_width(ui.available_width() - 200.0),
                );

                if ui.button("Browse...").clicked() {
                    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                        self.source = folder.to_string_lossy().to_string();
                    }
                }
            });

            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label("Destination:  ");
                ui.add(
                    egui::TextEdit::singleline(&mut self.destination)
                        .hint_text(
                            "Backup target directory (USB, external drive, backup folder)...",
                        )
                        .desired_width(ui.available_width() - 200.0),
                );

                if ui.button("Browse Target").clicked() {
                    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                        self.destination = folder.to_string_lossy().to_string();
                    }
                }
            });

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                let can_plan = !self.source.is_empty() && !self.destination.is_empty();
                if ui
                    .add_enabled(can_plan, egui::Button::new("Preview Backup"))
                    .clicked()
                {
                    let src = PathBuf::from(&self.source);
                    let dst = PathBuf::from(&self.destination);

                    if src.is_dir() {
                        let cancel = CancellationToken::new();
                        match plan_backup(&src, &dst, &cancel, |_| {}) {
                            Ok(plan) => {
                                self.status = format!(
                                    "Differential check: {} files to copy ({})",
                                    plan.plan.len(),
                                    fmt_bytes(plan.bytes)
                                );
                                self.plan = Some(plan);
                            }
                            Err(e) => {
                                self.status = format!("Backup plan error: {e}");
                            }
                        }
                    }
                }

                let can_run = self.plan.as_ref().is_some_and(|p| !p.plan.is_empty());
                if ui
                    .add_enabled(can_run, egui::Button::new("Run Differential Backup"))
                    .clicked()
                {
                    if let Some(plan) = &self.plan {
                        let cancel = CancellationToken::new();
                        let res = run_backup(&plan.plan, &cancel, |_| {});
                        self.status = format!(
                            "Backup complete! Successfully copied {} files ({})",
                            res.copied,
                            fmt_bytes(res.bytes)
                        );
                        self.plan = None;
                    }
                }
            });
        });

        if !self.status.is_empty() {
            ui.add_space(8.0);
            ui.colored_label(colors.accent_btn, &self.status);
        }

        ui.add_space(12.0);

        if let Some(plan) = &self.plan {
            ui.heading("Files to Backup");
            ui.add_space(4.0);

            egui::ScrollArea::vertical()
                .max_height(350.0)
                .show(ui, |ui| {
                    for item in &plan.plan {
                        ui.horizontal(|ui| {
                            let name = item.src.file_name().unwrap_or_default().to_string_lossy();
                            ui.label(name);
                            ui.colored_label(colors.accent_btn, fmt_bytes(item.size));
                            ui.label("➔");
                            ui.colored_label(colors.muted, item.target.display().to_string());
                        });
                    }
                });
        }
    }
}
