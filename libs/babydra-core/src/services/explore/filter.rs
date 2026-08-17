use crate::models::explore::file_entry::FileEntry;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use rayon::prelude::*;

/// Fuzzy filters and ranks file entries based on a search query using SkimMatcherV2 and Rayon.
pub fn filter_entries(entries: &[FileEntry], query: &str) -> Vec<FileEntry> {
    if query.is_empty() {
        return entries.to_vec();
    }

    let matcher = SkimMatcherV2::default();

    // Match and rank items using Rayon
    let mut scored_entries: Vec<(i64, FileEntry)> = entries
        .to_vec()
        .into_par_iter()
        .filter_map(|entry| {
            if let Some(score) = matcher.fuzzy_match(&entry.display_name, query) {
                Some((score, entry))
            } else {
                None
            }
        })
        .collect();

    // Sort by score descending (highest score first)
    scored_entries.sort_by(|a, b| b.0.cmp(&a.0));

    scored_entries.into_iter().map(|(_, e)| e).collect()
}
