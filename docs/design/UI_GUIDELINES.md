# Smart File Manager Pro — UI/UX Design Guidelines

**Document Version:** 1.0.0
**Date:** 2026-11-10
**Design Lead:** ณัฐพงศ์ เหินหาว (`team/ui-design`)

---

## 1. Design Philosophy

Smart File Manager Pro emphasizes visual clarity, high information density without clutter, accessibility across light and dark environments, and responsive user feedback.

---

## 2. Color Palettes & Themes

The application includes 6 built-in themes defined in `themes.py`:

| Theme Name | Background | Sidebar | Accent Button | Description |
|---|---|---|---|---|
| **Premium Light Violet** | `#F7F6FC` | `#FFFFFF` | `#7657E8` | Default refined violet theme with soft lavender cards. |
| **Light** | `#F4F6FB` | `#FFFFFF` | `#6D4ED8` | Clean, modern light slate theme with high contrast text. |
| **Neon Violet** | `#0B1020` | `#080D19` | `#6D4ED8` | Deep midnight blue with vibrant purple accents. |
| **Steam** | `#1B2838` | `#171D25` | `#2F7AB8` | Classic gaming-inspired dark charcoal and cyan accents. |
| **Cyber Cyan** | `#06121A` | `#040C12` | `#0891B2` | High-contrast dark sci-fi cyan palette. |
| **Crimson Night** | `#140A0E` | `#0E070A` | `#BE123C` | Deep burgundy dark palette with ruby accents. |

---

## 3. Typography & Font Hierarchy

- **Primary Latin Font:** `Instrument Sans` (bundled in `fonts/`).
- **Thai & Fallback Stack:** `'Leelawadee UI', 'Tahoma', 'Noto Sans Thai', 'Noto Looped Thai', 'Segoe UI', sans-serif`.
- **Default Sizes:**
  - English: 10 pt (Base)
  - Thai: 11 pt (Base)
- **Hierarchy:**
  - Page Titles (`pageTitle`): 1.9x base font size, Bold (700).
  - Card Titles (`cardTitle`): 1.05x base font size, Bold (700).
  - Stat Values (`statValue`): 1.9x base font size, Bold (700).
  - Descriptions & Notes (`note`, `pageDesc`): 0.9x base font size, Muted color.

---

## 4. UI States & Standards

1. **Default State:** Clean layout with informative placeholders in path fields.
2. **Loading / Processing State:** Background `Worker` threads emit progress text to status labels; action buttons disable gracefully with cancel buttons displayed.
3. **Empty State:** `BarList` and tables render helpful empty text (e.g. "Nothing to organize here").
4. **Success / Error State:** Non-blocking `Toast` overlays appear in the lower right corner (green for success, red for error, amber for warning).
5. **Confirmation State:** All irreversible file movements require explicit confirmation prompts (`confirm()` dialogs).
