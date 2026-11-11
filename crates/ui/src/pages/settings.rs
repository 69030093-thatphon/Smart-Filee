//! Settings page.

use crate::theme::{Theme, ThemeColors};
use crate::widgets::Card;
use egui::Ui;
use smart_filee_database::Database;
use smart_filee_localization::{current_language, set_language, tr, Language};

pub struct SettingsPage {
    pub selected_theme: Theme,
    pub status: String,
}

impl Default for SettingsPage {
    fn default() -> Self {
        Self {
            selected_theme: Theme::PremiumLightViolet,
            status: String::new(),
        }
    }
}

impl SettingsPage {
    pub fn show(
        &mut self,
        ui: &mut Ui,
        colors: &ThemeColors,
        db: Option<&mut Database>,
        on_theme_change: &mut impl FnMut(Theme),
        on_open_auth: &mut impl FnMut(),
    ) {
        ui.heading(tr("Settings"));
        ui.colored_label(
            colors.muted,
            "Personalize visual theme, choose interface language, and manage database maintenance",
        );
        ui.add_space(12.0);

        // Theme and Appearance
        Card::show(ui, colors, |ui| {
            ui.heading("Theme & Appearance");
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label("Active Theme:");
                for theme in Theme::ALL {
                    if ui
                        .selectable_label(self.selected_theme == theme, theme.name())
                        .clicked()
                    {
                        self.selected_theme = theme;
                        on_theme_change(theme);
                    }
                }
            });
        });

        ui.add_space(12.0);

        // Language & Localization
        Card::show(ui, colors, |ui| {
            ui.heading("Language & Localization");
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label("Display Language:");
                let cur = current_language();

                if ui
                    .selectable_label(cur == Language::English, "English")
                    .clicked()
                {
                    set_language(Language::English);
                }
                if ui
                    .selectable_label(cur == Language::Thai, "ภาษาไทย (Thai)")
                    .clicked()
                {
                    set_language(Language::Thai);
                }
            });
        });

        ui.add_space(12.0);

        // Database & Maintenance
        Card::show(ui, colors, |ui| {
            ui.heading("Database & Storage Maintenance");
            ui.add_space(8.0);

            if let Some(d) = db {
                ui.label("SQLite local database is active (WAL mode).");
                ui.add_space(6.0);

                if ui
                    .button("Run Maintenance (Prune Cache & Trim Logs)")
                    .clicked()
                {
                    d.maintenance();
                    self.status = "Database maintenance executed successfully!".to_string();
                }
            } else {
                ui.colored_label(colors.muted, "Database is running in memory mode.");
            }

            if !self.status.is_empty() {
                ui.add_space(6.0);
                ui.colored_label(colors.green, &self.status);
            }
        });

        ui.add_space(12.0);

        // Supabase Cloud Account
        Card::show(ui, colors, |ui| {
            ui.heading("Supabase Cloud Account");
            ui.add_space(8.0);
            ui.colored_label(
                colors.muted,
                "Sign in to synchronize preferences and manage connected device licenses.",
            );
            ui.add_space(6.0);

            if ui.button("Manage Account / Sign In").clicked() {
                on_open_auth();
            }
        });

        ui.add_space(12.0);

        // About Smart-Filee
        Card::show(ui, colors, |ui| {
            ui.heading("About Smart-Filee (Rust Edition)");
            ui.add_space(4.0);
            ui.label("Version: 4.3.1 (High-Performance Native Rust)");
            ui.colored_label(
                colors.muted,
                "Zero telemetry, local-first safety guards, and privacy-respecting file management.",
            );
        });
    }
}
