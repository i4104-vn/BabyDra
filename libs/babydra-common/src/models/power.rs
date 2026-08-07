//! Performance profile data model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerformanceProfile {
    Normal,
    Balanced,
    HighPerformance,
}

impl PerformanceProfile {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Normal => "Normal",
            Self::Balanced => "Balanced",
            Self::HighPerformance => "High Performance",
        }
    }

    pub fn key(&self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Balanced => "balanced",
            Self::HighPerformance => "performance",
        }
    }

    pub fn from_key(key: &str) -> Self {
        match key {
            "normal" | "power-saver" => Self::Normal,
            "performance" | "high" => Self::HighPerformance,
            _ => Self::Balanced,
        }
    }
}
