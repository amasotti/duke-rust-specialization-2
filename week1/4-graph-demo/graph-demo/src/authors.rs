/// Author struct
///
/// # Example
///
/// ```rust
/// use graph_demo::authors::Author;
///
/// let author = Author::new("Dante");
/// assert_eq!(author.name, "Dante");
///
/// let author = Author::new("Homer");
/// assert_eq!(author.name, "Homer");
/// ```
#[derive(Debug)]
pub struct Author {
    pub name: String,
}

/// Author implementation
///
/// Just a simple implementation of the Author struct
impl Author {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
