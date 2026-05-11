//! MIME magic-byte detection using the `infer` crate.
//!
//! Wins over the client's `Content-Type` header when they disagree, per
//! `security-rules.md` (anti-spoof). Called from the router's MIME step.

use mime::Mime;

/// Sniff the first few bytes of a body and return the detected MIME type
/// if recognized. Returns `None` for unknown formats; the caller may then
/// fall through to the extension step.
pub fn sniff_mime(body: &[u8]) -> Option<Mime> {
    infer::get(body).and_then(|t| t.mime_type().parse().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sniff_pdf_magic() {
        let body = b"%PDF-1.4\n%test content\n";
        let m = sniff_mime(body);
        assert_eq!(m.as_ref().map(Mime::essence_str), Some("application/pdf"));
    }

    #[test]
    fn sniff_png_magic() {
        let body = b"\x89PNG\r\n\x1a\n";
        let m = sniff_mime(body);
        assert_eq!(m.as_ref().map(Mime::essence_str), Some("image/png"));
    }

    #[test]
    fn sniff_jpeg_magic() {
        let body = b"\xff\xd8\xff\xe0\x00\x10JFIF";
        let m = sniff_mime(body);
        assert_eq!(m.as_ref().map(Mime::essence_str), Some("image/jpeg"));
    }

    #[test]
    fn sniff_unknown_returns_none() {
        let body = b"random bytes that are not a recognized format header";
        assert!(sniff_mime(body).is_none());
    }

    #[test]
    fn sniff_empty_returns_none() {
        assert!(sniff_mime(&[]).is_none());
    }
}
