//! Desktop User Interface for Smart-Filee built with `egui` and `eframe`.

pub mod app;
pub mod dialogs;
pub mod pages;
pub mod theme;
pub mod widgets;

pub use app::SmartFileeApp;
pub use theme::{Theme, ThemeColors};

/// Run the native desktop application.
///
/// # Errors
///
/// Returns an error if the graphical backend or window creation fails.
pub fn run_desktop_app() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Smart-Filee Pro (Rust Edition)")
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([960.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Smart-Filee Pro",
        native_options,
        Box::new(|cc| {
            // Apply initial theme
            let app = SmartFileeApp::new();
            app.theme.apply(&cc.egui_ctx);
            Ok(Box::new(app))
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dialogs::CommandPalette;
    use crate::theme::Theme;
    use crate::widgets::{Toast, ToastKind, ToastManager};

    #[test]
    fn test_app_initialization() {
        let app = SmartFileeApp::new();
        assert_eq!(app.current_page, 0);
        assert_eq!(app.theme, Theme::PremiumLightViolet);
        assert_eq!(app.toast_manager.toasts.len(), 0);
    }

    #[test]
    fn test_themes_coverage() {
        for theme in Theme::ALL {
            let colors = theme.colors();
            assert_ne!(colors.bg, colors.text);
            assert_ne!(colors.accent, colors.bg);
            assert!(!theme.name().is_empty());
        }
    }

    #[test]
    fn test_command_palette_filter() {
        let mut palette = CommandPalette::new();
        assert!(!palette.is_open);
        palette.toggle();
        assert!(palette.is_open);
        palette.query = "Disk".to_string();
        palette.toggle();
        assert!(!palette.is_open);
    }

    #[test]
    fn test_toast_manager_lifecycle() {
        let mut manager = ToastManager::new();
        manager.add(Toast::success("File organized"));
        manager.add(Toast::error("Failed to copy"));
        assert_eq!(manager.toasts.len(), 2);
        assert_eq!(manager.toasts[0].kind, ToastKind::Success);
        assert_eq!(manager.toasts[1].kind, ToastKind::Error);
    }

    #[test]
    fn test_nav_items_count_and_shortcuts() {
        assert_eq!(app::NAV_ITEMS.len(), 14);
        assert_eq!(app::NAV_ITEMS[0].title, "Dashboard");
        assert_eq!(app::NAV_ITEMS[13].title, "Settings");
    }
}
