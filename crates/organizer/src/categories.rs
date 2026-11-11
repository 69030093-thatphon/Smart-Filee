use std::path::Path;

pub const CATEGORY_MAP: &[(&str, &[&str])] = &[
    (
        "Images",
        &[
            "png", "jpg", "jpeg", "gif", "webp", "bmp", "svg", "heic", "tiff", "ico",
        ],
    ),
    (
        "Documents",
        &[
            "pdf", "doc", "docx", "txt", "rtf", "odt", "xls", "xlsx", "ppt", "pptx", "csv", "md",
        ],
    ),
    ("Archives", &["zip", "7z", "rar", "tar", "gz", "bz2", "xz"]),
    ("Installers", &["exe", "msi", "dmg", "pkg", "deb", "apk"]),
    ("Audio", &["mp3", "wav", "flac", "m4a", "ogg", "aac"]),
    ("Videos", &["mp4", "mkv", "mov", "avi", "webm", "wmv"]),
    (
        "Code",
        &[
            "py", "js", "ts", "html", "css", "json", "lua", "c", "cpp", "java", "cs", "go", "rs",
            "sh", "bat",
        ],
    ),
];

/// Categorize a file path based on its extension into one of 7 categories or "Others".
pub fn categorize(path: &Path) -> &'static str {
    let ext = match path.extension() {
        Some(e) => e.to_string_lossy().to_lowercase(),
        None => return "Others",
    };

    for &(category, extensions) in CATEGORY_MAP {
        if extensions.contains(&ext.as_str()) {
            return category;
        }
    }

    "Others"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_categorize() {
        assert_eq!(categorize(Path::new("photo.JPG")), "Images");
        assert_eq!(categorize(Path::new("doc.pdf")), "Documents");
        assert_eq!(categorize(Path::new("archive.tar.gz")), "Archives");
        assert_eq!(categorize(Path::new("setup.exe")), "Installers");
        assert_eq!(categorize(Path::new("song.mp3")), "Audio");
        assert_eq!(categorize(Path::new("movie.mkv")), "Videos");
        assert_eq!(categorize(Path::new("main.rs")), "Code");
        assert_eq!(categorize(Path::new("unknown.xyz")), "Others");
        assert_eq!(categorize(Path::new("no_extension")), "Others");
    }
}
