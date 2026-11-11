//! Activity Log page.

use crate::theme::ThemeColors;
use crate::widgets::Card;
use egui::Ui;
use smart_filee_database::Database;
use smart_filee_localization::tr;

#[derive(Default)]
pub struct ActivityLogPage {
    pub search_query: String,
    pub logs: Vec<String>,
    pub status: String,
}

impl ActivityLogPage {
    pub fn refresh(&mut self, db: &Database) {
        let q = if self.search_query.trim().is_empty() {
            None
        } else {
            Some(self.search_query.trim())
        };
        self.logs = db.recent_logs(500, q);
    }

    pub fn show(&mut self, ui: &mut Ui, colors: &ThemeColors, db: Option<&mut Database>) {
        ui.heading(tr("Activity Log"));
        ui.colored_label(
            colors.muted,
            "Audit trail of file operations, scans, and system maintenance events",
        );
        ui.add_space(12.0);

        Card::show(ui, colors, |ui| {
            ui.horizontal(|ui| {
                ui.label("Search:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.search_query)
                        .hint_text("Search log messages...")
                        .desired_width(240.0),
                );

                if let Some(d) = db.as_deref() {
                    if ui.button("Filter / Refresh").clicked() {
                        self.refresh(d);
                    }
                }

                if ui.button("Clear Logs").clicked() {
                    if let Some(d) = db {
                        d.clear_logs();
                        self.logs.clear();
                        self.status = "Cleared all logs.".to_string();
                    }
                }
            });
        });

        ui.add_space(12.0);

        ui.heading(format!("Events ({})", self.logs.len()));
        ui.add_space(4.0);

        egui::ScrollArea::vertical()
            .max_height(400.0)
            .show(ui, |ui| {
                for log in &self.logs {
                    ui.horizontal(|ui| {
                        ui.colored_label(colors.accent_btn, "•");
                        ui.label(log);
                    });
                }
            });
    }
}
