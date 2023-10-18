pub(crate) trait TokenInput {
    /// The line number where the bad input occurred.
    fn input(&self) -> &str;
}
