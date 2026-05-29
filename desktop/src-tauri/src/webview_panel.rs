/// 仅允许 http/https；其它（file:、javascript: 等）拒绝。返回规范化后的 url 字符串。
pub fn normalize_preview_url(input: &str) -> Result<String, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("empty url".to_string());
    }
    let lower = trimmed.to_ascii_lowercase();
    if lower.starts_with("http://") || lower.starts_with("https://") {
        Ok(trimmed.to_string())
    } else {
        Err(format!("unsupported url scheme: {trimmed}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_http_and_https() {
        assert_eq!(normalize_preview_url("http://localhost:5173/").unwrap(), "http://localhost:5173/");
        assert_eq!(normalize_preview_url("https://example.com").unwrap(), "https://example.com");
    }

    #[test]
    fn trims_whitespace() {
        assert_eq!(normalize_preview_url("  http://127.0.0.1:8080  ").unwrap(), "http://127.0.0.1:8080");
    }

    #[test]
    fn rejects_non_http_schemes() {
        assert!(normalize_preview_url("file:///etc/passwd").is_err());
        assert!(normalize_preview_url("javascript:alert(1)").is_err());
        assert!(normalize_preview_url("").is_err());
    }
}
