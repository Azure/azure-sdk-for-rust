pub enum BlockType<'a> {
    Committed(&'a str),
    Uncommitted(&'a str),
    Latest(&'a str)
}
