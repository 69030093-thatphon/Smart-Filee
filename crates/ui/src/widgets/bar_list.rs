//! Custom BarList widget displaying rows of label | colored bar | value.

use crate::theme::ThemeColors;
use egui::{vec2, Align2, Color32, FontFamily, FontId, Pos2, Rect, Rounding, Sense, Stroke, Ui};

#[derive(Debug, Clone)]
pub struct BarItem {
    pub label: String,
    pub fraction: f32,
    pub value_text: String,
    pub color: Option<Color32>,
}

impl BarItem {
    pub fn new(label: impl Into<String>, fraction: f32, value_text: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            fraction: fraction.clamp(0.0, 1.0),
            value_text: value_text.into(),
            color: None,
        }
    }

    pub fn with_color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }
}

pub struct BarList<'a> {
    items: &'a [BarItem],
    empty_text: &'a str,
}

impl<'a> BarList<'a> {
    #[must_use]
    pub fn new(items: &'a [BarItem], empty_text: &'a str) -> Self {
        Self { items, empty_text }
    }

    pub fn show(self, ui: &mut Ui, colors: &ThemeColors) {
        if self.items.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(12.0);
                ui.colored_label(colors.muted, self.empty_text);
                ui.add_space(12.0);
            });
            return;
        }

        let row_height = 36.0;
        let total_height = self.items.len() as f32 * row_height;
        let available_width = ui.available_width().max(200.0);

        let (rect, _response) =
            ui.allocate_exact_size(vec2(available_width, total_height), Sense::hover());

        let painter = ui.painter_at(rect);

        let label_width = 130.0_f32.min(available_width * 0.35);
        let value_width = 90.0_f32.min(available_width * 0.25);
        let bar_width = (available_width - label_width - value_width - 24.0).max(40.0);

        for (idx, item) in self.items.iter().enumerate() {
            let y = rect.min.y + idx as f32 * row_height;

            // Draw divider line if not first
            if idx > 0 {
                painter.line_segment(
                    [Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)],
                    Stroke::new(1.0_f32, colors.border),
                );
            }

            let text_y = y + row_height / 2.0;

            // 1. Label
            painter.text(
                Pos2::new(rect.min.x + 4.0, text_y),
                Align2::LEFT_CENTER,
                &item.label,
                FontId::new(13.0, FontFamily::Proportional),
                colors.text,
            );

            // 2. Bar Track
            let bar_x = rect.min.x + label_width + 8.0;
            let bar_h = 10.0;
            let bar_rect = Rect::from_min_size(
                Pos2::new(bar_x, text_y - bar_h / 2.0),
                vec2(bar_width, bar_h),
            );
            painter.rect_filled(bar_rect, Rounding::same(5.0), colors.panel2);

            // Bar Fill
            let fill_width = (bar_width * item.fraction).max(0.0);
            if fill_width > 0.0 {
                let fill_rect = Rect::from_min_size(
                    Pos2::new(bar_x, text_y - bar_h / 2.0),
                    vec2(fill_width, bar_h),
                );
                let fill_color = item.color.unwrap_or(colors.accent_btn);
                painter.rect_filled(fill_rect, Rounding::same(5.0), fill_color);
            }

            // 3. Value
            painter.text(
                Pos2::new(rect.max.x - 4.0, text_y),
                Align2::RIGHT_CENTER,
                &item.value_text,
                FontId::new(12.0, FontFamily::Proportional),
                colors.muted,
            );
        }
    }
}
