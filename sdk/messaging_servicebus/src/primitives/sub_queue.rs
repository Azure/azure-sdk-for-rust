#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SubQueue {
    None = 0,
    DeadLetter = 1,
    TransferDeadLetter = 2,
}

impl Default for SubQueue {
    fn default() -> Self {
        Self::None
    }
}
