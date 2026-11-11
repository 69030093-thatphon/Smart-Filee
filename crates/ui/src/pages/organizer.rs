//! File Organizer page.

use crate::theme::ThemeColors;
use crate::widgets::Card;
use egui::Ui;
use smart_filee_common::CancellationToken;
use smart_filee_database::Database;
use smart_filee_file_engine::{move_files, undo_moves};
use smart_filee_localization::tr;
use smart_filee_organizer::{plan_organize, OrganizeMode, OrganizePlanItem};
use std::path::PathBuf;

pub struct OrganizerPage {
    pub folder: String,
    pub mode: OrganizeMode,
    pub planned_moves: Vec<OrganizePlanItem>,
    pub status: String,
    pub is_busy: bool,
}

impl Default for OrganizerPage {
    fn default() -> Self {
        Self {
            folder: String::new(),
            mode: OrganizeMode::ByType,
            planned_moves: Vec::new(),
            status: String::new(),
            is_busy: false,
        }
    }
}

impl OrganizerPage {
    pub fn show(
        &mut self,
        ui: &mut Ui,
        colors: &ThemeColors,
        db: Option<&mut Database>,
        on_open_rules: &mut impl FnMut(),
    ) {
        ui.heading(tr("File Organizer"));
        ui.colored_label(
            colors.muted,
            "Sort files cleanly by file category, month, or photo EXIF date",
        );
        ui.add_space(12.0);

        Card::show(ui, colors, |ui| {
            ui.horizontal(|ui| {
                ui.label(tr("Select folder"));
                ui.add(
                    egui::TextEdit::singleline(&mut self.folder)
                        .hint_text("Path to folder to organize...")
                        .desired_width(ui.available_width() - 280.0),
                );

                if ui.button(tr("Browse...")).clicked() {
                    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                        self.folder = folder.to_string_lossy().to_string();
                    }
                }

                if ui.button(tr("Custom rules")).clicked() {
                    on_open_rules();
                }
            });

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label(tr("Mode:"));
                ui.selectable_value(&mut self.mode, OrganizeMode::ByType, tr("By type"));
                ui.selectable_value(&mut self.mode, OrganizeMode::ByMonth, tr("By month"));
                ui.selectable_value(
                    &mut self.mode,
                    OrganizeMode::ByTypeAndMonth,
                    tr("By type + month"),
                );
                ui.selectable_value(
                    &mut self.mode,
                    OrganizeMode::PhotosByDateTaken,
                    tr("Photos by date taken"),
                );
            });

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button(tr("Preview / Plan")).clicked() && !self.folder.is_empty() {
                    let root = PathBuf::from(&self.folder);
                    if root.is_dir() {
                        let rules = db.as_ref().map(|d| d.get_rules()).unwrap_or_default();
                        let parsed_rules = smart_filee_organizer::parse_rules(&rules);

                        match plan_organize(&root, self.mode, &parsed_rules) {
                            Ok(plan) => {
                                self.status = format!("Found {} files to organize", plan.len());
                                self.planned_moves = plan;
                            }
                            Err(e) => {
                                self.status = format!("Planning error: {e}");
                            }
                        }
                    }
                }

                let can_organize = !self.planned_moves.is_empty() && !self.is_busy;
                if ui
                    .add_enabled(can_organize, egui::Button::new(tr("Organize Now")))
                    .clicked()
                {
                    let cancel = CancellationToken::new();
                    let pairs: Vec<(PathBuf, PathBuf)> = self
                        .planned_moves
                        .iter()
                        .map(|m| {
                            let file_name = m.src.file_name().unwrap_or_default();
                            (m.src.clone(), m.dest_dir.join(file_name))
                        })
                        .collect();

                    let res = move_files(&pairs, &cancel, |_, _| {});
                    if !res.moves.is_empty() {
                        self.status = format!("Organized {} files successfully!", res.moves.len());
                        if let Some(d) = db.as_deref() {
                            let _ = d.push_undo("Organize Files", &res.moves);
                        }
                        self.planned_moves.clear();
                    } else if !res.errors.is_empty() {
                        self.status = format!("Errors occurred during move: {:?}", res.errors);
                    }
                }

                if ui.button(tr("Undo last batch")).clicked() {
                    if let Some(d) = db {
                        let stack = d.load_undo_stack(1);
                        if let Some((_label, moves)) = stack.into_iter().next() {
                            let cancel = CancellationToken::new();
                            let dirs = [
                                "Images",
                                "Documents",
                                "Archives",
                                "Installers",
                                "Audio",
                                "Videos",
                                "Code",
                                "Others",
                            ];
                            let res = undo_moves(&moves, &dirs, &cancel, |_, _| {});
                            d.pop_undo();
                            self.status = format!("Undid {} moves successfully!", res.restored);
                        } else {
                            self.status = "No undo batches found in history.".to_string();
                        }
                    }
                }
            });
        });

        if !self.status.is_empty() {
            ui.add_space(8.0);
            ui.colored_label(colors.accent_btn, &self.status);
        }

        ui.add_space(12.0);

        // Preview Table
        if !self.planned_moves.is_empty() {
            ui.heading("Planned Moves Preview");
            ui.add_space(4.0);

            egui::ScrollArea::vertical()
                .max_height(350.0)
                .show(ui, |ui| {
                    for m in &self.planned_moves {
                        ui.horizontal(|ui| {
                            let filename = m.src.file_name().unwrap_or_default().to_string_lossy();
                            ui.label(filename);
                            ui.label("➔");
                            ui.colored_label(colors.muted, m.dest_dir.display().to_string());
                        });
                    }
                });
        }
    }
}
