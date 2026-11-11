//! Command palette dialog triggered by Ctrl+K.

use crate::theme::ThemeColors;
use egui::{Align2, Frame, Margin, Rounding, Stroke};

pub struct CommandItem {
    pub title: &'static str,
    pub icon: &'static str,
    pub page_index: usize,
    pub category: &'static str,
}

pub const COMMANDS: &[CommandItem] = &[
    CommandItem {
        title: "Dashboard",
        icon: "⌂",
        page_index: 0,
        category: "Navigation",
    },
    CommandItem {
        title: "File Organizer",
        icon: "▤",
        page_index: 1,
        category: "Navigation",
    },
    CommandItem {
        title: "Disk Analyzer",
        icon: "◉",
        page_index: 2,
        category: "Navigation",
    },
    CommandItem {
        title: "Duplicate Finder",
        icon: "⧉",
        page_index: 3,
        category: "Navigation",
    },
    CommandItem {
        title: "Similar Photos",
        icon: "▣",
        page_index: 4,
        category: "Navigation",
    },
    CommandItem {
        title: "System Cleanup",
        icon: "✧",
        page_index: 5,
        category: "Navigation",
    },
    CommandItem {
        title: "File Search",
        icon: "⌕",
        page_index: 6,
        category: "Navigation",
    },
    CommandItem {
        title: "Bulk Renamer",
        icon: "✎",
        page_index: 7,
        category: "Navigation",
    },
    CommandItem {
        title: "Differential Backup",
        icon: "⟳",
        page_index: 8,
        category: "Navigation",
    },
    CommandItem {
        title: "Active Processes",
        icon: "▦",
        page_index: 9,
        category: "Navigation",
    },
    CommandItem {
        title: "Network Tools",
        icon: "⌁",
        page_index: 10,
        category: "Navigation",
    },
    CommandItem {
        title: "System Info",
        icon: "⚙",
        page_index: 11,
        category: "Navigation",
    },
    CommandItem {
        title: "Activity Log",
        icon: "☷",
        page_index: 12,
        category: "Navigation",
    },
    CommandItem {
        title: "Settings",
        icon: "✦",
        page_index: 13,
        category: "Navigation",
    },
];

pub struct CommandPalette {
    pub is_open: bool,
    pub query: String,
    pub selected_index: usize,
}

impl Default for CommandPalette {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandPalette {
    #[must_use]
    pub fn new() -> Self {
        Self {
            is_open: false,
            query: String::new(),
            selected_index: 0,
        }
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
        if self.is_open {
            self.query.clear();
            self.selected_index = 0;
        }
    }

    pub fn show(
        &mut self,
        ctx: &egui::Context,
        colors: &ThemeColors,
        on_select: &mut impl FnMut(usize),
    ) {
        if !self.is_open {
            return;
        }

        let mut close = false;
        let query = self.query.to_lowercase();
        let filtered: Vec<&CommandItem> = COMMANDS
            .iter()
            .filter(|cmd| {
                if query.is_empty() {
                    true
                } else {
                    cmd.title.to_lowercase().contains(&query)
                        || cmd.category.to_lowercase().contains(&query)
                }
            })
            .collect();

        egui::Window::new("Command Palette")
            .anchor(Align2::CENTER_TOP, egui::vec2(0.0, 100.0))
            .fixed_size(egui::vec2(520.0, 340.0))
            .collapsible(false)
            .resizable(false)
            .title_bar(false)
            .frame(
                Frame::none()
                    .fill(colors.panel)
                    .stroke(Stroke::new(1.5_f32, colors.accent_btn))
                    .rounding(Rounding::same(16.0))
                    .inner_margin(Margin::same(16.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.colored_label(colors.accent_btn, "⌕");
                    let text_edit = ui.add(
                        egui::TextEdit::singleline(&mut self.query)
                            .hint_text("Type a command or jump to page... (Esc to close)")
                            .desired_width(ui.available_width() - 40.0),
                    );
                    text_edit.request_focus();

                    if ui.button("✕").clicked() {
                        close = true;
                    }
                });

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                if filtered.is_empty() {
                    ui.colored_label(colors.muted, "No commands found matching query");
                } else {
                    egui::ScrollArea::vertical()
                        .max_height(250.0)
                        .show(ui, |ui| {
                            for (idx, cmd) in filtered.iter().enumerate() {
                                let is_selected = idx == self.selected_index;
                                let btn_text =
                                    format!("{}  {} ({})", cmd.icon, cmd.title, cmd.category);
                                let response = ui.selectable_label(is_selected, btn_text);
                                if response.clicked() {
                                    on_select(cmd.page_index);
                                    close = true;
                                }
                            }
                        });
                }

                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    close = true;
                }
                if ui.input(|i| i.key_pressed(egui::Key::Enter)) && !filtered.is_empty() {
                    let target_index = self.selected_index.min(filtered.len() - 1);
                    on_select(filtered[target_index].page_index);
                    close = true;
                }
            });

        if close {
            self.is_open = false;
        }
    }
}
