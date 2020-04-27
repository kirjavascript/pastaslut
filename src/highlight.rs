use syntect::parsing::{SyntaxSet};
use syntect::highlighting::{Color, ThemeSet};
use syntect::html::highlighted_html_for_string;

pub fn highlight(content: &str, filename: &str) -> String {
    let syntax = SyntaxSet::load_defaults_newlines();
    let theme = ThemeSet::load_defaults();

    let theme = &theme.themes["base16-ocean.dark"];
    let extension = crate::file::extension(filename).unwrap_or_else(|| "txt".to_string());
    let reference = syntax.find_syntax_by_extension(&extension)
            .unwrap_or_else(|| syntax.find_syntax_by_extension("txt").unwrap());

    let color = theme.settings.background.unwrap_or(Color::WHITE);
    let html = highlighted_html_for_string(content, &syntax, &reference, theme);

    format!(r#"
            <html>
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width" />
                    <title>{}</title>
                </head>
                <style>
                    pre {{
                        font-size: 13px;
                        font-family: Consolas, "Liberation Mono", Menlo, Courier, monospace;
                    }}
                </style>
                <body style="background-color:#{:02x}{:02x}{:02x};">
                {}
                </body>
            </html>
        "#, title(content), color.r, color.g, color.b, html)
}

fn title(content: &str) -> String {
    let max_length = 200;
    let title = content.replace("<", "&lt;").replace("&", "&amp;");
    let mut title = title[..title.len().min(max_length)].to_string();
    if title.len() == max_length {
        title.push_str("...");
    }
    title
}
