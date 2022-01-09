pub trait ImmutableStringReader {
    fn string(&self) -> &str;
    fn remaining_length(&self) -> usize;
    fn total_length(&self) -> usize;
    fn cursor(&self) -> usize;
    fn get_read(&self) -> &str;
    fn remaining(&self) -> &str;
    fn can_read_length(&self, length: usize) -> bool;
    fn can_read(&self) -> bool;
    fn peek(&self) -> char;
    fn peek_offset(&self, offset: usize) -> char;
}
