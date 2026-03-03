use gap_buffer_implementation::GapBuffer;

fn dump(buf: &GapBuffer<char>) -> String {
    (0..buf.len())
        .map(|i| *buf.get(i).expect("valid index"))
        .collect()
}

fn check(buf: &GapBuffer<char>, expected: &str, position: usize) {
    let got = dump(buf);
    assert_eq!(got, expected, "content mismatch");
    assert_eq!(buf.position(), position, "cursor mismatch");
}

fn main() {
    let mut buf = GapBuffer::new();
    buf.insert_iter("Lord of the Rings".chars());
    check(&buf, "Lord of the Rings", 17);

    buf.set_position(12);
    buf.insert('r');
    check(&buf, "Lord of the rRings", 13);

    buf.set_position(12);
    let removed = buf.remove();
    assert_eq!(removed, Some('r'));
    check(&buf, "Lord of the Rings", 12);

    buf.set_position(buf.len());
    buf.insert('!');
    check(&buf, "Lord of the Rings!", buf.len());

    println!("OK: {}", dump(&buf))
}
