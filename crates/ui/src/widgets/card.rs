//! Card and StatCard widgets.

use crate::theme::ThemeColors;
use egui::{Frame, Margin, Rounding, Stroke, Ui};

pub struct Card;

impl Card {
    pub fn show<R>(
        ui: &mut Ui,
        colors: &ThemeColors,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> R {
        Frame::none()
            .fill(colors.panel)
            .stroke(Stroke::new(1.0_f32, colors.border))
            .rounding(Rounding::same(16.0))
            .inner_margin(Margin::same(16.0))
            .show(ui, add_contents)
            .inner
    }
}

pub struct StatCard<'a> {
    pub title: &'a str,
    pub value: &'a str,
    pub subtitle: Option<&'a str>,
}

impl<'a> StatCard<'a> {
    #[must_use]
    pub fn new(title: &'a str, value: &'a str) -> Self {
        Self {
            title,
            value,
            subtitle: None,
        }
    }

    #[must_use]
    pub fn subtitle(mut self, subtitle: &'a str) -> Self {
        self.subtitle = Some(subtitle);
        self
    }

    pub fn show(self, ui: &mut Ui, colors: &ThemeColors) {
        Card::show(ui, colors, |ui| {
            ui.colored_label(colors.muted, self.title);
            ui.add_space(4.0);
            ui.heading(self.value);
            if let Some(sub) = self.subtitle {
                ui.add_space(2.0);
                ui.colored_label(colors.muted, sub);
            }
        });
    }
}
