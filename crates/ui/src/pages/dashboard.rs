//! Dashboard page: Tidy score ring, system overview, storage stats, quick actions.

use crate::theme::ThemeColors;
use crate::widgets::{Card, Ring};
use egui::Ui;
use smart_filee_common::{fmt_bytes, tidy_score};
use smart_filee_localization::tr;
use smart_filee_system_monitor::SystemSnapshot;
use std::path::PathBuf;

#[derive(Default)]
pub struct DashboardPage {
    pub selected_folder: String,
    pub tidy_score_val: u32,
    pub loose_count: u64,
    pub dup_count: u64,
    pub temp_count: u64,
    pub empty_dirs: u64,
    pub is_analyzing: bool,
}

impl DashboardPage {
    pub fn show(
        &mut self,
        ui: &mut Ui,
        colors: &ThemeColors,
        sys: Option<&SystemSnapshot>,
        on_navigate: &mut impl FnMut(usize),
    ) {
        ui.heading(tr("Dashboard"));
        ui.colored_label(
            colors.muted,
            "Overview of your system health, storage clutter, and quick actions",
        );
        ui.add_space(12.0);

        // 1. Directory selector bar
        Card::show(ui, colors, |ui| {
            ui.horizontal(|ui| {
                ui.label(tr("Select folder"));
                ui.add(
                    egui::TextEdit::singleline(&mut self.selected_folder)
                        .hint_text("Choose a folder to scan and calculate tidy score...")
                        .desired_width(ui.available_width() - 200.0),
                );

                if ui.button(tr("Browse...")).clicked() {
                    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                        self.selected_folder = folder.to_string_lossy().to_string();
                    }
                }

                if ui.button(tr("Analyze")).clicked() && !self.selected_folder.is_empty() {
                    let path = PathBuf::from(&self.selected_folder);
                    if path.is_dir() {
                        // Quick scan for metrics
                        if let Ok(entries) = std::fs::read_dir(&path) {
                            let mut loose = 0;
                            for e in entries.flatten() {
                                if e.path().is_file() {
                                    loose += 1;
                                }
                            }
                            self.loose_count = loose;
                            self.dup_count = 0;
                            self.temp_count = 0;
                            self.empty_dirs = 0;

                            self.tidy_score_val = tidy_score(
                                self.loose_count,
                                self.dup_count,
                                0,
                                0,
                                0,
                                self.empty_dirs,
                            );
                        }
                        self.is_analyzing = false;
                    }
                }
            });
        });

        ui.add_space(12.0);

        // 2. Metrics Row (Ring + System Stats)
        ui.horizontal(|ui| {
            // Tidy score card
            Card::show(ui, colors, |ui| {
                ui.set_width(240.0);
                ui.vertical_centered(|ui| {
                    ui.label(tr("Tidy score"));
                    ui.add_space(8.0);
                    let score_str = if self.tidy_score_val > 0 || !self.selected_folder.is_empty() {
                        format!("{}", self.tidy_score_val)
                    } else {
                        "—".to_string()
                    };
                    Ring::new(self.tidy_score_val as f32, &score_str)
                        .diameter(120.0)
                        .thickness(12.0)
                        .show(ui, colors);

                    ui.add_space(8.0);
                    let rating = match self.tidy_score_val {
                        90..=100 => "Spotless",
                        75..=89 => "Well kept",
                        50..=74 => "Needs tidy up",
                        _ => "Chaotic!",
                    };
                    ui.colored_label(colors.accent_btn, rating);
                });
            });

            ui.add_space(8.0);

            // Hardware & System overview
            Card::show(ui, colors, |ui| {
                ui.set_width(ui.available_width());
                ui.heading(tr("System Overview"));
                ui.add_space(6.0);

                if let Some(s) = sys {
                    ui.horizontal(|ui| {
                        ui.label(format!("CPU: {:.1}%", s.cpu_percent));
                        ui.separator();
                        ui.label(format!(
                            "RAM: {} / {}",
                            fmt_bytes(s.memory_used_bytes),
                            fmt_bytes(s.memory_total_bytes)
                        ));
                        ui.separator();
                        ui.label(format!("OS: {}", s.os_name));
                    });

                    ui.add_space(10.0);
                    ui.colored_label(
                        colors.muted,
                        format!(
                            "CPU: {} | Total Logical Cores: {}",
                            s.cpu_brand, s.logical_cores
                        ),
                    );
                } else {
                    ui.colored_label(colors.muted, "System metrics updating...");
                }
            });
        });

        ui.add_space(16.0);

        // 3. Quick Actions Cards
        ui.heading("Quick Actions");
        ui.add_space(6.0);

        ui.horizontal(|ui| {
            if ui.button("▤  Organize Files").clicked() {
                on_navigate(1);
            }
            if ui.button("◉  Disk Space").clicked() {
                on_navigate(2);
            }
            if ui.button("⧉  Find Duplicates").clicked() {
                on_navigate(3);
            }
            if ui.button("✧  System Cleanup").clicked() {
                on_navigate(5);
            }
            if ui.button("⟳  Differential Backup").clicked() {
                on_navigate(8);
            }
        });
    }
}
