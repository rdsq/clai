use atty::Stream;
use regex::Regex;

pub fn markdown_to_ansi(md: &str) -> String {
    // If stdout is not a TTY, return plain text.
    if !atty::is(Stream::Stdout) {
        return md.to_string();
    }

    let mut result = md.to_string();

    // Bold: **text**
    let bold = Regex::new(r"\*\*(.+?)\*\*").unwrap();
    result = bold.replace_all(&result, "\x1b[1m$1\x1b[0m").to_string();

    // Italic: *text*
    let italic = Regex::new(r"\*(.+?)\*").unwrap();
    result = italic.replace_all(&result, "\x1b[3m$1\x1b[0m").to_string();

    result
}
