#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Style {
    Bold,
    Italic,
    Underline,
    Strike,
}

impl Style {
    pub fn name(&self) -> &str {
        match self {
            Style::Bold => "bold",
            Style::Italic => "italic",
            Style::Underline => "underline",
            Style::Strike => "strike",
        }
    }
}

impl From<&str> for Style {
    fn from(s: &str) -> Self {
        match s {
            "bold" => Style::Bold,
            "italic" => Style::Italic,
            "underline" => Style::Underline,
            "strike" => Style::Strike,
            _ => panic!("Invalid style: {}", s),
        }
    }
}
