//! Custom progress ring widget for Tidy Score.

use crate::theme::ThemeColors;
use egui::{vec2, Align2, FontFamily, FontId, Pos2, Sense, Stroke, Ui};
use std::f32::consts::PI;

pub struct Ring<'a> {
    value: f32,
    text: &'a str,
    diameter: f32,
    thickness: f32,
}

impl<'a> Ring<'a> {
    #[must_use]
    pub fn new(value: f32, text: &'a str) -> Self {
        Self {
            value: value.clamp(0.0, 100.0),
            text,
            diameter: 130.0,
            thickness: 14.0,
        }
    }

    #[must_use]
    pub fn diameter(mut self, diameter: f32) -> Self {
        self.diameter = diameter;
        self
    }

    #[must_use]
    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn show(self, ui: &mut Ui, colors: &ThemeColors) {
        let (rect, _response) =
            ui.allocate_exact_size(vec2(self.diameter, self.diameter), Sense::hover());

        let painter = ui.painter_at(rect);
        let center = rect.center();
        let radius = (self.diameter - self.thickness) / 2.0;

        // 1. Draw track circle
        painter.circle_stroke(center, radius, Stroke::new(self.thickness, colors.navsel));

        // 2. Draw value arc
        if self.value > 0.0 {
            let num_points = ((self.value / 100.0) * 60.0).max(4.0) as usize;
            let start_angle = -PI / 2.0;
            let sweep_angle = 2.0 * PI * (self.value / 100.0);

            let points: Vec<Pos2> = (0..=num_points)
                .map(|i| {
                    let fraction = i as f32 / num_points as f32;
                    let angle = start_angle + sweep_angle * fraction;
                    Pos2::new(
                        center.x + radius * angle.cos(),
                        center.y + radius * angle.sin(),
                    )
                })
                .collect();

            let arc_stroke = Stroke::new(self.thickness, colors.accent_btn);
            for w in points.windows(2) {
                painter.line_segment([w[0], w[1]], arc_stroke);
            }
        }

        // 3. Draw center score text
        painter.text(
            center,
            Align2::CENTER_CENTER,
            self.text,
            FontId::new(self.diameter * 0.28, FontFamily::Proportional),
            colors.text,
        );
    }
}
