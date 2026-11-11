//! Color themes and styling for Smart-Filee UI.
//!
//! Preserves the Premium Light Violet visual identity and provides 6 built-in themes.

use egui::{Color32, Margin, Rounding, Stroke, Visuals};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    #[default]
    PremiumLightViolet,
    Light,
    NeonViolet,
    Steam,
    CyberCyan,
    CrimsonNight,
}

impl Theme {
    pub const ALL: [Theme; 6] = [
        Theme::PremiumLightViolet,
        Theme::Light,
        Theme::NeonViolet,
        Theme::Steam,
        Theme::CyberCyan,
        Theme::CrimsonNight,
    ];

    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::PremiumLightViolet => "Premium Light Violet",
            Self::Light => "Light",
            Self::NeonViolet => "Neon Violet",
            Self::Steam => "Steam",
            Self::CyberCyan => "Cyber Cyan",
            Self::CrimsonNight => "Crimson Night",
        }
    }

    #[must_use]
    pub const fn is_dark(&self) -> bool {
        match self {
            Self::PremiumLightViolet | Self::Light => false,
            Self::NeonViolet | Self::Steam | Self::CyberCyan | Self::CrimsonNight => true,
        }
    }

    #[must_use]
    pub fn colors(&self) -> ThemeColors {
        match self {
            Self::PremiumLightViolet => ThemeColors {
                bg: Color32::from_rgb(0xF7, 0xF6, 0xFC),
                sidebar: Color32::from_rgb(0xFF, 0xFF, 0xFF),
                panel: Color32::from_rgb(0xFF, 0xFF, 0xFF),
                panel2: Color32::from_rgb(0xF1, 0xEE, 0xFA),
                text: Color32::from_rgb(0x24, 0x21, 0x3A),
                muted: Color32::from_rgb(0x77, 0x73, 0x8F),
                accent: Color32::from_rgb(0x8B, 0x72, 0xF8),
                accent_btn: Color32::from_rgb(0x76, 0x57, 0xE8),
                accent2: Color32::from_rgb(0xA7, 0x8B, 0xFA),
                green: Color32::from_rgb(0x15, 0x9B, 0x72),
                amber: Color32::from_rgb(0xC9, 0x82, 0x20),
                red: Color32::from_rgb(0xD9, 0x4B, 0x68),
                border: Color32::from_rgb(0xE7, 0xE2, 0xF2),
                hover: Color32::from_rgb(0xF0, 0xEC, 0xFC),
                navsel: Color32::from_rgb(0xEA, 0xE4, 0xFF),
            },
            Self::Light => ThemeColors {
                bg: Color32::from_rgb(0xF4, 0xF6, 0xFB),
                sidebar: Color32::from_rgb(0xFF, 0xFF, 0xFF),
                panel: Color32::from_rgb(0xFF, 0xFF, 0xFF),
                panel2: Color32::from_rgb(0xEE, 0xF1, 0xF8),
                text: Color32::from_rgb(0x14, 0x18, 0x2B),
                muted: Color32::from_rgb(0x5B, 0x65, 0x85),
                accent: Color32::from_rgb(0x6D, 0x4E, 0xD8),
                accent_btn: Color32::from_rgb(0x6D, 0x4E, 0xD8),
                accent2: Color32::from_rgb(0x08, 0x91, 0xB2),
                green: Color32::from_rgb(0x05, 0x96, 0x69),
                amber: Color32::from_rgb(0xD9, 0x77, 0x06),
                red: Color32::from_rgb(0xE1, 0x1D, 0x48),
                border: Color32::from_rgb(0xD5, 0xDB, 0xEA),
                hover: Color32::from_rgb(0xEE, 0xEA, 0xFB),
                navsel: Color32::from_rgb(0xE4, 0xDE, 0xFA),
            },
            Self::NeonViolet => ThemeColors {
                bg: Color32::from_rgb(0x0B, 0x10, 0x20),
                sidebar: Color32::from_rgb(0x08, 0x0D, 0x19),
                panel: Color32::from_rgb(0x12, 0x1A, 0x2E),
                panel2: Color32::from_rgb(0x17, 0x22, 0x3A),
                text: Color32::from_rgb(0xEE, 0xF2, 0xFF),
                muted: Color32::from_rgb(0x9A, 0xA8, 0xC7),
                accent: Color32::from_rgb(0xA7, 0x8B, 0xFA),
                accent_btn: Color32::from_rgb(0x6D, 0x4E, 0xD8),
                accent2: Color32::from_rgb(0x22, 0xD3, 0xEE),
                green: Color32::from_rgb(0x34, 0xD3, 0x99),
                amber: Color32::from_rgb(0xFB, 0xBF, 0x24),
                red: Color32::from_rgb(0xFB, 0x71, 0x85),
                border: Color32::from_rgb(0x27, 0x36, 0x53),
                hover: Color32::from_rgb(0x20, 0x2D, 0x4A),
                navsel: Color32::from_rgb(0x24, 0x1D, 0x46),
            },
            Self::Steam => ThemeColors {
                bg: Color32::from_rgb(0x1B, 0x28, 0x38),
                sidebar: Color32::from_rgb(0x17, 0x1D, 0x25),
                panel: Color32::from_rgb(0x16, 0x20, 0x2D),
                panel2: Color32::from_rgb(0x22, 0x36, 0x4A),
                text: Color32::from_rgb(0xDF, 0xE3, 0xE6),
                muted: Color32::from_rgb(0x8F, 0x98, 0xA0),
                accent: Color32::from_rgb(0x66, 0xC0, 0xF4),
                accent_btn: Color32::from_rgb(0x2F, 0x7A, 0xB8),
                accent2: Color32::from_rgb(0xA4, 0xD0, 0x07),
                green: Color32::from_rgb(0xA4, 0xD0, 0x07),
                amber: Color32::from_rgb(0xE5, 0xB1, 0x43),
                red: Color32::from_rgb(0xE8, 0x58, 0x5A),
                border: Color32::from_rgb(0x2A, 0x3F, 0x55),
                hover: Color32::from_rgb(0x2A, 0x47, 0x5E),
                navsel: Color32::from_rgb(0x2A, 0x47, 0x5E),
            },
            Self::CyberCyan => ThemeColors {
                bg: Color32::from_rgb(0x06, 0x12, 0x1A),
                sidebar: Color32::from_rgb(0x04, 0x0C, 0x12),
                panel: Color32::from_rgb(0x0C, 0x1F, 0x2C),
                panel2: Color32::from_rgb(0x10, 0x29, 0x3A),
                text: Color32::from_rgb(0xE6, 0xFB, 0xFF),
                muted: Color32::from_rgb(0x8F, 0xB4, 0xC2),
                accent: Color32::from_rgb(0x22, 0xD3, 0xEE),
                accent_btn: Color32::from_rgb(0x08, 0x91, 0xB2),
                accent2: Color32::from_rgb(0xA3, 0xE6, 0x35),
                green: Color32::from_rgb(0x34, 0xD3, 0x99),
                amber: Color32::from_rgb(0xFB, 0xBF, 0x24),
                red: Color32::from_rgb(0xFB, 0x71, 0x85),
                border: Color32::from_rgb(0x1B, 0x44, 0x58),
                hover: Color32::from_rgb(0x14, 0x35, 0x49),
                navsel: Color32::from_rgb(0x0E, 0x34, 0x45),
            },
            Self::CrimsonNight => ThemeColors {
                bg: Color32::from_rgb(0x14, 0x0A, 0x0E),
                sidebar: Color32::from_rgb(0x0E, 0x07, 0x0A),
                panel: Color32::from_rgb(0x22, 0x12, 0x1A),
                panel2: Color32::from_rgb(0x2B, 0x17, 0x22),
                text: Color32::from_rgb(0xFF, 0xEE, 0xF2),
                muted: Color32::from_rgb(0xC9, 0xA0, 0xAD),
                accent: Color32::from_rgb(0xFB, 0x71, 0x85),
                accent_btn: Color32::from_rgb(0xBE, 0x12, 0x3C),
                accent2: Color32::from_rgb(0xFB, 0xBF, 0x24),
                green: Color32::from_rgb(0x34, 0xD3, 0x99),
                amber: Color32::from_rgb(0xFB, 0xBF, 0x24),
                red: Color32::from_rgb(0xF4, 0x3F, 0x5E),
                border: Color32::from_rgb(0x4A, 0x24, 0x33),
                hover: Color32::from_rgb(0x3A, 0x1D, 0x2A),
                navsel: Color32::from_rgb(0x3B, 0x15, 0x22),
            },
        }
    }

    pub fn apply(&self, ctx: &egui::Context) {
        let colors = self.colors();
        let is_dark = self.is_dark();

        let mut visuals = if is_dark {
            Visuals::dark()
        } else {
            Visuals::light()
        };

        visuals.override_text_color = Some(colors.text);
        visuals.panel_fill = colors.bg;
        visuals.window_fill = colors.panel;
        visuals.faint_bg_color = colors.panel2;
        visuals.extreme_bg_color = colors.panel2;

        visuals.widgets.noninteractive.bg_fill = colors.panel;
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, colors.border);
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, colors.text);
        visuals.widgets.noninteractive.rounding = Rounding::same(12.0);

        visuals.widgets.inactive.bg_fill = colors.panel2;
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, colors.border);
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, colors.text);
        visuals.widgets.inactive.rounding = Rounding::same(10.0);

        visuals.widgets.hovered.bg_fill = colors.hover;
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, colors.accent);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, colors.accent_btn);
        visuals.widgets.hovered.rounding = Rounding::same(10.0);

        visuals.widgets.active.bg_fill = colors.navsel;
        visuals.widgets.active.bg_stroke = Stroke::new(1.5_f32, colors.accent_btn);
        visuals.widgets.active.fg_stroke = Stroke::new(1.5_f32, colors.accent_btn);
        visuals.widgets.active.rounding = Rounding::same(10.0);

        visuals.selection.bg_fill = colors.navsel;
        visuals.selection.stroke = Stroke::new(1.0_f32, colors.accent_btn);

        ctx.set_visuals(visuals);

        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        style.spacing.button_padding = egui::vec2(14.0, 8.0);
        style.spacing.window_margin = Margin::same(16.0);
        ctx.set_style(style);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ThemeColors {
    pub bg: Color32,
    pub sidebar: Color32,
    pub panel: Color32,
    pub panel2: Color32,
    pub text: Color32,
    pub muted: Color32,
    pub accent: Color32,
    pub accent_btn: Color32,
    pub accent2: Color32,
    pub green: Color32,
    pub amber: Color32,
    pub red: Color32,
    pub border: Color32,
    pub hover: Color32,
    pub navsel: Color32,
}
