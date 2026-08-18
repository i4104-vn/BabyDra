//! Navigation categories and settings page metadata models.

/// Navigation item in the settings sidebar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NavItem {
    pub id: &'static str,
    pub icon: &'static str,
    pub i18n_key: &'static str,
}

/// Navigation category grouping items together under a section header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NavCategory {
    pub title_key: &'static str,
    pub items: &'static [NavItem],
}
