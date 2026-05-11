//! File-extension → MIME mapping.
//!
//! Single source of truth for the `extension` step in the routing
//! precedence chain. New entries land per ADR amendment.

use mime::Mime;

/// Map a file extension (without the leading dot, case-insensitive) to a
/// MIME type if known.
pub fn mime_from_extension(ext: &str) -> Option<Mime> {
    let lower = ext.to_ascii_lowercase();
    match lower.as_str() {
        "pdf" => Some(mime::APPLICATION_PDF),
        "html" | "htm" => Some(mime::TEXT_HTML),
        "txt" => Some(mime::TEXT_PLAIN),
        "json" => Some(mime::APPLICATION_JSON),
        "png" => Some(mime::IMAGE_PNG),
        "jpg" | "jpeg" => Some(mime::IMAGE_JPEG),
        "gif" => Some(mime::IMAGE_GIF),
        "doc" => "application/msword".parse().ok(),
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
            .parse()
            .ok(),
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
            .parse()
            .ok(),
        "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation"
            .parse()
            .ok(),
        "md" | "markdown" => "text/markdown".parse().ok(),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;

    #[test]
    fn case_insensitive_lookup() {
        assert_eq!(mime_from_extension("PDF"), Some(mime::APPLICATION_PDF));
        assert_eq!(mime_from_extension("pdf"), Some(mime::APPLICATION_PDF));
    }

    #[test]
    fn aliases_resolve_to_same_type() {
        assert_eq!(mime_from_extension("htm"), mime_from_extension("html"));
        assert_eq!(mime_from_extension("jpg"), mime_from_extension("jpeg"));
        assert_eq!(mime_from_extension("md"), mime_from_extension("markdown"));
    }

    #[test]
    fn unknown_returns_none() {
        assert!(mime_from_extension("unknown_xyz").is_none());
        assert!(mime_from_extension("").is_none());
    }

    #[test]
    fn office_extensions_parse_to_proper_mimes() {
        let docx = mime_from_extension("docx").unwrap();
        assert!(docx.essence_str().contains("wordprocessingml"));
        let xlsx = mime_from_extension("xlsx").unwrap();
        assert!(xlsx.essence_str().contains("spreadsheetml"));
    }
}
