//! Read-only active processes page.

use crate::theme::ThemeColors;
use crate::widgets::Card;
use egui::Ui;
use smart_filee_common::fmt_bytes;
use smart_filee_localization::tr;
use smart_filee_system_monitor::{ProcessItem, ProcessSort, SystemMonitor};

pub struct ProcessesPage {
    pub search_query: String,
    pub sort_by_memory: bool,
    pub processes: Vec<ProcessItem>,
    pub status: String,
}

impl Default for ProcessesPage {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            sort_by_memory: true,
            processes: Vec::new(),
            status: String::new(),
        }
    }
}

impl ProcessesPage {
    pub fn refresh(&mut self, monitor: &mut SystemMonitor) {
        let sort = if self.sort_by_memory {
            ProcessSort::Memory
        } else {
            ProcessSort::Cpu
        };
        let q = if self.search_query.trim().is_empty() {
            None
        } else {
            Some(self.search_query.trim())
        };
        self.processes = monitor.list_processes(q, sort);
        self.status = format!("Loaded {} active processes", self.processes.len());
    }

    pub fn show(&mut self, ui: &mut Ui, colors: &ThemeColors, monitor: &mut SystemMonitor) {
        ui.heading(tr("Processes"));
        ui.colored_label(
            colors.muted,
            "Read-only inspection of top system processes sorted by CPU and memory consumption",
        );
        ui.add_space(12.0);

        Card::show(ui, colors, |ui| {
            ui.horizontal(|ui| {
                ui.label("Search:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.search_query)
                        .hint_text("Filter by process name or PID...")
                        .desired_width(240.0),
                );

                if ui.button(tr("Refresh")).clicked() {
                    self.refresh(monitor);
                }

                ui.separator();
                if ui
                    .selectable_label(self.sort_by_memory, "Sort by Memory")
                    .clicked()
                {
                    self.sort_by_memory = true;
                    self.refresh(monitor);
                }
                if ui
                    .selectable_label(!self.sort_by_memory, "Sort by CPU")
                    .clicked()
                {
                    self.sort_by_memory = false;
                    self.refresh(monitor);
                }
            });
        });

        if self.processes.is_empty() {
            self.refresh(monitor);
        }

        ui.add_space(12.0);

        ui.heading(format!("Active Processes ({})", self.processes.len()));
        ui.add_space(4.0);

        egui::ScrollArea::vertical()
            .max_height(400.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.set_min_width(ui.available_width());
                    ui.label("PID");
                    ui.add_space(40.0);
                    ui.label("Name");
                    ui.add_space(180.0);
                    ui.label("CPU %");
                    ui.add_space(40.0);
                    ui.label("Memory");
                });
                ui.separator();

                for p in &self.processes {
                    ui.horizontal(|ui| {
                        ui.label(format!("{}", p.pid));
                        ui.add_space(30.0);
                        ui.label(&p.name);
                        ui.add_space(20.0);
                        ui.colored_label(colors.accent_btn, format!("{:.1}%", p.cpu_percent));
                        ui.add_space(20.0);
                        ui.colored_label(colors.muted, fmt_bytes(p.memory_rss_bytes));
                    });
                }
            });
    }
}
