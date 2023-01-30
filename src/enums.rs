/// A collection of statements that instruct generation to continue/stop.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub enum GenerateInstruction {
    #[default] Continue,
    Stop,
}

/// A collection of results indicating whether to continue the search.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub enum SearchResult {
    #[default] Success,
    Pruned,
}
