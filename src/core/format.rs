use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

pub fn format_size(bytes: u64) -> String {
    match bytes {
        b if b < 1_024 => format!("{} B", b),
        b if b < 1_048_576 => format!("{:.1} KB", b as f64 / 1_024.0),
        b if b < 1_073_741_824 => format!("{:.1} MB", b as f64 / 1_048_576.0),
        b => format!("{:.2} GB", b as f64 / 1_073_741_824.0),
    }
}

/// Format an RFC3339 timestamp string as "YYYY-MM-DD HH:MM:SS UTC".
pub fn format_datetime(rfc3339: &str) -> String {
    OffsetDateTime::parse(rfc3339, &Rfc3339)
        .map(|t| {
            format!(
                "{} {:02}:{:02}:{:02} UTC",
                t.date(),
                t.hour(),
                t.minute(),
                t.second()
            )
        })
        .unwrap_or_else(|_| rfc3339.to_string())
}

/// Format a Unix timestamp (seconds) as a date string "YYYY-MM-DD".
pub fn format_date_unix(unix_secs: i64) -> String {
    OffsetDateTime::from_unix_timestamp(unix_secs)
        .map(|t| t.date().to_string())
        .unwrap_or_else(|_| "-".to_string())
}

pub fn truncate(s: &str, max_len: usize) -> String {
    if s.chars().count() <= max_len {
        return s.to_string();
    }
    if max_len <= 3 {
        return ".".repeat(max_len);
    }
    let mut out: String = s.chars().take(max_len - 3).collect();
    out.push_str("...");
    out
}
