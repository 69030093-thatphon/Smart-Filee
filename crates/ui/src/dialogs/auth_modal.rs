//! Supabase Authentication & Profile Modal Dialog.

use crate::theme::ThemeColors;
use egui::{Align2, Frame, Margin, Rounding, Stroke};
use smart_filee_auth::{AuthSession, SupabaseClient};
use smart_filee_localization::tr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AuthTab {
    #[default]
    SignIn,
    SignUp,
    Profile,
}

pub struct AuthModal {
    pub is_open: bool,
    pub tab: AuthTab,
    pub email: String,
    pub password: String,
    pub display_name: String,
    pub status_message: Option<(bool, String)>,
    pub session: Option<AuthSession>,
}

impl Default for AuthModal {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthModal {
    #[must_use]
    pub fn new() -> Self {
        Self {
            is_open: false,
            tab: AuthTab::SignIn,
            email: String::new(),
            password: String::new(),
            display_name: String::new(),
            status_message: None,
            session: None,
        }
    }

    pub fn open(&mut self) {
        self.is_open = true;
        self.status_message = None;
    }

    pub fn show(
        &mut self,
        ctx: &egui::Context,
        colors: &ThemeColors,
        client: Option<&SupabaseClient>,
    ) {
        if !self.is_open {
            return;
        }

        let mut close = false;

        egui::Window::new("Authentication & Account")
            .anchor(Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .fixed_size(egui::vec2(460.0, 420.0))
            .collapsible(false)
            .resizable(false)
            .frame(
                Frame::none()
                    .fill(colors.panel)
                    .stroke(Stroke::new(1.0_f32, colors.border))
                    .rounding(Rounding::same(16.0))
                    .inner_margin(Margin::same(24.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.tab, AuthTab::SignIn, tr("Sign In"));
                    ui.selectable_value(&mut self.tab, AuthTab::SignUp, tr("Sign Up"));
                    if self.session.is_some() {
                        ui.selectable_value(&mut self.tab, AuthTab::Profile, tr("Profile"));
                    }
                });

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(12.0);

                if let Some((is_success, msg)) = &self.status_message {
                    let col = if *is_success { colors.green } else { colors.red };
                    ui.colored_label(col, msg);
                    ui.add_space(8.0);
                }

                match self.tab {
                    AuthTab::SignIn => {
                        ui.heading(tr("Sign in to your account"));
                        ui.add_space(8.0);

                        ui.label("Email");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.email)
                                .hint_text("user@example.com")
                                .desired_width(ui.available_width()),
                        );
                        ui.add_space(6.0);

                        ui.label("Password");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.password)
                                .password(true)
                                .desired_width(ui.available_width()),
                        );
                        ui.add_space(16.0);

                        if ui.button(tr("Sign In")).clicked() {
                            if let Some(c) = client {
                                match c.sign_in_with_password(&self.email, &self.password) {
                                    Ok(session) => {
                                        self.display_name = session.user.display_name.clone();
                                        self.session = Some(session);
                                        self.status_message = Some((true, "Signed in successfully!".to_string()));
                                        self.tab = AuthTab::Profile;
                                    }
                                    Err(e) => {
                                        self.status_message = Some((false, format!("Login error: {e}")));
                                    }
                                }
                            } else {
                                self.status_message = Some((false, "Supabase client not configured or publishable key missing.".to_string()));
                            }
                        }
                    }
                    AuthTab::SignUp => {
                        ui.heading(tr("Create a new account"));
                        ui.add_space(8.0);

                        ui.label("Email");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.email)
                                .hint_text("user@example.com")
                                .desired_width(ui.available_width()),
                        );
                        ui.add_space(6.0);

                        ui.label("Password");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.password)
                                .password(true)
                                .desired_width(ui.available_width()),
                        );
                        ui.add_space(16.0);

                        if ui.button(tr("Sign Up")).clicked() {
                            if let Some(c) = client {
                                match c.sign_up_with_password(&self.email, &self.password) {
                                    Ok(_) => {
                                        self.status_message = Some((
                                            true,
                                            "Verification email sent! Check your inbox or click the link.".to_string(),
                                        ));
                                    }
                                    Err(e) => {
                                        self.status_message = Some((false, format!("Sign up error: {e}")));
                                    }
                                }
                            } else {
                                self.status_message = Some((false, "Supabase client not configured.".to_string()));
                            }
                        }
                    }
                    AuthTab::Profile => {
                        ui.heading(tr("Account Profile"));
                        ui.add_space(8.0);

                        let mut do_sign_out = false;
                        let mut do_update_name = false;
                        let mut access_token = String::new();

                        if let Some(session) = &self.session {
                            access_token = session.access_token.clone();
                            ui.label(format!("User ID: {}", session.user.id));
                            ui.label(format!("Email: {}", session.user.email));
                            ui.add_space(8.0);

                            ui.label(tr("Display name"));
                            ui.add(
                                egui::TextEdit::singleline(&mut self.display_name)
                                    .desired_width(ui.available_width()),
                            );
                            ui.add_space(12.0);

                            ui.horizontal(|ui| {
                                if ui.button(tr("Save profile")).clicked() {
                                    do_update_name = true;
                                }

                                if ui.button(tr("Sign Out")).clicked() {
                                    do_sign_out = true;
                                }
                            });
                        }

                        if do_update_name {
                            if let Some(c) = client {
                                if let Err(e) = c.update_profile(&access_token, &self.display_name) {
                                    self.status_message = Some((false, format!("Update failed: {e}")));
                                } else {
                                    self.status_message = Some((true, "Profile updated!".to_string()));
                                }
                            }
                        }

                        if do_sign_out {
                            self.session = None;
                            self.tab = AuthTab::SignIn;
                            self.status_message = Some((true, "Signed out.".to_string()));
                        }
                    }
                }

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(8.0);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(tr("Close")).clicked() {
                        close = true;
                    }
                });
            });

        if close {
            self.is_open = false;
        }
    }
}
