//! Disk Analyzer page.

use crate::theme::ThemeColors;
use crate::widgets::{BarItem, BarList, Card};
use egui::Ui;
use smart_filee_common::{fmt_bytes, CancellationToken};
use smart_filee_disk_analyzer::{analyze_disk, DiskAnalysisResult};
use smart_filee_localization::tr;
use std::path::PathBuf;

#[derive(Default)]
pub struct DiskPage {
    pub folder: String,
    pub report: Option<DiskAnalysisResult>,
    pub status: String,
}

impl DiskPage {
    pub fn show(&mut self, ui: &mut Ui, colors: &ThemeColors) {
        ui.heading(tr("Disk Analyzer"));
        ui.colored_label(
            colors.muted,
            "Analyze disk space distribution across categories and identify large files",
        );
        ui.add_space(12.0);

        Card::show(ui, colors, |ui| {
            ui.horizontal(|ui| {
                ui.label(tr("Select folder"));
                ui.add(
                    egui::TextEdit::singleline(&mut self.folder)
                        .hint_text("Choose directory to analyze...")
                        .desired_width(ui.available_width() - 200.0),
                );

                if ui.button(tr("Browse...")).clicked() {
                    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                        self.folder = folder.to_string_lossy().to_string();
                    }
                }

                if ui.button(tr("Analyze Disk")).clicked() && !self.folder.is_empty() {
                    let path = PathBuf::from(&self.folder);
                    if path.is_dir() {
                        let cancel = CancellationToken::new();
                        let rep = analyze_disk(&path, 100, &cancel, |_| {});
                        self.status =
                            format!("Analyzed {} files ({})", rep.scanned, fmt_bytes(rep.total));
                        self.report = Some(rep);
                    }
                }
            });
        });

        if !self.status.is_empty() {
            ui.add_space(8.0);
            ui.colored_label(colors.accent_btn, &self.status);
        }

        ui.add_space(12.0);

        if let Some(rep) = &self.report {
            // Category Breakdown using BarList
            ui.heading(tr("Categories"));
            ui.add_space(4.0);

            let max_bytes = rep.categories.values().copied().max().unwrap_or(1).max(1) as f32;

            let mut bar_items: Vec<BarItem> = rep
                .categories
                .iter()
                .map(|(cat, &bytes)| {
                    let fraction = bytes as f32 / max_bytes;
                    BarItem::new(cat.as_str(), fraction, fmt_bytes(bytes))
                })
                .collect();
            bar_items.sort_by(|a, b| {
                b.fraction
                    .partial_cmp(&a.fraction)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            BarList::new(&bar_items, "No categories found").show(ui, colors);

            ui.add_space(16.0);

            // Top Largest Files Table
            ui.heading("Top Largest Files");
            ui.add_space(4.0);

            egui::ScrollArea::vertical()
                .max_height(280.0)
                .show(ui, |ui| {
                    for (idx, f) in rep.top.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(format!("{}.", idx + 1));
                            ui.label(f.path.file_name().unwrap_or_default().to_string_lossy());
                            ui.colored_label(colors.accent_btn, fmt_bytes(f.size));
                            ui.colored_label(colors.muted, f.path.display().to_string());
                        });
                    }
                });
        }
    }
}
