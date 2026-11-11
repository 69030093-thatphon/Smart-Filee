//! Central application shell for Smart-Filee.

use crate::dialogs::{AuthModal, CommandPalette, RulesModal};
use crate::pages::{
    ActivityLogPage, BackupPage, CleanupPage, DashboardPage, DiskPage, DuplicatesPage, NetworkPage,
    OrganizerPage, ProcessesPage, RenamerPage, SearchPage, SettingsPage, SimilarPhotosPage,
    SystemInfoPage,
};
use crate::theme::Theme;
use crate::widgets::ToastManager;
use egui::{vec2, Align, Color32, Frame, Margin, Rounding, Stroke};
use smart_filee_auth::{SupabaseClient, SupabaseConfig};
use smart_filee_database::Database;
use smart_filee_system_monitor::{SystemMonitor, SystemSnapshot};
use std::path::PathBuf;
use std::time::{Duration, Instant};

pub struct SmartFileeApp {
    pub theme: Theme,
    pub current_page: usize,
    pub db: Option<Database>,
    pub auth_client: Option<SupabaseClient>,
    pub system_monitor: SystemMonitor,
    pub last_system_snapshot: Option<SystemSnapshot>,
    pub last_snapshot_time: Instant,
    pub toast_manager: ToastManager,

    // Dialogs
    pub command_palette: CommandPalette,
    pub rules_modal: RulesModal,
    pub auth_modal: AuthModal,

    // Pages
    pub dashboard_page: DashboardPage,
    pub organizer_page: OrganizerPage,
    pub disk_page: DiskPage,
    pub dups_page: DuplicatesPage,
    pub similar_page: SimilarPhotosPage,
    pub cleanup_page: CleanupPage,
    pub search_page: SearchPage,
    pub renamer_page: RenamerPage,
    pub backup_page: BackupPage,
    pub procs_page: ProcessesPage,
    pub network_page: NetworkPage,
    pub system_page: SystemInfoPage,
    pub log_page: ActivityLogPage,
    pub settings_page: SettingsPage,
}

pub struct NavItem {
    pub icon: &'static str,
    pub title: &'static str,
    pub shortcut: &'static str,
}

pub const NAV_ITEMS: [NavItem; 14] = [
    NavItem {
        icon: "⌂",
        title: "Dashboard",
        shortcut: "Ctrl+1",
    },
    NavItem {
        icon: "▤",
        title: "File Organizer",
        shortcut: "Ctrl+2",
    },
    NavItem {
        icon: "◉",
        title: "Disk Analyzer",
        shortcut: "Ctrl+3",
    },
    NavItem {
        icon: "⧉",
        title: "Duplicate Finder",
        shortcut: "Ctrl+4",
    },
    NavItem {
        icon: "▣",
        title: "Similar Photos",
        shortcut: "Ctrl+5",
    },
    NavItem {
        icon: "✧",
        title: "Cleanup",
        shortcut: "Ctrl+6",
    },
    NavItem {
        icon: "⌕",
        title: "File Search",
        shortcut: "Ctrl+7",
    },
    NavItem {
        icon: "✎",
        title: "Bulk Renamer",
        shortcut: "Ctrl+8",
    },
    NavItem {
        icon: "⟳",
        title: "Backup",
        shortcut: "Ctrl+9",
    },
    NavItem {
        icon: "▦",
        title: "Processes",
        shortcut: "Ctrl+Shift+1",
    },
    NavItem {
        icon: "⌁",
        title: "Network Tools",
        shortcut: "Ctrl+Shift+2",
    },
    NavItem {
        icon: "⚙",
        title: "System Info",
        shortcut: "Ctrl+Shift+3",
    },
    NavItem {
        icon: "☷",
        title: "Activity Log",
        shortcut: "Ctrl+Shift+4",
    },
    NavItem {
        icon: "✦",
        title: "Settings",
        shortcut: "Ctrl+Shift+5",
    },
];

impl Default for SmartFileeApp {
    fn default() -> Self {
        Self::new()
    }
}

