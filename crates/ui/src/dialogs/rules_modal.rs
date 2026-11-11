//! Custom rules editor modal dialog.

use crate::theme::ThemeColors;
use egui::{Align2, Frame, Margin, Rounding, Stroke};
use smart_filee_localization::tr;

#[derive(Debug, Clone)]
pub struct RuleEntry {
    pub pattern: String,
    pub folder: String,
}

pub struct RulesModal {
    pub is_open: bool,
    pub rules: Vec<RuleEntry>,
}

impl Default for RulesModal {
    fn default() -> Self {
        Self::new()
    }
}

impl RulesModal {
    #[must_use]
    pub fn new() -> Self {
        Self {
            is_open: false,
            rules: Vec::new(),
        }
    }

    pub fn open_with(&mut self, rules: Vec<(String, String)>) {
        self.rules = rules
            .into_iter()
            .map(|(pattern, folder)| RuleEntry { pattern, folder })
            .collect();
        self.is_open = true;
    }

    pub fn show(
        &mut self,
        ctx: &egui::Context,
        colors: &ThemeColors,
        on_save: &mut impl FnMut(Vec<(String, String)>),
    ) {
        if !self.is_open {
            return;
        }

        let mut close = false;
        let mut save = false;

        egui::Window::new("Custom Rules")
            .anchor(Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .fixed_size(egui::vec2(580.0, 440.0))
            .collapsible(false)
            .resizable(false)
            .frame(
                Frame::none()
                    .fill(colors.panel)
                    .stroke(Stroke::new(1.0_f32, colors.border))
                    .rounding(Rounding::same(16.0))
                    .inner_margin(Margin::same(20.0)),
            )
            .show(ctx, |ui| {
                ui.heading(tr("Custom rules"));
                ui.add_space(4.0);
                ui.colored_label(
                    colors.muted,
                    "Files whose name matches a pattern go to that folder. Use wildcards (*.psd, report_*) or part of a name. The first matching rule wins.",
                );
                ui.add_space(12.0);

                let mut remove_idx = None;

                egui::ScrollArea::vertical().max_height(260.0).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.set_min_width(ui.available_width());
                        ui.label(tr("Pattern"));
                        ui.add_space(140.0);
                        ui.label(tr("Folder"));
                    });
                    ui.separator();

                    for (idx, rule) in self.rules.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::TextEdit::singleline(&mut rule.pattern)
                                    .hint_text("*.pdf or report_*")
                                    .desired_width(180.0),
                            );
                            ui.label("➔");
                            ui.add(
                                egui::TextEdit::singleline(&mut rule.folder)
                                    .hint_text("Documents/Reports")
                                    .desired_width(180.0),
                            );
                            if ui.button("✕").clicked() {
                                remove_idx = Some(idx);
                            }
                        });
                    }
                });

                if let Some(idx) = remove_idx {
                    if idx < self.rules.len() {
                        self.rules.remove(idx);
                    }
                }

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    if ui.button("+ Add Rule").clicked() {
                        self.rules.push(RuleEntry {
                            pattern: String::new(),
                            folder: String::new(),
                        });
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(tr("Save")).clicked() {
                            save = true;
                            close = true;
                        }
                        if ui.button(tr("Cancel")).clicked() {
                            close = true;
                        }
                    });
                });
            });

        if save {
            let pairs = self
                .rules
                .iter()
                .filter(|r| !r.pattern.trim().is_empty() && !r.folder.trim().is_empty())
                .map(|r| (r.pattern.trim().to_string(), r.folder.trim().to_string()))
                .collect();
            on_save(pairs);
        }

        if close {
            self.is_open = false;
        }
    }
}
