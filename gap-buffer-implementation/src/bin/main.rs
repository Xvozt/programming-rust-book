use gap_buffer_implementation::GapBuffer;

fn main() {
    let mut buf = GapBuffer::new();
    buf.insert_iter("Lord of the Rings".chars());
    buf.set_position(12);
    buf.insert('r');
}