impl SmartFileeApp {
    #[must_use]
    pub fn new() -> Self {
        let mut sys = SystemMonitor::new();
        let snap = sys.snapshot();

        let db_path = PathBuf::from("sfm.db");
        let db = Database::open(&db_path)
            .or_else(|_| Database::open(":memory:"))
            .ok();
        let auth_client = SupabaseConfig::from_env().ok().map(SupabaseClient::new);

        Self {
            theme: Theme::PremiumLightViolet,
            current_page: 0,
            db,
            auth_client,
            system_monitor: sys,
            last_system_snapshot: Some(snap),
            last_snapshot_time: Instant::now(),
            toast_manager: ToastManager::new(),

            command_palette: CommandPalette::new(),
            rules_modal: RulesModal::new(),
            auth_modal: AuthModal::new(),

            dashboard_page: DashboardPage::default(),
            organizer_page: OrganizerPage::default(),
            disk_page: DiskPage::default(),
            dups_page: DuplicatesPage::default(),
            similar_page: SimilarPhotosPage::default(),
            cleanup_page: CleanupPage::default(),
            search_page: SearchPage::default(),
            renamer_page: RenamerPage::default(),
            backup_page: BackupPage::default(),
            procs_page: ProcessesPage::default(),
            network_page: NetworkPage::default(),
            system_page: SystemInfoPage::new(),
            log_page: ActivityLogPage::default(),
            settings_page: SettingsPage::default(),
        }
    }

    fn check_shortcuts(&mut self, ctx: &egui::Context) {
        // Ctrl+K -> Command palette
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::K)) {
            self.command_palette.toggle();
        }

        // Ctrl+1 through Ctrl+9
        let ctrl = ctx.input(|i| i.modifiers.command && !i.modifiers.shift);
        if ctrl {
            if ctx.input(|i| i.key_pressed(egui::Key::Num1)) {
                self.current_page = 0;
            }
            if ctx.input(|i| i.key_pressed(egui::Key::Num2)) {
                self.current_page = 1;
            }
            if ctx.input(|i| i.key_pressed(egui::Key::Num3)) {
                self.current_page = 2;
            }
            if ctx.input(|i| i.key_pressed(egui::Key::Num4)) {
                self.current_page = 3;
            }
            if ctx.input(|i| i.key_pressed(egui::Key::Num5)) {
                self.current_page = 4;
            }
            if ctx.input(|i| i.key_pressed(egui::Key::Num6)) {
                self.current_page = 5;
            }
            if ctx.input(|i| i.key_pressed(egui::Key::Num7)) {
                self.current_page = 6;
            }
            if ctx.input(|i| i.key_pressed(egui::Key::Num8)) {
                self.current_page = 7;
            }
            if ctx.input(|i| i.key_pressed(egui::Key::Num9)) {
                self.current_page = 8;
            }
        }

        // Ctrl+Shift+1 through Ctrl+Shift+5
        let ctrl_shift = ctx.input(|i| i.modifiers.command && i.modifiers.shift);
        if ctrl_shift {
            if ctx.input(|i| i.key_pressed(egui::Key::Num1)) {
                self.current_page = 9;
            }
            if ctx.input(|i| i.key_pressed(egui::Key::Num2)) {
                self.current_page = 10;
            }
            if ctx.input(|i| i.key_pressed(egui::Key::Num3)) {
                self.current_page = 11;
            }
            if ctx.input(|i| i.key_pressed(egui::Key::Num4)) {
                self.current_page = 12;
            }
            if ctx.input(|i| i.key_pressed(egui::Key::Num5)) {
                self.current_page = 13;
            }
        }
    }
}

