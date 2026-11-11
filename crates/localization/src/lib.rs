use std::sync::RwLock;

pub mod thai;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English,
    Thai,
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Thai => "th",
        }
    }

    pub fn from_code(code: &str) -> Self {
        match code.to_lowercase().as_str() {
            "th" | "thai" => Language::Thai,
            _ => Language::English,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Thai => "ไทย",
        }
    }
}

static CURRENT_LANG: RwLock<Language> = RwLock::new(Language::English);

/// Set global application language.
pub fn set_language(lang: Language) {
    if let Ok(mut lock) = CURRENT_LANG.write() {
        *lock = lang;
    }
}

/// Get current application language.
pub fn current_language() -> Language {
    CURRENT_LANG.read().map_or(Language::English, |l| *l)
}

/// Pure translation function for a specific language.
pub fn translate(lang: Language, text: &'static str) -> &'static str {
    if lang == Language::Thai {
        if let Some(translated) = thai::lookup(text) {
            return translated;
        }
    }
    text
}

/// Translate static text into the active global language.
pub fn tr(text: &'static str) -> &'static str {
    translate(current_language(), text)
}

/// Format translated text with placeholder replacement for a specific language.
pub fn translate_fmt(lang: Language, text: &'static str, replacements: &[(&str, &str)]) -> String {
    let mut result = translate(lang, text).to_string();
    for (key, val) in replacements {
        let pattern = format!("{{{key}}}");
        result = result.replace(&pattern, val);
    }
    result
}

/// Format translated text with placeholder replacement into active global language.
pub fn tr_fmt(text: &'static str, replacements: &[(&str, &str)]) -> String {
    translate_fmt(current_language(), text, replacements)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_translations() {
        assert_eq!(translate(Language::English, "Dashboard"), "Dashboard");
        assert_eq!(translate(Language::Thai, "Dashboard"), "แดชบอร์ด");
        assert_eq!(translate(Language::Thai, "File Organizer"), "จัดระเบียบไฟล์");
        assert_eq!(translate(Language::Thai, "Settings"), "การตั้งค่า");
    }

    #[test]
    fn test_translate_fmt() {
        assert_eq!(
            translate_fmt(
                Language::English,
                "Moving {i}/{total}…",
                &[("i", "5"), ("total", "10")]
            ),
            "Moving 5/10…"
        );

        assert_eq!(
            translate_fmt(
                Language::Thai,
                "Moving {i}/{total}…",
                &[("i", "5"), ("total", "10")]
            ),
            "กำลังย้าย 5/10…"
        );
    }
}
