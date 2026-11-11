//! File Search page.

use crate::theme::ThemeColors;
use crate::widgets::Card;
use egui::Ui;
use smart_filee_common::{fmt_bytes, CancellationToken};
use smart_filee_localization::tr;
use smart_filee_search::{search_files, SearchResultItem};
use std::path::PathBuf;

#[derive(Default)]
pub struct SearchPage {
    pub folder: String,
    pub query: String,
    pub min_size_kb: u64,
    pub results: Vec<SearchResultItem>,
    pub status: String,
}

impl SearchPage {
    pub fn show(&mut self, ui: &mut Ui, colors: &ThemeColors) {
        ui.heading(tr("File Search"));
        ui.colored_label(
            colors.muted,
            "Blazing fast directory search with wildcard matching and size filters",
        );
        ui.add_space(12.0);

        Card::show(ui, colors, |ui| {
            ui.horizontal(|ui| {
                ui.label(tr("Select folder"));
                ui.add(
                    egui::TextEdit::singleline(&mut self.folder)
                        .hint_text("Choose directory to search...")
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
                ui.label("Query pattern:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.query)
                        .hint_text("e.g. *.pdf, budget*, invoice")
                        .desired_width(260.0),
                );

                if ui.button(tr("Search")).clicked() && !self.folder.is_empty() {
                    let path = PathBuf::from(&self.folder);
                    if path.is_dir() {
                        let cancel = CancellationToken::new();
                        let min_bytes = self.min_size_kb * 1024;
                        let res =
                            search_files(&path, &self.query, min_bytes, 500, &cancel, |_, _| {});
                        self.status = format!("Found {} matching files", res.results.len());
                        self.results = res.results;
                    }
                }
            });
        });

        if !self.status.is_empty() {
            ui.add_space(8.0);
            ui.colored_label(colors.accent_btn, &self.status);
        }

        ui.add_space(12.0);

        if !self.results.is_empty() {
            ui.heading("Search Results");
            ui.add_space(4.0);

            egui::ScrollArea::vertical()
                .max_height(350.0)
                .show(ui, |ui| {
                    for (idx, item) in self.results.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(format!("{}.", idx + 1));
                            ui.label(item.path.file_name().unwrap_or_default().to_string_lossy());
                            ui.colored_label(colors.accent_btn, fmt_bytes(item.size));
                            ui.colored_label(colors.muted, item.path.display().to_string());
                        });
                    }
                });
        }
    }
}
