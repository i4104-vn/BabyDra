/// Renders `markdown to pango`.
pub fn render_markdown_to_pango(markdown: &str) -> String {
    let options = pulldown_cmark::Options::empty();
    let parser = pulldown_cmark::Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    // Simple parser translating basic HTML tags to Pango markup tags
    html_output
        .replace("<h1>", "\n<b><span size=\"xx-large\">")
        .replace("</h1>", "</span></b>\n")
        .replace("<h2>", "\n<b><span size=\"x-large\">")
        .replace("</h2>", "</span></b>\n")
        .replace("<h3>", "\n<b><span size=\"large\">")
        .replace("</h3>", "</span></b>\n")
        .replace("<p>", "")
        .replace("</p>", "\n")
        .replace("<strong>", "<b>")
        .replace("</strong>", "</b>")
        .replace("<em>", "<b>")
        .replace("</em>", "</b>")
        .replace(
            "<pre><code>",
            "\n<span face=\"monospace\" background=\"#2e2e2e\">",
        )
        .replace("</code></pre>", "</span>\n")
        .replace(
            "<code>",
            "<span face=\"monospace\" background=\"#2e2e2e\"> ",
        )
        .replace("</code>", " </span>")
        .replace("<ul>", "")
        .replace("</ul>", "")
        .replace("<li>", " • ")
        .replace("</li>", "\n")
}
