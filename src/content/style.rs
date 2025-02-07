#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Style {
    Bold,
    Italic,
    Underline,
    Strike,
    Code,
    TextColor(String),
    BackgroundColor(String),
}

impl Style {
    pub fn name(&self) -> &str {
        match self {
            Style::Bold => "bold",
            Style::Italic => "italic",
            Style::Underline => "underline",
            Style::Strike => "strike",
            Style::Code => "code",
            Style::TextColor(_) => "textColor",
            Style::BackgroundColor(_) => "backgroundColor",
        }
    }
}

impl TryFrom<&str> for Style {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "bold" => Ok(Style::Bold),
            "italic" => Ok(Style::Italic),
            "underline" => Ok(Style::Underline),
            "strike" => Ok(Style::Strike),
            "code" => Ok(Style::Code),
            _ => Err(format!("Cannot create Style from {}", s)),
        }
    }
}
