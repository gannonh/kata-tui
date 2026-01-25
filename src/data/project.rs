/// Project metadata from PROJECT.md
#[derive(Debug, Clone, Default)]
pub struct Project {
    /// Project name (from first H1)
    pub name: String,
    /// Core value/description
    pub description: String,
    /// The problem being solved
    pub problem: String,
    /// The solution approach
    pub solution: String,
}
