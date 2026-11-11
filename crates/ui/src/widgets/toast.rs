//! Non-blocking Toast notification system.

use crate::theme::ThemeColors;
use egui::{Align2, Color32, Frame, Margin, Rounding, Stroke};
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastKind {
    Success,
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct Toast {
    pub message: String,
    pub kind: ToastKind,
    pub created_at: Instant,
    pub duration_secs: f32,
}

impl Toast {
    pub fn success(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
            kind: ToastKind::Success,
            created_at: Instant::now(),
            duration_secs: 4.0,
        }
    }

    pub fn info(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
            kind: ToastKind::Info,
            created_at: Instant::now(),
            duration_secs: 4.0,
        }
    }

    pub fn warning(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
            kind: ToastKind::Warning,
            created_at: Instant::now(),
            duration_secs: 5.0,
        }
    }

    pub fn error(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
            kind: ToastKind::Error,
            created_at: Instant::now(),
            duration_secs: 6.0,
        }
    }

    #[must_use]
    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed().as_secs_f32() > self.duration_secs
    }

    pub fn color(&self, colors: &ThemeColors) -> Color32 {
        match self.kind {
            ToastKind::Success => colors.green,
            ToastKind::Info => colors.accent_btn,
            ToastKind::Warning => colors.amber,
            ToastKind::Error => colors.red,
        }
    }
}

pub struct ToastManager {
    pub toasts: Vec<Toast>,
}

impl Default for ToastManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ToastManager {
    #[must_use]
    pub fn new() -> Self {
        Self { toasts: Vec::new() }
    }

    pub fn add(&mut self, toast: Toast) {
        self.toasts.push(toast);
    }

    pub fn show(&mut self, ctx: &egui::Context, colors: &ThemeColors) {
        self.toasts.retain(|t| !t.is_expired());

        if self.toasts.is_empty() {
            return;
        }

        egui::Area::new(egui::Id::new("toast_area"))
            .anchor(Align2::RIGHT_BOTTOM, egui::vec2(-20.0, -20.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    for toast in &self.toasts {
                        let fill = toast.color(colors);
                        Frame::none()
                            .fill(fill)
                            .rounding(Rounding::same(10.0))
                            .stroke(Stroke::new(1.0_f32, colors.border))
                            .inner_margin(Margin::symmetric(16.0, 10.0))
                            .show(ui, |ui| {
                                ui.colored_label(Color32::WHITE, &toast.message);
                            });
                        ui.add_space(4.0);
                    }
                });
            });
    }
}
