use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Settings {
    title_style: StyleConfig,
    normal_style: StyleConfig,
    highlighted_style: StyleConfig,
    error_style: StyleConfig,
}

impl Settings {
    fn load(filename: &PathBuf) -> Result<Settings, io::Error> {
        let contents = fs::read_to_string(filename)?;
        let settings: Settings = toml::from_str(&contents)?;
        Ok(settings)
    }


}

#[derive(Serialize, Deserialize, Debug)]
struct StyleConfig {
    fg: String,
    bg: Option<String>,
    modifiers: Vec<String>,
}


impl From<StyleConfig> for Style {
    fn from(styleConfig: StyleConfig) -> Style {
        let fg = styleConfig.fg.parse().unwrap_or(Color::White);
        let bg = styleConfig.bg.as_ref().map_or(Color::Black, |c| c.parse().unwrap_or(Color::Black));
        let mut style = Style::default().fg(fg).bg(bg);

        for modifier in &styleConfig.modifiers {
            match modifier.as_str() {
                "bold" => style = style.add_modifier(Modifier::BOLD),
                "italic" => style = style.add_modifier(Modifier::ITALIC),
                "underlined" => style = style.add_modifier(Modifier::UNDERLINED),
                "reversed" => style = style.add_modifier(Modifier::REVERSED),
                _ => {}
            }
        }

        style
    }
}
