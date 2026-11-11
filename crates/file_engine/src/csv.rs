use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

/// Export rows to a CSV file with UTF-8 BOM (`\u{FEFF}`) so it opens cleanly in Microsoft Excel.
pub fn export_csv(
    path: &Path,
    headers: &[&str],
    rows: &[Vec<String>],
) -> Result<(), std::io::Error> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    // Write UTF-8 BOM (Byte Order Mark: EF BB BF)
    writer.write_all("\u{FEFF}".as_bytes())?;

    // Write headers
    write_csv_row(&mut writer, headers)?;

    // Write rows
    for row in rows {
        let refs: Vec<&str> = row.iter().map(String::as_str).collect();
        write_csv_row(&mut writer, &refs)?;
    }

    writer.flush()?;
    Ok(())
}

fn write_csv_row<W: Write>(writer: &mut W, fields: &[&str]) -> Result<(), std::io::Error> {
    for (i, field) in fields.iter().enumerate() {
        if i > 0 {
            writer.write_all(b",")?;
        }
        if field.contains(',')
            || field.contains('"')
            || field.contains('\n')
            || field.contains('\r')
        {
            writer.write_all(b"\"")?;
            writer.write_all(field.replace('"', "\"\"").as_bytes())?;
            writer.write_all(b"\"")?;
        } else {
            writer.write_all(field.as_bytes())?;
        }
    }
    writer.write_all(b"\r\n")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_export_csv_with_bom() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("report.csv");

        let headers = ["File", "Size", "Type"];
        let rows = vec![
            vec!["doc.pdf".into(), "1.2 MB".into(), "Document".into()],
            vec!["file with, comma.txt".into(), "400 B".into(), "Text".into()],
        ];

        export_csv(&path, &headers, &rows).unwrap();

        let bytes = std::fs::read(&path).unwrap();
        // Check UTF-8 BOM prefix
        assert_eq!(&bytes[0..3], b"\xEF\xBB\xBF");

        let text = std::fs::read_to_string(&path).unwrap();
        assert!(text.contains("doc.pdf,1.2 MB,Document"));
        assert!(text.contains("\"file with, comma.txt\",400 B,Text"));
    }
}