impl eframe::App for SmartFileeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.theme.apply(ctx);
        let colors = self.theme.colors();

        self.check_shortcuts(ctx);

        // Refresh hardware metrics every 3 seconds
        if self.last_snapshot_time.elapsed() > Duration::from_secs(3) {
            self.last_system_snapshot = Some(self.system_monitor.snapshot());
            self.last_snapshot_time = Instant::now();
        }

        // 1. Sidebar Navigation Rail
        egui::SidePanel::left("sidebar_rail")
            .resizable(false)
            .exact_width(64.0)
            .frame(
                Frame::none()
                    .fill(colors.sidebar)
                    .stroke(Stroke::new(1.0_f32, colors.border))
                    .inner_margin(Margin::symmetric(8.0, 12.0)),
            )
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(4.0);
                    // Brand mark
                    ui.colored_label(colors.accent_btn, "SF");
                    ui.add_space(16.0);

                    // Nav Buttons
                    for (idx, item) in NAV_ITEMS.iter().enumerate() {
                        let is_active = self.current_page == idx;
                        let btn_color = if is_active {
                            colors.navsel
                        } else {
                            Color32::TRANSPARENT
                        };

                        let btn = egui::Button::new(item.icon)
                            .fill(btn_color)
                            .stroke(if is_active {
                                Stroke::new(1.0_f32, colors.accent_btn)
                            } else {
                                Stroke::NONE
                            })
                            .rounding(Rounding::same(10.0))
                            .min_size(vec2(44.0, 44.0));

                        let resp = ui
                            .add(btn)
                            .on_hover_text(format!("{} ({})", item.title, item.shortcut));
                        if resp.clicked() {
                            self.current_page = idx;
                        }

                        ui.add_space(4.0);
                    }
                });
            });

        // 2. Top Header Bar
        egui::TopBottomPanel::top("top_header")
            .exact_height(50.0)
            .frame(
                Frame::none()
                    .fill(colors.sidebar)
                    .stroke(Stroke::new(1.0_f32, colors.border))
                    .inner_margin(Margin::symmetric(20.0, 10.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let page_title = NAV_ITEMS[self.current_page].title;
                    ui.heading(page_title);

                    ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                        // Account button
                        if ui.button("👤  Account").clicked() {
                            self.auth_modal.open();
                        }

                        // Command palette trigger
                        if ui
                            .button("⌕  Ctrl+K")
                            .on_hover_text("Quick Command Palette")
                            .clicked()
                        {
                            self.command_palette.toggle();
                        }
                    });
                });
            });

        // 3. Central Content Area
        egui::CentralPanel::default()
            .frame(
                Frame::none()
                    .fill(colors.bg)
                    .inner_margin(Margin::same(24.0)),
            )
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let mut navigate_to = None;
                    let mut open_rules = false;

                    match self.current_page {
                        0 => {
                            self.dashboard_page.show(
                                ui,
                                &colors,
                                self.last_system_snapshot.as_ref(),
                                &mut |idx| navigate_to = Some(idx),
                            );
                        }
                        1 => {
                            self.organizer_page
                                .show(ui, &colors, self.db.as_mut(), &mut || open_rules = true);
                        }
                        2 => {
                            self.disk_page.show(ui, &colors);
                        }
                        3 => {
                            self.dups_page.show(ui, &colors);
                        }
                        4 => {
                            self.similar_page.show(ui, &colors);
                        }
                        5 => {
                            self.cleanup_page.show(ui, &colors);
                        }
                        6 => {
                            self.search_page.show(ui, &colors);
                        }
                        7 => {
                            self.renamer_page.show(ui, &colors);
                        }
                        8 => {
                            self.backup_page.show(ui, &colors);
                        }
                        9 => {
                            self.procs_page.show(ui, &colors, &mut self.system_monitor);
                        }
                        10 => {
                            self.network_page.show(ui, &colors);
                        }
                        11 => {
                            self.system_page
                                .show(ui, &colors, self.last_system_snapshot.as_ref());
                        }
                        12 => {
                            self.log_page.show(ui, &colors, self.db.as_mut());
                        }
                        13 => {
                            let mut change_theme = None;
                            let mut open_auth = false;

                            self.settings_page.show(
                                ui,
                                &colors,
                                self.db.as_mut(),
                                &mut |t| change_theme = Some(t),
                                &mut || open_auth = true,
                            );

                            if let Some(t) = change_theme {
                                self.theme = t;
                            }
                            if open_auth {
                                self.auth_modal.open();
                            }
                        }
                        _ => {}
                    }

                    if let Some(idx) = navigate_to {
                        self.current_page = idx;
                    }
                    if open_rules {
                        let current_rules =
                            self.db.as_ref().map(|d| d.get_rules()).unwrap_or_default();
                        self.rules_modal.open_with(current_rules);
                    }
                });
            });

        // 4. Modals
        let mut command_target = None;
        self.command_palette.show(ctx, &colors, &mut |idx| {
            command_target = Some(idx);
        });
        if let Some(idx) = command_target {
            self.current_page = idx;
        }

        self.rules_modal.show(ctx, &colors, &mut |rules| {
            if let Some(d) = &self.db {
                d.set_rules(&rules);
            }
        });

        self.auth_modal
            .show(ctx, &colors, self.auth_client.as_ref());

        // 5. Toast overlays
        self.toast_manager.show(ctx, &colors);
    }
}
