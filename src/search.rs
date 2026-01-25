use nucleo_matcher::{
    pattern::{AtomKind, CaseMatching, Normalization, Pattern},
    Matcher, Utf32Str,
};

/// Fuzzy matcher for tree items
pub struct FuzzyMatcher {
    matcher: Matcher,
}

impl FuzzyMatcher {
    pub fn new() -> Self {
        Self {
            matcher: Matcher::new(nucleo_matcher::Config::DEFAULT),
        }
    }

    /// Match a query against a haystack, returning score if matched
    /// Higher score = better match
    pub fn score(&mut self, query: &str, haystack: &str) -> Option<u32> {
        if query.is_empty() {
            return Some(0); // Empty query matches everything
        }

        let pattern = Pattern::new(query, CaseMatching::Ignore, Normalization::Smart, AtomKind::Fuzzy);

        let mut haystack_buf = Vec::new();
        let haystack_str = Utf32Str::new(haystack, &mut haystack_buf);

        pattern.score(haystack_str, &mut self.matcher)
    }

    /// Check if query matches haystack
    pub fn matches(&mut self, query: &str, haystack: &str) -> bool {
        self.score(query, haystack).is_some()
    }
}

impl Default for FuzzyMatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_query_matches_all() {
        let mut matcher = FuzzyMatcher::new();
        assert!(matcher.matches("", "anything"));
    }

    #[test]
    fn test_exact_match() {
        let mut matcher = FuzzyMatcher::new();
        assert!(matcher.matches("phase", "Phase 1"));
    }

    #[test]
    fn test_fuzzy_match() {
        let mut matcher = FuzzyMatcher::new();
        assert!(matcher.matches("ph1", "Phase 1"));
    }

    #[test]
    fn test_no_match() {
        let mut matcher = FuzzyMatcher::new();
        assert!(!matcher.matches("xyz", "Phase 1"));
    }

    #[test]
    fn test_case_insensitive() {
        let mut matcher = FuzzyMatcher::new();
        assert!(matcher.matches("PHASE", "phase"));
    }
}
