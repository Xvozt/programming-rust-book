// here i cant use 1 lifetime, borrow checker will complain on line println!("{}", s)
struct MutStr<'a, 'b> {
    s: &'a mut &'b str,
}

#[repr(transparent)]
pub struct NewString(String);

fn main() {
    let mut s = "Hello";
    *MutStr { s: &mut s }.s = "world";
    println!("{}", s)
}
