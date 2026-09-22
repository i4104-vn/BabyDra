//! Line and text highlighting engine.

use std::error::Error;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, Theme};
use syntect::parsing::SyntaxReference;
use syntect::util::LinesWithEndings;

use super::detection::syntax_set;

/// Creates a syntect line highlighter for the given syntax and theme.
pub fn create_highlighter<'a>(syntax: &'a SyntaxReference, theme: &'a Theme) -> HighlightLines<'a> {
    HighlightLines::new(syntax, theme)
}

/// Highlights a single line of code with the provided highlighter.
pub fn highlight_line<'a>(
    highlighter: &mut HighlightLines<'a>,
    line: &'a str,
) -> Result<Vec<(Style, &'a str)>, Box<dyn Error + Send + Sync>> {
    let ps = syntax_set();
    let ranges = highlighter
        .highlight_line(line, ps)
        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
    Ok(ranges)
}

/// Highlights full text content into lines of styled spans, suitable for any rendering frontend.
pub fn highlight_text(
    text: &str,
    syntax: &SyntaxReference,
    theme: &Theme,
) -> Vec<Vec<(Style, String)>> {
    let ps = syntax_set();
    let mut highlighter = HighlightLines::new(syntax, theme);
    let mut result = Vec::new();

    for line in LinesWithEndings::from(text) {
        if let Ok(ranges) = highlighter.highlight_line(line, ps) {
            let line_spans = ranges
                .into_iter()
                .map(|(style, slice)| (style, slice.to_string()))
                .collect();
            result.push(line_spans);
        } else {
            result.push(vec![(Style::default(), line.to_string())]);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::syntax::detection::detect_syntax_by_extension;
    use crate::services::syntax::theme::get_theme;

    #[test]
    fn test_highlight_line() {
        let rust_syntax = detect_syntax_by_extension("rs").expect("rs syntax should exist");
        let theme = get_theme(true);
        let mut highlighter = create_highlighter(rust_syntax, theme);

        let res = highlight_line(&mut highlighter, "fn main() {\n");
        assert!(res.is_ok());
        let tokens = res.unwrap();
        assert!(!tokens.is_empty());
        // Verify tokens reconstruct the original line
        let joined: String = tokens.iter().map(|(_, s)| *s).collect();
        assert_eq!(joined, "fn main() {\n");
    }

    #[test]
    fn test_highlight_text() {
        let json_syntax = detect_syntax_by_extension("json").expect("json syntax should exist");
        let theme = get_theme(false);
        let code = "{\n  \"key\": \"value\"\n}\n";

        let lines = highlight_text(code, json_syntax, theme);
        assert_eq!(lines.len(), 3);
        let reconstructed: String = lines
            .into_iter()
            .flat_map(|l| l.into_iter().map(|(_, s)| s))
            .collect();
        assert_eq!(reconstructed, code);
    }
}
