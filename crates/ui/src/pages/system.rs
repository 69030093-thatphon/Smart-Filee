//! System Information page.

use crate::theme::ThemeColors;
use crate::widgets::{Card, StatCard};
use egui::Ui;
use smart_filee_common::fmt_bytes;
use smart_filee_localization::tr;
use smart_filee_system_monitor::SystemSnapshot;

pub struct SystemInfoPage;

impl Default for SystemInfoPage {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemInfoPage {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    pub fn show(&mut self, ui: &mut Ui, colors: &ThemeColors, sys: Option<&SystemSnapshot>) {
        ui.heading(tr("System Info"));
        ui.colored_label(
            colors.muted,
            "Hardware specifications, memory distribution, and storage partition details",
        );
        ui.add_space(12.0);

        if let Some(s) = sys {
            // Stat row
            ui.horizontal(|ui| {
                StatCard::new("Operating System", &s.os_name)
                    .subtitle(&s.kernel_version)
                    .show(ui, colors);

                StatCard::new("CPU Architecture", &s.arch)
                    .subtitle(&format!("{} Cores", s.logical_cores))
                    .show(ui, colors);

                StatCard::new("Total RAM", &fmt_bytes(s.memory_total_bytes))
                    .subtitle(&format!("Used: {}", fmt_bytes(s.memory_used_bytes)))
                    .show(ui, colors);
            });

            ui.add_space(16.0);

            // Disk Partitions Table
            Card::show(ui, colors, |ui| {
                ui.heading("Storage Partitions & Drives");
                ui.add_space(8.0);

                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.set_min_width(ui.available_width());
                            ui.label("Name");
                            ui.add_space(60.0);
                            ui.label("Mount Point");
                            ui.add_space(80.0);
                            ui.label("Total");
                            ui.add_space(60.0);
                            ui.label("Free");
                        });
                        ui.separator();

                        for disk in &s.disks {
                            ui.horizontal(|ui| {
                                ui.label(&disk.name);
                                ui.add_space(40.0);
                                ui.colored_label(colors.muted, &disk.mount_point);
                                ui.add_space(40.0);
                                ui.label(fmt_bytes(disk.total_bytes));
                                ui.add_space(40.0);
                                ui.colored_label(colors.green, fmt_bytes(disk.free_bytes));
                            });
                        }
                    });
            });
        } else {
            ui.colored_label(colors.muted, "Collecting system specifications...");
        }
    }
}
